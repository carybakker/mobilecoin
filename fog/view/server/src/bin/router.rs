// Copyright (c) 2018-2022 The MobileCoin Foundation
#![deny(missing_docs)]

//! MobileCoin Fog View Router target
use mc_attest_verifier::Verifier;
use mc_common::logger::log;
use mc_fog_uri::FogViewUri;
use mc_fog_view_connection::FogViewGrpcClient;
use mc_fog_view_enclave::{SgxViewEnclave, ENCLAVE_FILE};
use mc_fog_view_server::{
    config::FogViewRouterConfig, fog_view_router_server::FogViewRouterServer,
};
use mc_util_cli::ParserWithBuildInfo;
use mc_util_grpc::GrpcRetryConfig;
use std::{env, str::FromStr, sync::Arc};

fn main() {
    mc_common::setup_panic_handler();
    let config = FogViewRouterConfig::parse();
    let (logger, _global_logger_guard) =
        mc_common::logger::create_app_logger(mc_common::logger::o!());

    let enclave_path = env::current_exe()
        .expect("Could not get the path of our executable")
        .with_file_name(ENCLAVE_FILE);
    log::info!(
        logger,
        "enclave path {}, responder ID {}",
        enclave_path.to_str().unwrap(),
        &config.client_responder_id
    );
    let sgx_enclave = SgxViewEnclave::new(
        enclave_path,
        config.client_responder_id.clone(),
        config.omap_capacity,
        logger.clone(),
    );
    let env = Arc::new(
        grpcio::EnvBuilder::new()
            .name_prefix("Main-RPC".to_string())
            .build(),
    );
    // TODO: Remove and get from a config.
    let mut fog_view_grpc_clients: Vec<FogViewGrpcClient> = Vec::new();
    for i in 0..50 {
        let shard_uri_string = format!("insecure-fog-view://node{}.test.mobilecoin.com:3225", i);
        let shard_uri = FogViewUri::from_str(&shard_uri_string).unwrap();
        let fog_view_grpc_client = FogViewGrpcClient::new(
            shard_uri,
            GrpcRetryConfig::default(),
            Verifier::default(),
            env.clone(),
            logger.clone(),
        );
        fog_view_grpc_clients.push(fog_view_grpc_client);
    }

    let mut router_server =
        FogViewRouterServer::new(config, sgx_enclave, fog_view_grpc_clients, logger);
    router_server.start();

    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
