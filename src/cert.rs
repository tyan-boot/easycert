extern crate openssl;
extern crate rand;

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::net::IpAddr;
use std::path::Path;

use self::openssl::asn1::*;
use self::openssl::bn::BigNum;
use self::openssl::hash::MessageDigest;
use self::openssl::pkey::PKey;
use self::openssl::pkey::Private;
use self::openssl::rsa::Rsa;
use self::openssl::stack::Stack;
use self::openssl::x509::extension::*;
use self::openssl::x509::X509;
use self::openssl::x509::X509Extension;
use self::openssl::x509::*;

use self::rand::prelude::random;

pub struct Cert {
    ca: Option<X509>,

    pkey: Option<PKey<Private>>,
}

pub enum SubjectName {
    Ip(String),
    Dns(String),
}

impl Cert {
    pub fn new() -> Cert {
        Cert {
            ca: None,
            pkey: None,
        }
    }

    pub fn init(&mut self, name: &str, length: u32, force: bool) {
        if (Path::new("ca.key").exists() || Path::new("ca.pem").exists()) && !force {
            eprintln!(
                "{}",
                "old ca exist, use -f or --force to force init a new ca"
            );
        } else {
            self.generate_ca(name, length);
            self.save_ca();

            println!("{}", "new ca generated!");
        }
    }

    pub fn load_ca(&mut self) -> bool {
        let ca = File::open("ca.pem");
        let key = File::open("ca.key");

        if let (Ok(mut ca), Ok(mut key)) = (ca, key) {
            let mut buf = Vec::<u8>::new();
            ca.read_to_end(&mut buf).unwrap();
            let x509 =
                X509::from_pem(buf.as_slice()).expect("unable to load ca! may be wrong format.");

            self.ca = Some(x509);

            buf.clear();

            key.read_to_end(&mut buf).unwrap();
            let pkey =
                PKey::private_key_from_pem(buf.as_slice()).expect("unable to load private key.");

            self.pkey = Some(pkey);

            return true;
        } else {
            return false;
        }
    }

    pub fn generate_key(&self, len: u32) -> PKey<Private> {
        let rsa: Rsa<Private> = Rsa::generate(len).unwrap();

        let pkey: PKey<Private> = PKey::from_rsa(rsa).unwrap();

        pkey
    }

    pub fn generate_ca(&mut self, name: &str, len: u32) {
        let pkey = self.generate_key(len);

        let mut x509_name = X509NameBuilder::new().unwrap();
        x509_name.append_entry_by_text("CN", name).unwrap();
        let x509_name = x509_name.build();

        let serial = BigNum::from_u32(random()).unwrap();
        let serial = serial.to_asn1_integer().unwrap();

        let mut x509 = X509Builder::new().unwrap();
        x509.set_version(2).unwrap();
        x509.set_subject_name(&x509_name).unwrap();
        x509.set_issuer_name(&x509_name).unwrap();
        x509.set_not_before(&Asn1Time::days_from_now(0).unwrap())
            .unwrap();
        x509.set_not_after(&Asn1Time::days_from_now(3650).unwrap())
            .unwrap();
        x509.set_serial_number(&serial).unwrap();

        x509.set_pubkey(&pkey).unwrap();

        // build ext

        let mut bc = BasicConstraints::new();
        let bc = bc.ca().build().unwrap();
        x509.append_extension(bc).unwrap();

        let mut usage = KeyUsage::new();
        let usage = usage
            .digital_signature()
            .crl_sign()
            .key_cert_sign()
            .build()
            .unwrap();
        x509.append_extension(usage).unwrap();

        let sid = SubjectKeyIdentifier::new();
        let sid = sid.build(&x509.x509v3_context(None, None)).unwrap();
        x509.append_extension(sid).unwrap();

        let mut aid = AuthorityKeyIdentifier::new();
        let aid = aid.keyid(true)
            .build(&x509.x509v3_context(None, None))
            .unwrap();
        x509.append_extension(aid).unwrap();

        x509.sign(&pkey, MessageDigest::sha512()).unwrap();

        let x509 = x509.build();

        self.pkey = Some(pkey);
        self.ca = Some(x509);
    }

    pub fn save_ca(&mut self) {
        if let Some(ref ca) = self.ca {
            let pem = ca.to_pem().unwrap();

            let mut file = File::create("ca.pem").unwrap();
            file.write(pem.as_slice()).unwrap();
        }

        if let Some(ref key) = self.pkey {
            let key = key.private_key_to_pem_pkcs8().unwrap();

            let mut file = File::create("ca.key").unwrap();
            file.write(key.as_slice()).unwrap();
        }
    }

    pub fn prase_name(&self, name: &str) -> SubjectName {
        if name.parse::<IpAddr>().is_ok() {
            SubjectName::Ip(name.to_string())
        } else {
            SubjectName::Dns(name.to_string())
        }
    }

    pub fn new_cert(&mut self, names: Vec<&str>, out_name: Option<&str>) {
        if let (Some(ref ca), Some(ref ca_key)) = (&self.ca, &self.pkey) {
            let pkey = self.generate_key(4096);

            let first_name = &names[0];

            let mut x509_name = X509NameBuilder::new().unwrap();
            x509_name.append_entry_by_text("CN", first_name).unwrap();
            let x509_name = x509_name.build();

            let serial = BigNum::from_u32(random()).unwrap();
            let serial = serial.to_asn1_integer().unwrap();

            let issuer_name = ca.subject_name();

            let mut x509 = X509Builder::new().unwrap();
            x509.set_version(2).unwrap();
            x509.set_subject_name(&x509_name).unwrap();
            x509.set_issuer_name(&issuer_name).unwrap();
            x509.set_not_before(&Asn1Time::days_from_now(0).unwrap())
                .unwrap();
            x509.set_not_after(&Asn1Time::days_from_now(3650).unwrap())
                .unwrap();
            x509.set_serial_number(&serial).unwrap();
            x509.set_pubkey(&pkey).unwrap();

            let mut exts = Stack::<X509Extension>::new().unwrap();

            let bc = BasicConstraints::new();
            let bc = bc.build().unwrap();
            exts.push(bc).unwrap();

            let mut usage = KeyUsage::new();
            let usage = usage
                .key_encipherment()
                .digital_signature()
                .build()
                .unwrap();
            exts.push(usage).unwrap();

            {
                let ctx = &x509.x509v3_context(Some(&ca), None);
                let mut aid = AuthorityKeyIdentifier::new();
                let aid = aid.keyid(true).build(&ctx).unwrap();

                exts.push(aid).unwrap();

                let mut sid = SubjectKeyIdentifier::new();
                let sid = sid.build(&ctx).unwrap();

                exts.push(sid).unwrap();

                let mut alt_name = SubjectAlternativeName::new();
                for name in &names {
                    match self.prase_name(&name) {
                        SubjectName::Ip(ip) => {
                            alt_name.ip(&ip);
                        }
                        SubjectName::Dns(dns) => {
                            alt_name.dns(&dns);
                        }
                    }
                }
                let alt_name = alt_name.build(&ctx).unwrap();
                exts.push(alt_name).unwrap();
            }

            for ext in exts {
                x509.append_extension(ext).unwrap();
            }

            x509.sign(&pkey, MessageDigest::sha512()).unwrap();
            x509.sign(&ca_key, MessageDigest::sha512()).unwrap();

            let x509 = x509.build();

            let mut _out_name: &str;

            if let Some(out_name) = out_name {
                _out_name = out_name;
            } else {
                _out_name = first_name;
            }

            let _out_name = _out_name.replace("*", "_wildcard");

            let key_name = format!("{}.key", &_out_name);
            let pem_name = format!("{}.crt", &_out_name);

            let mut f = File::create(&key_name).unwrap();
            let key = pkey.private_key_to_pem_pkcs8().unwrap();
            f.write(key.as_slice()).unwrap();

            let mut f = File::create(&pem_name).unwrap();
            let pem = x509.to_pem().unwrap();
            f.write(pem.as_slice()).unwrap();

            println!("New cert created to {} {}", key_name, pem_name);
        } else {
            eprintln!("No ca found, please init first!");
        }
    }
}
