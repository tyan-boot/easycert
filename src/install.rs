extern crate dirs;

use std::fs::copy;
use std::fs::DirBuilder;
use std::path::Path;
use std::process::Command;

use utils;

pub fn install() {
    if cfg!(unix) {
        install_linux();
    }
}

pub fn install_linux() {
    let ssl_dirs = vec![
        (
            "/etc/ca-certificates/trust-source/anchors",
            "update-ca-trust",
        ),
        ("/etc/pki/ca-trust/source/anchors", "update-ca-trust"),
    ];

    for (ssl_dir, update_command) in ssl_dirs {
        if Path::new(ssl_dir).exists() {
            let target_path = format!("{}/easycert-ca.pem", ssl_dir);

            Command::new("sudo")
                .args(&["cp", &utils::ca_path(), &target_path])
                .output()
                .unwrap();

            Command::new("sudo")
                .args(&[update_command])
                .output()
                .unwrap();

            return;
        }
    }

    eprintln!("can't determinate system trusted store path");
    eprintln!("failed to install certificate, try install to nss store");
}
