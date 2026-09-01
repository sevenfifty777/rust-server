use std::convert::TryFrom;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use tokio::sync::Mutex;
use tokio::time::MissedTickBehavior;

use crate::shutdown::ShutdownHandle;

#[derive(Clone)]
pub struct Stats(Arc<Inner>);

struct Inner {
    shutdown_signal: ShutdownHandle,
    /// Total numer of calls into the MSE.
    calls_count: AtomicU32,
    /// Total numer of events received from the MSE.
    events_count: AtomicU32,
    /// Total numer of calls in the queue.
    queue_size: AtomicU32,
    /// Time spent waiting for MSE calls to complete (since last report).
    nanoseconds_waited: AtomicUsize,
    /// IPC requests completed since the last report.
    ipc_requests_completed: AtomicU64,
    /// IPC requests that returned a Lua-side error since the last report.
    ipc_requests_failed: AtomicU64,
    /// Cancelled requests removed before entering Lua since the last report.
    ipc_requests_cancelled: AtomicU64,
    /// Total time IPC requests spent waiting to enter Lua since the last report.
    ipc_queue_wait_nanoseconds: AtomicU64,
    /// Total time IPC callbacks spent executing in Lua since the last report.
    ipc_execution_nanoseconds: AtomicU64,
    /// Largest queue depth observed when a request was enqueued.
    ipc_queue_depth_highest: AtomicUsize,
    /// Stats collected during an interval necessary to create a report at the end of the interval.
    interval_stats: Arc<Mutex<IntervalStats>>,
}

#[derive(Default)]
struct IntervalStats {
    /// Highest TPS count of calls into the MSE.
    tps_highest: f64,
    /// Highest events per second.
    eps_highest: f64,
    /// Sum of the queue sizes at each tick (neccessary to calculate the average).
    queue_size_total: u32,
    /// Highest queue size at a tick of the interval.
    queue_size_highest: u32,
}

/// This guard is used to keep track of the time the gRPC server blocked DCS.
pub struct TrackBlockTimeGuard {
    start: Instant,
    stats: Arc<Inner>,
}

/// This guard is used to keep track of calls in the queue.
pub struct TrackQueueSizeGuard {
    stats: Arc<Inner>,
}

pub struct IpcRequestMeasurement<'a> {
    pub request_id: u64,
    pub method: &'a str,
    pub queue_wait: Duration,
    pub execution_time: Duration,
    pub queue_depth_at_enqueue: usize,
    pub queue_depth_at_dequeue: usize,
    pub outcome: &'a str,
}

impl Stats {
    pub fn new(shutdown_signal: ShutdownHandle) -> Self {
        Stats(Arc::new(Inner {
            shutdown_signal,
            calls_count: AtomicU32::new(0),
            events_count: AtomicU32::new(0),
            queue_size: AtomicU32::new(0),
            nanoseconds_waited: AtomicUsize::new(0),
            ipc_requests_completed: AtomicU64::new(0),
            ipc_requests_failed: AtomicU64::new(0),
            ipc_requests_cancelled: AtomicU64::new(0),
            ipc_queue_wait_nanoseconds: AtomicU64::new(0),
            ipc_execution_nanoseconds: AtomicU64::new(0),
            ipc_queue_depth_highest: AtomicUsize::new(0),
            interval_stats: Arc::new(Mutex::new(IntervalStats::default())),
        }))
    }

    pub fn track_call(&self) {
        self.0.calls_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn track_event(&self) {
        self.0.events_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn track_block_time(&self, start: Instant) -> TrackBlockTimeGuard {
        self.0.calls_count.fetch_add(1, Ordering::Relaxed);
        TrackBlockTimeGuard {
            start,
            stats: self.0.clone(),
        }
    }

    pub fn track_queue_size(&self) -> TrackQueueSizeGuard {
        self.0.queue_size.fetch_add(1, Ordering::Relaxed);
        TrackQueueSizeGuard {
            stats: self.0.clone(),
        }
    }

    pub fn track_ipc_request(&self, measurement: IpcRequestMeasurement<'_>) {
        if measurement.outcome == "success" {
            self.0
                .ipc_requests_completed
                .fetch_add(1, Ordering::Relaxed);
        } else {
            self.0.ipc_requests_failed.fetch_add(1, Ordering::Relaxed);
        }
        self.0.ipc_queue_wait_nanoseconds.fetch_add(
            u64::try_from(measurement.queue_wait.as_nanos()).unwrap_or(u64::MAX),
            Ordering::Relaxed,
        );
        self.0.ipc_execution_nanoseconds.fetch_add(
            u64::try_from(measurement.execution_time.as_nanos()).unwrap_or(u64::MAX),
            Ordering::Relaxed,
        );
        self.0
            .ipc_queue_depth_highest
            .fetch_max(measurement.queue_depth_at_enqueue, Ordering::Relaxed);

        log::debug!(
            "IPC request completed: id={} method={} queue_wait_ms={:.3} execution_ms={:.3} queue_depth_at_enqueue={} queue_depth_at_dequeue={} outcome={}",
            measurement.request_id,
            measurement.method,
            measurement.queue_wait.as_secs_f64() * 1_000.0,
            measurement.execution_time.as_secs_f64() * 1_000.0,
            measurement.queue_depth_at_enqueue,
            measurement.queue_depth_at_dequeue,
            measurement.outcome
        );
    }

    pub fn track_cancelled_ipc_requests(&self, count: u64) {
        if count > 0 {
            self.0
                .ipc_requests_cancelled
                .fetch_add(count, Ordering::Relaxed);
            log::debug!("Discarded {} cancelled IPC request(s)", count);
        }
    }

    pub async fn run_in_background(self) {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        let mut last_logged = Instant::now();
        let log_interval = Duration::from_secs(60);
        let mut shutdown_signal = self.0.shutdown_signal.signal();

        loop {
            let calls_count_before = self.0.calls_count.load(Ordering::Relaxed);
            let events_count_before = self.0.events_count.load(Ordering::Relaxed);
            let start = Instant::now();

            // wait for either the shutdown signal or the next interval tick, whatever happens first
            tokio::select! {
                _ = &mut shutdown_signal => {
                    break
                }
                _ = interval.tick() => {}
            };

            let mut interval_stats = self.0.interval_stats.lock().await;
            let calls_count = self.0.calls_count.load(Ordering::Relaxed);
            let events_count = self.0.events_count.load(Ordering::Relaxed);

            // update report for elapsed second
            let elapsed = start.elapsed().as_secs_f64();
            if elapsed > 0.0 {
                // update highest TPS
                let tps = f64::from(calls_count - calls_count_before) / elapsed;
                if tps > interval_stats.tps_highest {
                    interval_stats.tps_highest = tps;
                }

                // update highest events per second
                let eps = f64::from(events_count - events_count_before) / elapsed;
                if eps > interval_stats.eps_highest {
                    interval_stats.eps_highest = eps;
                }

                // update queue size
                let queue_size = self.0.queue_size.load(Ordering::Relaxed);
                interval_stats.queue_size_total += queue_size;
                if queue_size > interval_stats.queue_size_highest {
                    interval_stats.queue_size_highest = queue_size;
                }
            }

            // log summary every minute
            let elapsed = last_logged.elapsed();
            if elapsed > log_interval {
                // average TPS
                let tps_average = f64::from(calls_count) / elapsed.as_secs_f64();

                // average events per second
                let eps_average = f64::from(events_count) / elapsed.as_secs_f64();

                // total block time
                let block_time_total = Duration::from_nanos(
                    u64::try_from(self.0.nanoseconds_waited.swap(0, Ordering::Relaxed))
                        .unwrap_or(u64::MAX),
                );
                let block_time_total_percentage =
                    (block_time_total.as_secs_f64() / elapsed.as_secs_f64()) * 100.0;

                // average queue size
                let queue_size_average =
                    f64::from(interval_stats.queue_size_total) / elapsed.as_secs_f64();

                // format and log stats
                log::info!(
                    "Calls per second: average={:.2}, highest={:.2}",
                    tps_average,
                    interval_stats.tps_highest
                );
                log::info!(
                    "Events per second: average={:.2}, highest={:.2}",
                    eps_average,
                    interval_stats.eps_highest
                );
                log::info!(
                    "Blocking time: total={:?} (≙ {:.2}%)",
                    block_time_total,
                    block_time_total_percentage
                );
                log::info!(
                    "Queue size: average={:.2}, biggest={:.2}",
                    queue_size_average,
                    interval_stats.queue_size_highest
                );
                let completed = self.0.ipc_requests_completed.swap(0, Ordering::Relaxed);
                let failed = self.0.ipc_requests_failed.swap(0, Ordering::Relaxed);
                let cancelled = self.0.ipc_requests_cancelled.swap(0, Ordering::Relaxed);
                let measured = completed + failed;
                let queue_wait = Duration::from_nanos(
                    self.0.ipc_queue_wait_nanoseconds.swap(0, Ordering::Relaxed),
                );
                let execution = Duration::from_nanos(
                    self.0.ipc_execution_nanoseconds.swap(0, Ordering::Relaxed),
                );
                let queue_depth_highest = self.0.ipc_queue_depth_highest.swap(0, Ordering::Relaxed);
                let divisor = measured.max(1) as f64;
                log::info!(
                    "IPC requests: completed={}, failed={}, cancelled={}, average_queue_wait_ms={:.3}, average_execution_ms={:.3}, highest_enqueue_depth={}",
                    completed,
                    failed,
                    cancelled,
                    queue_wait.as_secs_f64() * 1_000.0 / divisor,
                    execution.as_secs_f64() * 1_000.0 / divisor,
                    queue_depth_highest
                );

                // reset data for next interval
                last_logged = Instant::now();
                *interval_stats = IntervalStats::default();
                self.0.calls_count.store(0, Ordering::Relaxed);
                self.0.nanoseconds_waited.store(0, Ordering::Relaxed);
            }
        }
    }
}

impl Drop for TrackBlockTimeGuard {
    fn drop(&mut self) {
        self.stats.nanoseconds_waited.fetch_add(
            usize::try_from(self.start.elapsed().as_nanos()).unwrap_or(usize::MAX),
            Ordering::Relaxed,
        );
    }
}

impl Drop for TrackQueueSizeGuard {
    fn drop(&mut self) {
        self.stats.queue_size.fetch_sub(1, Ordering::Relaxed);
    }
}
