# Contexte de discussion Codex — évolution DCS-gRPC pour LSO

> Document machine-first dédié aux futures discussions et modifications du fork DCS-gRPC nécessaires à l’acquisition bufferisée des positions avion/porte-avions. Synthèse établie le 2 septembre 2026 depuis `.agents/agents.md`, `.ignore/prompt.md`, `.ignore/tasking-v3.md` et la documentation Markdown du dépôt. Ce document décrit une cible et des contrats à implémenter ; il ne constitue pas une preuve de fonctionnement dans DCS réel.

## Objet de ce contexte

Le projet LSO est un client Rust/Tokio externe à DCS World. Il dépend aujourd’hui de deux appels unary `UnitService/GetTransform`, lancés concurremment, pour obtenir la position de l’avion et celle du porte-avions. Cette acquisition perd les positions intermédiaires lorsqu’une réponse DCS-gRPC arrive tardivement.

La future modification doit être réalisée dans le fork DCS-gRPC et fournir à LSO des snapshots bruts avion/carrier capturés côté mission Lua, au même tick source, conservés dans un tampon circulaire et récupérables en lots incrémentaux. Rust reste l’autorité pour l’alignement métier, les gates, la corrélation, la notation, les rapports et la persistance.

Ce fichier sert à ouvrir une discussion Codex centrée sur **DCS-gRPC**. Pour le contexte intégral du client LSO, lire `.agents/agents.md`.

## Règles de vérité et précautions

Ordre des sources : résultat fraîchement exécuté > code courant du dépôt concerné > artefact live authentifié > contrat ou décision explicite > cible de conception > documentation historique ou hypothèse.

- Ne jamais annoncer une validation DCS live, une amélioration de p99, un impact FPS acceptable ou une compatibilité fonctionnelle sans nouvelle preuve.
- La cause exacte des pauses n’est pas prouvée. Les suspects restent le scheduler DCS, le Lua DCS-gRPC, la file/DLL, la sérialisation, la concurrence des requêtes et le comportement client.
- Le snapshot `docs/DCS-gRPC-0.9.0/` est une copie fournisseur de référence, pas le dépôt source complet du serveur ni une preuve du déploiement réel.
- Préserver les modifications utilisateur et les contrats protobuf existants. Favoriser une évolution additive.
- Ne pas relever le seuil LSO de 300 ms, interpoler une coupure proche de 900 ms ou fabriquer des positions.
- Ne pas utiliser l’ACMI produit par LSO comme source indépendante : il dérive des mêmes positions live.
- Ne pas déplacer les règles de gates ou de grading dans Lua.
- Ne pas intégrer le hook/crosse au snapshot positionnel qualifiant. Ses erreurs doivent rester indépendantes.
- Ne pas confondre timestamp de capture DCS, timestamp de livraison et timestamp de réception Rust.
- Un stream sans rétention source ne résout pas le problème. La propriété essentielle est la capture avant transport, avec séquences et overflow observable.

## État des dépôts et dépendances connu

### Server DCS
- Chemin installation : E:\DCS World Server\DCS World Server\bin\DCS_server.exe
- Chemin Saved Games : C:\Users\Olivier\Saved Games\DCS.dcs_serverrelease
- IP : 127.0.0.1
- Port : 10308

### Client LSO courant

- Dépôt : `E:\DCS stuffs\Initiative ESG\DCS-gRPC-lso`.
- Branche : `feature/refonte-v3-lua-buffer`.
- HEAD de base : `f962498109b78eac40c16af14962a888024f17fe`.
- Crate `lso` 0.2.0, Rust 2021.
- Worktree volontairement dirty : la refonte v3 Rust est non commitée.
- `tonic = 0.13`; résolution actuelle `tonic 0.13.1`.
- Stubs : `sevenfifty777/rust-server`, tag `v0.9.0`, commit verrouillé `5bd6d6e42491c8697a5c5a95e80a2e689923bd3b`.
- Serveur attendu : fork officiel 0.9.0. Un serveur 0.9.1 a été observé dans un corpus, mais sa compatibilité fonctionnelle n’est pas validée.

Dernière validation locale consignée dans `.agents/agents.md` :

- `cargo fmt --all -- --check` réussi ;
- `cargo test --locked --no-fail-fast` : 125 tests réussis ;
- `cargo clippy --locked --all-targets -- -D warnings` réussi ;
- `git diff --check` réussi ;
- aucune validation DCS live de la source bufferisée, puisqu’elle n’existe pas encore.

### Ce qui est déjà préparé dans LSO

- `src/tasks/position_collector.rs` isole les deux transforms prioritaires, leur alignement et leurs métriques.
- L’implémentation actuelle reste `paired_unary_polling_v1` et utilise deux `GetTransform` concurrents.
- La frontière `PositionCollector` est destinée à recevoir une seconde implémentation alimentée par le batch source.
- `MissedTickBehavior::Skip`, `--positions-only`, la suspension des détecteurs et les métriques par recovery sont déjà présents.
- Les sorties, le hook et les événements sont séparés du collecteur prioritaire.
- La complétude positionnelle ne peut être affectée que par la perte réelle des positions ; les overflows hook/event sont seulement diagnostiques.

Ne pas refaire cette refonte Rust dans le fork DCS-gRPC. Le travail du fork est d’ajouter une source fiable et observable, puis le client LSO devra la consommer derrière sa frontière existante.

## Problème mesuré à résoudre

Corpus live historique du 31 août 2026, F-14B(U) sur CVN-72 :

- 895 positions en 180,81 s, environ 4,95 Hz au lieu de 10 Hz ;
- gap moyen 202 ms ; p50 98 ms ; p90 756 ms ; p95 933 ms ; p99 960 ms ; maximum 971 ms ;
- environ 20 % des samples dépassent 300 ms ; 83 dépassent 900 ms ;
- latence moyenne d’environ 129 ms par transform, avec des pointes proches de 877 ms ;
- skew avion/carrier de ce run acceptable, maximum 30 ms ;
- brackets réels des gates : 960, 930 et 360 ms, donc invalidation correcte au seuil de 300 ms ;
- LQM DCS : grade C, câble 1 ; grade Rust : `NC`, aucun point, car les preuves de gates sont insuffisantes.

Interprétation correcte : le volume de données ne sature pas le collecteur. LSO demande une paire, attend, puis recommence. Une attente de 900 ms signifie qu’aucune position intermédiaire n’est capturée par le client. Le réseau localhost n’est probablement pas la cause principale, mais cela reste une inférence, pas une preuve.

Le buffer source doit changer cette propriété : si la livraison prend 900 ms alors que DCS/Lua continue à s’exécuter, le lot suivant doit contenir les snapshots capturés durant ces 900 ms. Si DCS lui-même ne produit aucun tick pendant le gel, le buffer ne peut évidemment rien inventer ; ce trou doit être détectable.

## Comportement DCS-gRPC 0.9.0 utile à la conception

Dans la copie fournisseur :

- `grpc-mission.lua` charge la configuration, la DLL et `grpc.lua`; avec `GRPC.autostart = true`, DCS-gRPC démarre automatiquement pour la mission.
- `grpc.lua` charge les exporters et tous les modules `methods/*.lua`.
- Les requêtes mission sont dépilées via `grpc.next(MISSION_ENV, handleRequest)`.
- Le scheduler mission utilise `timer.scheduleFunction` avec un intervalle dérivé de `GRPC.throughputLimit` : `max(0.03, min(1.0, 16 / throughputLimit))`.
- La limite par défaut est 600 appels/s, soit un intervalle minimal de 30 ms et plusieurs appels possibles par tick de dépilage.
- `methods/unit.lua#getUnitTransform` recherche l’unité par nom, lit `timer.getTime()`, puis exporte `getPosition()` et `getVelocity()` via `GRPC.exporters.rawTransform`.
- `GetTransformResponse` contient déjà temps de scénario, position, orientation et vitesse, mais chaque unité est lue par une requête distincte.

Conséquence de conception : ne pas réaliser la capture périodique en envoyant à Lua une requête par unité. Le module source doit avoir son **propre timer de capture**, séparé du timer qui dépile les RPC. Le RPC de lecture ne fait que copier un lot déjà capturé.

## Architecture cible décidée

```text
DCS / environnement mission Lua
  -> module chargé une fois avec DCS-gRPC
  -> inactif lorsqu’aucune recovery n’est enregistrée
  -> timer unique pour toutes les recoveries actives
  -> lecture avion + carrier dans le même callback/tick
  -> déduplication de la lecture carrier par tick
  -> snapshots bruts horodatés et séquencés
  -> ring buffer borné par recovery ou contrat équivalent

DCS-gRPC / DLL + protobuf
  -> RPC de démarrage/enregistrement d’une paire
  -> batch incrémental après une séquence connue
  -> RPC d’arrêt/retrait, plus expiration automatique
  -> métadonnées explicites de plage et d’overflow

LSO / Rust
  -> supervise le cycle de vie de la paire
  -> récupère les lots dans l’ordre
  -> détecte trous, doublons, resets et overflow
  -> convertit les snapshots vers la télémétrie commune
  -> calcule gates, outcomes, grade et rapports
```

Choix provisoire acté : commencer par un **batch incrémental avec `after_sequence`**, plus simple à reprendre, tester et diagnostiquer qu’un stream initial. Un futur stream pourra réutiliser exactement le même modèle de snapshots, séquences et overflow.

## Cycle de vie recommandé

1. Au chargement de mission, le module Lua est présent mais ne collecte rien.
2. LSO détecte une recovery et enregistre le couple avion–carrier avec une identité opaque de suivi.
3. Le collecteur Lua unique parcourt les couples actifs à chaque tick de capture.
4. Il lit chaque carrier actif une seule fois pour ce tick, puis chaque avion, et construit pour chaque recovery une paire issue du même callback.
5. Les snapshots sont ajoutés au ring source.
6. LSO appelle périodiquement le batch avec `after_sequence = dernière_séquence_traitée`.
7. À la fin/annulation de recovery, LSO retire le couple.
8. Un TTL source retire aussi les enregistrements orphelins si le client disparaît sans arrêt propre.
9. À une nouvelle mission, un reconnect ou un reset du module, l’identité d’epoch/session change ; Rust ne doit jamais joindre des séquences de deux epochs.

Un timer par avion est déconseillé. Un timer unique réduit la charge, assure une notion de tick commun et permet de partager proprement la lecture d’un carrier entre plusieurs recoveries.

## Paramètres cibles déjà décidés

- Cadence visée : 20 Hz, soit 50 ms, uniquement pendant au moins une recovery active.
- Plancher d’acceptation : 10 Hz si les mesures montrent que 20 Hz coûte trop cher ou n’apporte pas de fiabilité supplémentaire.
- Aucun balayage permanent de toutes les unités.
- Rétention cible initiale : 30 secondes.
- Lot borné : au maximum 100 snapshots par réponse.
- Séquence : monotone et exploitable par `after_sequence`.
- Mémoire : fixe/bornée, jamais une liste croissant avec la durée de mission.
- Hook/crosse : hors du snapshot groupé.
- Seuil métier LSO : inchangé, 300 ms.

Ces nombres sont une base de développement, pas encore un dimensionnement certifié. Mesurer leur impact sur le tick/FPS DCS, le temps Lua, la taille sérialisée et la mémoire.

## Contrat minimal d’un snapshot

Champs nécessaires, avec types protobuf exacts à arrêter dans le fork :

- identifiant/handle de recovery ;
- numéro de séquence monotone ;
- epoch de source ou identifiant de session du buffer ;
- timestamp DCS commun du callback de capture ;
- identité avion : au minimum ID DCS et nom attendu ;
- identité carrier : au minimum ID DCS et nom attendu ;
- transform brut avion : position, orientation et vitesse ;
- transform brut carrier : position, orientation et vitesse ;
- état de validité de chaque unité si elle a disparu ou si une lecture a échoué ;
- éventuellement durée/coût de capture pour l’observabilité, sans en faire une donnée métier.

Le timestamp commun doit être lu une fois au début ou à un point défini du callback. L’ordre de lecture avion/carrier existe toujours dans Lua ; il faut donc documenter que « même tick » signifie même callback et timestamp commun, pas simultanéité physique parfaite. Si le coût de boucle devient mesurable, exposer ce coût ou les offsets de lecture comme diagnostic plutôt que prétendre à un skew nul.

Réutiliser autant que possible les messages `Position`, `Orientation` et `Velocity` existants afin d’éviter deux représentations incompatibles. Préserver les numéros de champs protobuf existants et n’ajouter que de nouveaux messages/RPC.

## Contrat minimal du batch

La requête de lecture doit exprimer au minimum :

- handle de recovery ;
- epoch/session source attendu si connu ;
- `after_sequence` exclusif ;
- limite maximale souhaitée, bornée aussi côté serveur.

La réponse doit permettre de distinguer sans ambiguïté :

- aucun nouveau snapshot ;
- snapshots contigus disponibles ;
- `after_sequence` trop ancien parce que les données ont été écrasées ;
- epoch inconnu/réinitialisé ;
- recovery inconnue, expirée ou arrêtée ;
- avion ou carrier disparu ;
- erreur interne de capture.

Métadonnées recommandées :

- `source_epoch` ;
- `oldest_available_sequence` ;
- `newest_available_sequence` ;
- `next_after_sequence` ou dernière séquence effectivement renvoyée ;
- `overflow_count` cumulatif pour cette recovery/epoch ;
- booléen ou enum signalant que la requête a subi une perte avant le premier élément renvoyé ;
- cadence configurée/effective et capacité/rétention annoncées ;
- liste ordonnée de snapshots, limitée à 100 par défaut/cible.

Sémantique recommandée : `after_sequence = N` retourne les éléments de séquence strictement supérieure à N. Une répétition de la même requête peut renvoyer les mêmes éléments ; le client déduplique par `(source_epoch, recovery_handle, sequence)`. Le contrat doit être idempotent et permettre une reprise après timeout sans ACK destructif.

Ne pas supprimer les éléments lors de leur lecture. Ils expirent uniquement selon le ring/TTL. Un protocole « read-and-pop » rendrait les timeouts et retries dangereux.

## Séquences, overflow et resets

- La séquence doit augmenter à chaque tentative de snapshot qualifiant ou à chaque tick de capture ; choisir puis documenter la sémantique exacte.
- Recommandation : consommer une séquence par tick/recovery même si une unité est absente, avec un statut invalide explicite. Cela différencie une capture invalide d’une capture jamais exécutée.
- Ne jamais réutiliser silencieusement une séquence dans la même epoch.
- Lors d’un restart/reload Lua, créer une nouvelle epoch au lieu de tenter de prolonger l’ancienne séquence.
- Si le producteur dépasse la capacité, écraser le plus ancien élément, incrémenter `overflow_count` et avancer `oldest_available_sequence`.
- Si `after_sequence + 1 < oldest_available_sequence`, la réponse doit annoncer une perte de source. Rust transformera cette perte en diagnostic/completeness approprié ; Lua ne décide pas du grade.
- Des séquences absentes dans la plage annoncée, dupliquées avec un contenu différent ou non monotones sont des erreurs de contrat et doivent être journalisées/testées.

Le mot `BufferLimit` dans LSO doit rester réservé à la perte de positions. Un overflow de ce ring est donc matériel et bloquant pour la zone concernée, contrairement aux anciens overflows hook/event diagnostiques.

## Identité, multi-recovery et partage carrier

LSO isole déjà les tâches par session, génération, ID avion et ID carrier. Le fork ne doit pas utiliser le seul nom d’unité comme identité durable : un respawn peut conserver le nom avec un nouvel ID.

Le contrat de démarrage doit éviter les collisions et les suppressions croisées. Recommandation : LSO fournit un handle opaque unique par recovery, tandis que Lua vérifie et capture aussi les IDs/noms DCS réels.

Pour plusieurs avions sur le même carrier :

- lire le carrier une fois par tick source ;
- réutiliser exactement cette lecture dans chaque snapshot concerné ;
- ne pas exposer un cache carrier périmé comme preuve de gate ;
- la clé conceptuelle du partage qualifiant est `(source_epoch, carrier_id, tick/sequence_source)` ;
- un cache distinct et moins strict reste acceptable pour la simple détection non qualifiante côté LSO.

Décisions à prendre explicitement dans le fork :

- séquence globale d’epoch, par recovery, ou tick global + index ;
- politique lorsqu’une recovery est enregistrée deux fois ;
- comportement si les noms pointent vers de nouveaux IDs ;
- TTL et keepalive ;
- limite du nombre de recoveries/carriers actifs ;
- droits/authentification requis pour start/stop/read.

## Placement du code dans DCS-gRPC

Direction recommandée d’après la copie 0.9.0 :

- ajouter un module Lua dédié, par exemple `methods/recovery_telemetry.lua` plus un module d’état/capture si nécessaire ;
- le charger depuis `grpc.lua` avec les autres méthodes ;
- initialiser son timer au chargement, mais ne faire aucun travail de capture sans paire active ;
- réutiliser `GRPC.exporters.rawTransform` ou extraire un exporter stable commun ;
- enregistrer start/read/stop dans `GRPC.methods` selon les conventions du fork ;
- ajouter un service protobuf dédié plutôt que surcharger sémantiquement `GetTransform` ;
- implémenter le routage/handler Rust de la DLL et régénérer les stubs ;
- mettre à jour packaging, changelog, version, documentation API et tests du fork.

La copie incluse ne contient pas tout le source Rust du serveur. Identifier les fichiers réels de dispatch protobuf/DLL dans le clone du fork avant de rédiger un plan de patch définitif.

### Chargement et déploiement recommandés

Intégrer le module au chargement normal DCS-gRPC est l’option privilégiée : installation unique par l’administrateur, aucune modification des missions `.miz`, aucune action du Mission Maker.

Solutions de repli, par ordre décroissant :

1. injection par le hook à `onMissionLoadEnd` ; transparente mais plus fragile entre environnements hook/mission ;
2. `missionEval` pour un prototype uniquement ; désactivé par défaut et risqué côté sécurité/timing ;
3. script/trigger dans le `.miz` ; fiable mais impose une modification de mission ;
4. modification automatique du `.miz` ; fragile et difficile à tracer.

Ne pas choisir `missionEval` comme architecture de production sans décision explicite de sécurité.

## Frontière métier Lua / Rust

Lua/DCS-gRPC peut :

- enregistrer et retirer une paire ;
- lire les objets DCS ;
- capturer positions, orientations, vitesses et états bruts ;
- timestamp/sequence/bufferiser ;
- partager une lecture carrier au même tick ;
- déclarer absence, reset, overflow et erreurs techniques.

Lua/DCS-gRPC ne doit pas :

- détecter ou valider les gates 3/4, 1/2 et 1/4 NM ;
- interpoler pour le grading ;
- choisir une note ou des points ;
- décider d’un bolter, waveoff, arrest ou câble ;
- corréler LQM/événements métier ;
- lisser silencieusement la trajectoire ;
- transformer une perte technique en faute pilote.

Une variante légère où Lua ne conserve que les samples avant/après des franchissements existe dans les notes, mais elle est secondaire : elle rapproche Lua de la logique de gate et complique la généralité. La cible principale reste un flux court de snapshots bruts.

## Intégration attendue côté LSO après évolution du fork

Travail ultérieur dans `DCS-gRPC-lso` :

- mettre à jour le pin des stubs vers une release/commit du fork revu et authentifié ;
- garder `tonic` aligné sur la version requise par les stubs ;
- ajouter un client de télémétrie recovery ;
- ajouter une implémentation bufferisée derrière la frontière `PositionCollector` ;
- démarrer/arrêter la capture selon le cycle de recovery et la génération du superviseur ;
- lire les lots avec une seule demande contrôlée, timeout et reprise idempotente ;
- convertir chaque snapshot vers le type de télémétrie commun sans perdre le timestamp DCS, la séquence, l’epoch ni les diagnostics ;
- détecter doublons, trous, overflow, time reversal et reset ;
- réinitialiser l’aligneur/consommateur lors d’une coupure ou d’une nouvelle epoch ;
- conserver temporairement le polling unary comme contrôle/rollback mesurable ;
- exposer une nouvelle valeur `acquisition_source`, sans renommer les champs JSON v3 existants.

Le batch peut contenir plusieurs snapshots anciens mais valides. Le client doit les traiter selon leur ordre et leurs timestamps source, sans les rejeter uniquement parce que leur réception est tardive. En revanche, la latence de livraison doit rester mesurée séparément : elle affecte la réactivité, même si elle ne détruit plus la trajectoire.

## Observabilité obligatoire

Mesurer séparément, autant que possible :

- fréquence/ticks de capture demandés et réellement exécutés ;
- dérive du timer Lua et gaps entre timestamps DCS ;
- temps passé à lire les unités et à remplir le ring ;
- nombre de recoveries et carriers actifs ;
- nombre de snapshots produits, invalides, lus et expirés ;
- capacité, occupation/high-water mark et nombre d’overflows ;
- plage de séquences conservée et servie ;
- taille/nombre de lots et snapshots par lot ;
- temps entre capture DCS et lecture/retour côté serveur ;
- temps de sérialisation et, si instrumentable, temps de file DLL ;
- erreurs par code gRPC ;
- côté LSO : délai de livraison, source age, gaps source, trous de séquence, duplications, p50/p95/p99/max et validité des gates.

Journaliser avec des identifiants de recovery non sensibles. Ne jamais exposer d’UCID dans le protobuf public, les logs partagés ou les artefacts LSO.

## Tests attendus dans le fork

### Tests Lua/unitaires ou harnais simulé

- module inactif sans recovery ;
- start idempotent et conflit de handle explicite ;
- stop et TTL libèrent l’état ;
- ordre monotone des séquences ;
- `after_sequence` exclusif et pagination à 100 ;
- répétition d’une lecture retourne un résultat dédupliquable ;
- ring plein écrase seulement les plus anciens éléments et annonce l’overflow ;
- demande trop ancienne retourne plage + perte explicite ;
- reset produit une nouvelle epoch ;
- disparition puis respawn d’une unité avec le même nom est détectée par ID ;
- plusieurs avions partagent une seule lecture carrier par tick ;
- plusieurs carriers/recoveries restent isolés ;
- erreur d’un avion ne bloque pas les autres paires ;
- aucun travail significatif au repos.

### Tests protobuf/serveur

- compatibilité additive des APIs existantes ;
- validation des champs, limites et tailles ;
- codes gRPC stables pour handle inconnu, argument invalide et état expiré ;
- réponse bornée même si le client demande une limite excessive ;
- retry après deadline sans perte destructive ;
- sérialisation de tous les statuts invalides/overflow/reset ;
- authentification cohérente avec les autres RPC.

### Tests intégrés artificiels

- retard de livraison de 300 ms puis 1 s sans perte des snapshots déjà capturés ;
- gel du producteur pendant 1 s produit un vrai gap source, pas de faux snapshots ;
- client absent plus de 30 s : overflow observable à la reprise ;
- reconnect entre lecture et retry ;
- mission rotation et reload Lua ;
- 1, 5, 10 et davantage de recoveries actives avec carriers partagés ;
- mesure mémoire/CPU/temps Lua à 10 et 20 Hz.

## Validation live et critères d’acceptation

Chaque run doit capturer : commit LSO, commit/tag DCS-gRPC, version stubs/serveur, build DCS, mission et hashes `.miz`, DLL et Lua. Les runs ne sont comparables que si ces éléments sont identiques ou si les différences sont explicitement attribuées.

Matrice minimale :

- positions-only unary comme contrôle ;
- positions bufferisées à 10 Hz puis 20 Hz ;
- mode normal avec/sans ACMI ;
- hook 2/4 Hz, sans l’intégrer au snapshot ;
- détection suspendue ou active ;
- un avion/un carrier, recoveries simultanées et carrier partagé ;
- carrier droit, tournant et accélérant ;
- reconnect, rotation de mission, respawn avion/carrier ;
- délais artificiels 300 ms et 1 s ;
- scénarios CATOBAR câbles 1–4 et V/STOL sans régression.

Critères essentiels :

- un retard de transport n’efface pas les snapshots capturés ;
- toute perte source est visible par séquence/plage/overflow ;
- aucune gate n’est reconstruite sans preuves brutes ;
- les snapshots avion/carrier ont un timestamp source commun documenté ;
- aucun mélange de mission, epoch, recovery ou incarnation d’unité ;
- aucune note favorable si les preuves restent insuffisantes ;
- aucun impact non mesuré/masqué sur FPS ou tick DCS ;
- p99 des gaps **source** compatible avec le besoin ou cause de perte précisément expliquée ;
- sorties et événements secondaires n’influencent pas la capture.

La cible LSO active reste 10 Hz et le bracket gate reste limité à 300 ms. La capture 20 Hz fournit de la marge, mais son acceptation dépend du coût réel dans DCS.

## Version, migration et rollback

- Ne pas modifier seulement les protos dans la copie `docs/DCS-gRPC-0.9.0/` du dépôt LSO : travailler dans le vrai clone `sevenfifty777/rust-server`.
- Créer une branche dédiée et enregistrer le commit exact revu.
- Mettre à jour la version/changelog du fork selon son processus de release.
- Générer les stubs et construire DLL/package Lua ensemble ; ne pas déployer un mélange de versions.
- Vérifier le tag, le SHA complet, les changements Cargo/protobuf et les dépendances.
- Dans LSO, mettre à jour le tag seulement après revue, tests, audit et smoke test DCS.
- Conserver le pin 0.9.0 actuel et le polling unary comme rollback jusqu’à promotion de la nouvelle source.
- Un simple succès de connexion ne prouve pas la compatibilité : des RPC peuvent être absents ou avoir un comportement différent.

## Décisions recommandées sur les questions de conception

Ces décisions résultent de la lecture du fork courant. Celui-ci contient déjà le package `dcs.recovery.v0`, le service `RecoveryService`, le RPC unary `GetRecoverySnapshot`, son handler `src/rpc/recovery.rs` et la méthode mission Lua `getRecoverySnapshot`. Le serveur est actuellement en version workspace 0.9.1 sur `main`. L’authentification existante est un intercepteur global par `X-API-Key`, sans rôles ni propagation actuelle de l’identité du client jusqu’au Lua. Les recommandations ci-dessous sont des contrats de conception ; leurs valeurs de charge restent à confirmer par les tests artificiels puis dans DCS réel.

### 1. Service et package protobuf

Étendre additivement `dcs.recovery.v0.RecoveryService`, sans créer un package concurrent et sans modifier ni retirer `GetRecoverySnapshot`. Ajouter des RPC explicitement nommés, par exemple :

```protobuf
rpc StartRecoveryTelemetry(StartRecoveryTelemetryRequest)
    returns (StartRecoveryTelemetryResponse);
rpc ReadRecoveryTelemetry(ReadRecoveryTelemetryRequest)
    returns (ReadRecoveryTelemetryResponse);
rpc StopRecoveryTelemetry(StopRecoveryTelemetryRequest)
    returns (StopRecoveryTelemetryResponse);
```

`Start` enregistre la paire et crée ou renouvelle son bail, `Read` retourne un lot idempotent non destructif, et `Stop` libère la paire. Le nom « telemetry » décrit le service rendu sans imposer le détail interne du ring. Un futur stream devra réutiliser les mêmes messages de snapshot et les mêmes identifiants. Le changement reste compatible protobuf : nouveaux RPC, messages et numéros de champs uniquement.

### 2. Séquence et tick global

Utiliser une `uint64 sequence` **par recovery**, démarrant à 1 dans une epoch, car `after_sequence` devient alors simple, indépendant et sans trous causés par les autres recoveries. Ajouter séparément une `uint64 capture_tick` globale à l’epoch, incrémentée une fois par callback réel du collecteur. Toutes les recoveries traitées dans ce callback partagent ce `capture_tick` ; cela permet aussi de démontrer qu’une lecture carrier a été réutilisée.

La clé d’idempotence est `(source_epoch, recovery_handle, sequence)`. La clé de partage carrier est `(source_epoch, carrier_id, capture_tick)`. Les entiers restent très en dessous de la limite exacte des nombres Lua (`2^53`) à 20 Hz ; le module doit néanmoins refuser ou renouveler l’epoch avant cette limite au lieu de perdre de la précision.

### 3. Forme et génération de l’epoch

Exposer `source_epoch` comme une chaîne opaque, de préférence un UUID aléatoire de 128 bits généré par la DLL Rust à chaque chargement effectif du module collecteur puis conservé dans l’état Lua. Ajouter pour cela une petite fonction native, par exemple `grpc.newSessionId()`, plutôt que de dépendre de `math.random`, de l’horloge DCS ou d’un identifiant fourni par le client.

Une rotation de mission, un reload Lua ou un redémarrage du serveur doit créer une nouvelle epoch. Une simple reconnexion réseau du client ne doit pas la changer tant que le producteur Lua et son ring ont survécu. Si la génération native n’est pas disponible dans le premier prototype, combiner plusieurs temps DCS et un nonce reste un repli de test, pas le contrat de production.

### 4. Séquence lorsqu’une unité est invalide

Oui : chaque recovery consomme exactement une séquence à chaque callback de capture réellement exécuté, même si l’avion, le carrier ou les deux sont invalides. Le snapshot contient alors les observations invalides explicites au lieu d’omettre la séquence. Cela distingue une tentative effectuée mais inexploitable d’un callback qui n’a jamais été exécuté.

Ne jamais créer de snapshot pour « rattraper » un tick scheduler manqué. Un gel du producteur se traduit par un écart entre timestamps DCS et par un compteur diagnostique de périodes probablement manquées, pas par des données synthétiques.

### 5. Statuts d’unité et tick manqué

Modéliser séparément chaque côté de la paire avec un `UnitObservationStatus` au minimum :

- `VALID` : ID attendu confirmé et transform complet ;
- `NOT_FOUND` : aucun objet ne correspond au nom attendu ;
- `ID_MISMATCH` : le nom existe à nouveau mais son ID DCS diffère de l’ID enregistré ;
- `READ_ERROR` : une API DCS a levé une exception pendant la lecture ;
- `INVALID_DATA` : lecture terminée mais transform absent, non numérique ou non fini.

Conserver `UNSPECIFIED = 0` selon la convention protobuf. Une erreur sur une unité ne doit ni annuler la séquence ni empêcher la capture des autres recoveries. Le snapshot transporte le nom et l’ID attendus, ainsi que l’ID résolu lorsqu’il existe, mais pas une trace Lua non bornée.

Un tick manqué n’est pas un statut d’unité puisqu’aucun snapshot n’existe. L’exposer dans les diagnostics du batch avec `last_capture_time`, `configured_period`, `observed_gap` et un `missed_capture_intervals` cumulatif estimé. Les erreurs de cycle de vie (`UNKNOWN`, `EXPIRED`, `STOPPED`, `EPOCH_MISMATCH`) appartiennent à un enum séparé ou aux métadonnées du batch.

### 6. Capacité et rétention

Appliquer deux bornes simultanées par recovery : une capacité fixe de **600 snapshots** et un âge maximal de **30 secondes DCS**. À 20 Hz nominal, les deux coïncident ; si la cadence baisse, la borne temporelle évite de prétendre à une rétention différente, et si elle monte accidentellement, la capacité protège la mémoire.

Nettoyer par âge pendant les callbacks de capture et les opérations start/read/stop, puis appliquer la capacité. Distinguer dans les compteurs les expirations normales par âge des écrasements par capacité. Dans les deux cas, si `after_sequence + 1` précède `oldest_available_sequence`, `Read` annonce une perte et sa cause (`RETENTION_EXPIRED` ou `CAPACITY_OVERFLOW`). La mémoire reste ainsi déterministe ; la cadence configurée et la cadence observée sont renvoyées séparément.

### 7. Limites et protection contre l’abus

Commencer avec des limites configurables prudentes : **16 recoveries actives**, **8 carriers distincts**, 600 snapshots par recovery, 100 snapshots par lot, et 128 octets UTF-8 pour un handle ou un nom. Fixer aussi des plafonds codés non contournables, par exemple 64 recoveries et 32 carriers, jusqu’à ce que les mesures justifient une hausse. Les limites initiales de 16/8 sont des valeurs de départ, pas une capacité validée dans DCS.

`Start` au-delà d’une limite retourne `RESOURCE_EXHAUSTED` sans modifier l’état existant. Valider longueur, caractères, cadence et limite avant toute allocation. Un même handle avec la même paire et les mêmes IDs est idempotent et renouvelle le bail sans vider le ring ; le même handle avec une autre identité retourne `ALREADY_EXISTS`. Le serveur doit borner la limite de lot même si le client demande davantage et prévoir un quota de lectures par propriétaire pour qu’un polling agressif ne monopolise pas la file mission.

Le fork devra ajouter `GRPC.errorResourceExhausted` et mapper `RESOURCE_EXHAUSTED` dans `src/rpc.rs`. Le mapping actuel ne traite explicitement que quelques codes et transformerait notamment `PERMISSION_DENIED` en `INTERNAL` ; corriger ce mapping fait partie du contrat d’erreurs stable.

### 8. TTL, bail et renouvellement

Utiliser un bail glissant de **60 secondes**, configurable avec des bornes administrateur raisonnables, par exemple 15 à 300 secondes. Un `Start` idempotent ou un `Read` authentifié avec handle, propriétaire et epoch valides renouvelle le bail. Aucun RPC keepalive séparé n’est nécessaire au départ puisque LSO lit normalement bien plus souvent que le TTL.

`Stop` est idempotent. À expiration, retirer immédiatement la paire active et son ring, mais conserver un petit tombstone borné pendant 30 secondes afin qu’un retry puisse distinguer `EXPIRED` de `UNKNOWN`; faire de même pour `STOPPED`. Les tombstones ne contiennent pas de transforms. Une lecture avec mauvaise epoch ne renouvelle rien. Le TTL utilise l’horloge DCS disponible dans l’environnement mission ; si la simulation est en pause, l’expiration retardée ne crée aucune charge de capture puisque les callbacks sont eux-mêmes arrêtés.

### 9. Mesure de la durée des lectures

Lire `capture_time` une seule fois avec `timer.getTime()` pour horodater le snapshot. Pour mesurer le coût réel à l’intérieur du callback, ajouter une horloge monotone native légère exposée par la DLL, par exemple `grpc.monotonicTimeNs()`. La lire avant la première résolution d’objet, autour de chaque lecture carrier/aircraft si l’instrumentation détaillée est activée, puis après la dernière insertion dans les rings.

Exposer au minimum `capture_duration_us`, et facultativement les offsets de début/fin de chaque observation relativement au début du callback. Ces champs restent diagnostiques. `timer.getTime()` ou `timer.getAbsTime()` peuvent ne pas avancer à l’intérieur d’une même frame et ne suffisent donc pas à prouver un coût nul ; en l’absence d’horloge native, consigner la mesure comme indisponible plutôt que renvoyer artificiellement zéro.

### 10. Représentation du transform carrier

Pour la première version, dupliquer le transform carrier dans chaque snapshot. La lecture DCS reste effectuée une seule fois par `(source_epoch, carrier_id, capture_tick)` puis la même valeur Lua est copiée dans les snapshots concernés. Cette forme simplifie fortement pagination, retry, sérialisation et consommation côté LSO, et chaque snapshot reste autonome.

Mesurer la taille des réponses et le coût de sérialisation avant d’introduire une table normalisée `carrier_by_tick`. Si la duplication devient matériellement coûteuse, ajouter plus tard un nouveau message ou RPC référencé par `capture_tick`, sans changer la sémantique de la première version.

### 11. Authentification, autorisation et propriété

Appliquer l’authentification `X-API-Key` existante aux trois RPC, mais ne pas considérer le handle comme un secret. Comme `Start` déclenche une charge périodique dans DCS, rendre la fonctionnalité désactivable par configuration (`recoveryTelemetry.enabled`) et recommander `auth.enabled = true`; si le serveur écoute ailleurs que sur loopback, refuser l’activation de la collecte sans authentification explicite.

Faire propager par l’intercepteur Rust un identifiant de propriétaire stable — le champ `client` associé à la clé, jamais le token — jusqu’au handler puis au Lua. Seul ce propriétaire peut read/stop/renouveler son handle. Quand l’authentification est volontairement désactivée sur loopback, utiliser un propriétaire anonyme unique et compter surtout sur les limites de ressources. Une évolution ultérieure pourra ajouter des scopes `recovery:read` et `recovery:write`; elle n’est pas nécessaire pour le premier contrat si toutes les clés authentifiées ont les mêmes droits. Ne jamais journaliser le token, un UCID ou des transforms complets au niveau normal.

### 12. Délais et gels déterministes dans les tests

Séparer le moteur de capture de l’API DCS : une fonction du type `captureTick(now, unitProvider, state)` reçoit une horloge, un résolveur d’unités et un état injectables. Un harnais Lua fournit de faux `Unit`, `timer`, transforms et exceptions, puis appelle manuellement chaque tick. Il peut ainsi avancer le temps sans attendre, remplacer un ID sous le même nom, faire échouer une seule lecture et vérifier exactement le ring.

Tester deux phénomènes distincts :

- retard de livraison : exécuter normalement plusieurs captures, retarder seulement l’appel ou la réponse `Read` de 300 ms puis 1 s dans un adaptateur IPC/client de test, et vérifier que tous les snapshots restent présents ;
- gel producteur : avancer la fausse horloge d’une seconde sans appeler `captureTick`, reprendre avec un seul tick réel et vérifier un vrai gap source sans snapshots inventés.

Les hooks de faute doivent être compilés ou chargés uniquement dans le harnais de test, jamais exposés comme RPC de production. Compléter par des tests Rust du mapping protobuf/gRPC et par une mission automatisée avec unités IA pour le smoke test ; seul ce dernier niveau peut renseigner le coût et le comportement réels dans DCS.

## Anti-solutions

- multiplier les `GetTransform` concurrents pour « rattraper » ;
- augmenter arbitrairement `GRPC.throughputLimit` sans mesure ;
- remplacer le batch par un stream non bufferisé ;
- ACK destructif/read-and-pop ;
- cache carrier sans epoch/tick/fraîcheur ;
- timestamp de réception utilisé comme temps de capture ;
- interpolation côté Lua ;
- calcul des gates ou grading côté Lua ;
- capture permanente de toutes les unités ;
- un timer par avion ;
- ajout obligatoire d’un script dans chaque `.miz` si le chargement global est possible ;
- repin automatique vers 0.9.1 ou une branche mouvante ;
- affirmation de performance depuis des tests offline.

## Plan de travail conseillé pour une future discussion Codex

1. Ouvrir le vrai dépôt du fork DCS-gRPC et lire ses éventuels `AGENTS.md`.
2. Vérifier branche, version, propreté du worktree et correspondance avec le commit 0.9.0 utilisé par LSO.
3. Cartographier proto -> stubs -> handler Rust/DLL -> nom de méthode Lua -> packaging.
4. Instrumenter d’abord la chaîne actuelle pour localiser scheduler, file, sérialisation et transport.
5. Rédiger le contrat protobuf additif et les invariants Lua, en tranchant les questions de séquence/epoch/TTL.
6. Implémenter le ring et le timer Lua avec tests déterministes.
7. Implémenter les RPC start/read/stop et les métadonnées d’overflow.
8. Vérifier format, tests, Clippy, audit et package du fork.
9. Générer/consommer les nouveaux stubs dans une branche LSO et brancher la source derrière `PositionCollector`.
10. Exécuter la matrice de validation live avec manifeste complet avant toute conclusion ou promotion.

## Sources internes utiles

Sources principales :

- `.agents/agents.md` : état intégral et règles de continuité du projet LSO ;
- `.ignore/prompt.md` : analyse du run live, diagnostic du polling et justification de la capture source ;
- `.ignore/tasking-v3.md` : roadmap, décisions batch/cadence/rétention et introduction du buffer Lua ;
- `docs/DCS_GRPC_FORK_MIGRATION.md` : pin du fork, versions Tonic/stubs et procédure d’upgrade ;
- `docs/RELIABILITY_ARCHITECTURE.md` : temps, gaps, gates, isolation et complétude ;
- `docs/BENCHMARK_PROTOCOL.md` : métriques, matrice live et critères de non-régression ;
- `docs/DATA_CONTRACTS.md` : compatibilité additive et provenance ;
- `docs/LIVE_VALIDATION.md` : manifeste et exigences de preuve live ;
- `README.md` et `CHANGES.md` : état utilisateur de la refonte Rust.

Référence DCS-gRPC 0.9.0 incluse :

- `docs/DCS-gRPC-0.9.0/Scripts/DCS-gRPC/grpc-mission.lua` ;
- `docs/DCS-gRPC-0.9.0/Scripts/DCS-gRPC/grpc.lua` ;
- `docs/DCS-gRPC-0.9.0/Scripts/DCS-gRPC/methods/unit.lua` ;
- `docs/DCS-gRPC-0.9.0/Scripts/DCS-gRPC/exporters/object.lua` ;
- `docs/DCS-gRPC-0.9.0/Tools/DCS-gRPC/protos/dcs/unit/v0/unit.proto` ;
- `docs/DCS-gRPC-0.9.0/Docs/DCS-gRPC/README.md`.

## Résumé durable

Le défaut à supprimer est la perte de positions provoquée par le couplage entre instant de capture et retour d’un RPC unary. La solution retenue est un collecteur Lua paresseux, unique et borné, qui capture l’avion et le carrier dans le même callback DCS, numérote les snapshots, les conserve environ 30 secondes et les expose par lots incrémentaux de 100 maximum via `after_sequence`. Le protocole doit rendre les retries idempotents, les resets et overflows explicites et le multi-recovery isolé. Rust reçoit les faits bruts et conserve toute la logique métier. Aucune performance ni compatibilité ne sera considérée acquise avant une validation DCS live versionnée.
