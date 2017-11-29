#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
struct AUTH_LOGON_CHALLENGE_C {
    cmd: u8,
    error: u8,
    size: u16,
    gamename: u8,
    version1: u8,
    version2: u8,
    version3: u8,
    build: u16,
    platform: u8,
    os: u8,
    country: u8,
    timezone_bias: u32,
    ip: u32,
    I_len: u8,
    I: u8
}
fn main() {
    println!("Hello, realm!");
    let challenge = AUTH_LOGON_CHALLENGE_C{
        cmd: 1,
        error: 0,
        size: 0u16,
        gamename: 0,
        version1: 1,
        version2: 2,
        version3: 3,
        build: 34,
        platform: 1,
        os: 0,
        country: 1,
        timezone_bias: 0u32,
        ip: 1u32,
        I_len: 5u8,
        I: 5
    };
    println!("{:?}",challenge);
}
