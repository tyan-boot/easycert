use std::env;
use std::str;

extern "C" {
    fn gethostname(name: *mut u8, len: usize) -> u32;
}

pub fn hostname() -> String {
    if cfg!(windows) {
        env::var("COMPUTERNAME").unwrap()
    } else if cfg!(unix) {
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
    } else {
        println!("unable to detect platform, use default cn=tyanboot");
        "tyanboot".to_string()
    }
}
