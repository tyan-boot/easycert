extern crate dirs;

use std::env;

use self::dirs::home_dir;

#[cfg(target_os = "linux")]
extern "C" {
    fn gethostname(name: *mut u8, len: usize) -> u32;
}
#[cfg(windows)]
pub fn hostname() -> String {
    env::var("COMPUTERNAME").unwrap()
}

#[cfg(unix)]
pub fn hostname() -> String {
    let mut name = vec![0; 64]; // 64 is max size of hostname in linux since 1.0
    unsafe {
        gethostname(name.as_mut_ptr(), 64);
    }

    let name: Result<&str, str::Utf8Error> = str::from_utf8(name.as_slice())
        .map(|n| n.trim_matches(char::from(0)))
        .or_else(|_| {
            println!("unable to get hostname, fallback to default");
            return Ok("tyanboot");
        });

    return name.unwrap().to_string();
}

pub fn easycert_dir() -> String {
    return format!("{}/.easycert", home_dir().unwrap().to_str().unwrap());
}

pub fn ca_path() -> String {
    return format!("{}/{}", easycert_dir(), "ca.pem");
}

pub fn key_path() -> String {
    return format!("{}/{}", easycert_dir(), "ca.key");
}