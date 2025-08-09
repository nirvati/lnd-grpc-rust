use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-env-changed=LND_REPO_DIR");
    let dir = match std::env::var_os("LND_REPO_DIR") {
        Some(lnd_repo_path) => {
            let mut lnd_rpc_dir = PathBuf::from(lnd_repo_path);
            lnd_rpc_dir.push("lnrpc");
            lnd_rpc_dir
                .to_str()
                .expect("LND_REPO_DIR must be a valid UTF-8 path")
                .to_string()
        }
        None => "vendor".to_string(),
    };

    let protos = vec![
        "autopilotrpc/autopilot.proto",
        "chainrpc/chainkit.proto",
        "chainrpc/chainnotifier.proto",
        "devrpc/dev.proto",
        "invoicesrpc/invoices.proto",
        "lightning.proto",
        "stateservice.proto",
        "walletunlocker.proto",
        "lnclipb/lncli.proto",
        "neutrinorpc/neutrino.proto",
        "peersrpc/peers.proto",
        "routerrpc/router.proto",
        "signrpc/signer.proto",
        "verrpc/verrpc.proto",
        "walletrpc/walletkit.proto",
        "watchtowerrpc/watchtower.proto",
        "wtclientrpc/wtclient.proto",
    ];

    let proto_paths: Vec<_> = protos
        .iter()
        .map(|proto| {
            let mut path = PathBuf::from(&dir);
            path.push(proto);
            path.display().to_string()
        })
        .collect();

    tonic_prost_build::configure()
        .build_client(true)
        .build_server(false)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(&proto_paths, &[dir])?;

    Ok(())
}
