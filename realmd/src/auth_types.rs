#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct AUTH_LOGON_CHALLENGE_C {
    pub cmd: u8,
    pub error: u8,
    pub size: u16,
    pub gamename: u8,
    pub version1: u8,
    pub version2: u8,
    pub version3: u8,
    pub build: u16,
    pub platform: u8,
    pub os: u8,
    pub country: u8,
    pub timezone_bias: u32,
    pub ip: u32,
    pub I_len: u8,
    pub I: u8
}

