extern crate dirs;
extern crate libc;
extern crate openssl;

use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;

use self::libc::*;
use self::openssl::x509::X509;

use utils;

#[cfg(windows)]
#[link(name = "crypt32")]
extern "C" {
    fn CertAddEncodedCertificateToSystemStoreA(
        store_name: *const c_char,
        cert_encoded: *const c_uchar,
        cert_length: usize,
    ) -> c_int;
}

#[cfg(windows)]
pub fn install() {
    let store_name = CString::new("ROOT").unwrap();

    let mut x509 = File::open(utils::ca_path())
        .expect(&format!("unable to load ca file in {}", utils::ca_path()));

    let mut x509_pem = Vec::new();
    x509.read_to_end(&mut x509_pem).unwrap();

    let x509 = X509::from_pem(x509_pem.as_slice()).unwrap();
    let x509_der = x509.to_der().unwrap();

    let r;
    unsafe {
        r = CertAddEncodedCertificateToSystemStoreA(
            store_name.as_ptr(),
            x509_der.as_ptr(),
            x509_der.len(),
        );
    }

    if r == 1 {
        println!("{:}", "install to windows trusted store success");
    } else {
        eprintln!("failed to install to windows trusted store");
    }

    println!("{:}", "if you are using FireFox, please import cert manually in :");
    println!("{:}", "FireFox -> Preferences -> Security -> View Certificate.");
    println!("{:}", "and import cert into ca tab.");

    Command::new("explorer")
        .args(&[utils::easycert_dir().replace("/", "\\")])
        .spawn().unwrap();
}

#[cfg(linux)]
pub fn install() {
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
