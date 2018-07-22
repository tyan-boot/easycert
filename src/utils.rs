use std::env;

pub fn hostname() -> String {
    if cfg!(windows) {
        env::var("COMPUTERNAME").unwrap()
    } else if cfg!(unix) {
        env::var("HOSTNAME").unwrap()
    } else {
        println!("unable to detect platform, use default cn=tyanboot");
        "tyanboot".to_string()
    }
}
