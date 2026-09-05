use tonic::body::Body;
use tonic::codegen::http::Request;
use tonic::{Status, async_trait};
use tonic_middleware::RequestInterceptor;

use crate::config::AuthConfig;

#[derive(Clone, Debug)]
pub struct ClientIdentity(pub String);

#[derive(Clone)]
pub struct AuthInterceptor {
    pub auth_config: AuthConfig,
}

#[async_trait]
impl RequestInterceptor for AuthInterceptor {
    async fn intercept(&self, mut req: Request<Body>) -> Result<Request<Body>, Status> {
        if !self.auth_config.enabled {
            req.extensions_mut()
                .insert(ClientIdentity("anonymous-loopback".to_string()));
            Ok(req)
        } else {
            match req.headers().get("X-API-Key").map(|v| v.to_str()) {
                Some(Ok(token)) => {
                    let mut client: Option<&String> = None;
                    for key in &self.auth_config.tokens {
                        if key.token == token {
                            client = Some(&key.client);
                            break;
                        }
                    }

                    match client {
                        Some(client_name) => {
                            log::debug!("Authenticated client: {}", client_name);
                            req.extensions_mut()
                                .insert(ClientIdentity(client_name.to_string()));
                            Ok(req)
                        }
                        _ => Err(Status::unauthenticated("Unauthenticated")),
                    }
                }
                _ => Err(Status::unauthenticated("Unauthenticated")),
            }
        }
    }
}
