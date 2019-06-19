use std::fmt;

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct AUTH_HEADER {
    pub cmd: AuthCmds,
    pub error: u8,
    pub size: u16,
}

pub fn getHeader(packet: [u8; 256]) -> AUTH_HEADER {
    AUTH_HEADER {
        cmd: self::AuthCmds::from_u8(packet[0]),
        error: packet[1],
        size: (packet[2] + packet[3]) as u16,

    }
}

#[derive(Debug)]
pub struct AUTH_LOGON_CHALLENGE_C {
    pub header: AUTH_HEADER,
    pub gamename: [u8; 4],
    pub version1: u8,
    pub version2: u8,
    pub version3: u8,
    pub build: u16,
    pub platform: [u8; 4],
    pub os: [u8; 4],
    pub country: [u8; 4],
    pub timezone_bias: u32,
    pub ip: u32,
    pub I_len: u8,
    pub username: String,
}

impl fmt::Display for AUTH_HEADER {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\ncmd: {:?}\nerror: {}\n size: {}\n", self.cmd, self.error, self.size)
    }
}

impl fmt::Display for AUTH_LOGON_CHALLENGE_C {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{{ \nheader: {}\n gamename: {}\n version1: {}\n version2: {}\n version3: {}\n\
                 build: {}\n platform: {}\n os: {}\n country: {}\n timezone_bias: {}\n, ip: {}\n, I_len: {}\n, I: {}\n
            }}"
               , self.header, create_string(self.gamename.to_vec()), self.version1, self.version2, self.version3,
               self.build, create_string(self.platform.to_vec()), create_string(self.os.to_vec()), create_string(self.country.to_vec()),
               self.timezone_bias, self.ip, self.I_len, self.username
        )
    }
}

fn create_string(input: Vec<u8>) -> String {
    let mut output = String::new();
    for inp in input.iter() {
        output.push(*inp as char);
    }
    return output;
}

// TODO: little-endian conversion, currently just reversing byte order?...
pub fn getLogonChallenge(packet: [u8; 256], head: AUTH_HEADER) -> AUTH_LOGON_CHALLENGE_C {
    let username_offset: usize = 34+packet[33] as usize;
    AUTH_LOGON_CHALLENGE_C {
        header: head,
        gamename: [packet[7], packet[6], packet[5], packet[4]],
        version1: packet[8],
        version2: packet[9],
        version3: packet[10],
        build: as_u16_be(&packet[11..13]),
        platform: [packet[16], packet[15], packet[14], packet[13]],
        os: [packet[20], packet[19], packet[18], packet[17]],
        country: [packet[24], packet[23], packet[22], packet[21]],
        timezone_bias: as_u32_be(&packet[25..29]),
        ip: as_u32_be(&packet[30..34]),
        I_len: packet[33],
        username: std::str::from_utf8(&packet[34..username_offset]).unwrap().to_string()
    }
}

fn as_u32_be(array: &[u8]) -> u32 {
    ((array[0] as u32) << 24) +
        ((array[1] as u32) << 16) +
        ((array[2] as u32) << 8) +
        ((array[3] as u32) << 0)
}

fn as_u32_le(array: &[u8]) -> u32 {
    ((array[0] as u32) << 0) +
        ((array[1] as u32) << 8) +
        ((array[2] as u32) << 16) +
        ((array[3] as u32) << 24)
}

fn as_u16_be(array: &[u8]) -> u16 {
    ((array[0] as u16) << 8) +
        ((array[1] as u16) << 0)
}

fn as_u16_le(array: &[u8]) -> u16 {
    ((array[0] as u16) << 0) +
        ((array[1] as u16) << 8)
}

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum AuthCmds {
    LogonChallenge = 0x00,
    LogonProof = 0x01,
    ReconnectChallenge = 0x02,
    ReconnectProof = 0x03,
    RealmList = 0x10,
    XferInitiate = 0x30,
    XferData = 0x31,
    XferAccept = 0x32,
    XferResume = 0x33,
    XferCancel = 0x34,
}

impl AuthCmds {
    pub fn from_u8(value: u8) -> AuthCmds {
        match value {
            0x00 => AuthCmds::LogonChallenge,
            0x01 => AuthCmds::LogonProof,
            _ => panic!("Unknown AuthCmd: {}", value)
        }
    }
}

pub enum AuthResults {
    Success = 0x00,
    FailUnknown0 = 0x01,
    FailUnknown1 = 0x02,
    FailBanned = 0x03,
    FailUnknownAccount = 0x04,
    FailPassword = 0x05,
    FailAlreadyOnline = 0x06,
    FailNoTime = 0x07,
    FailDBBusy = 0x08,
    FailVersionInvalid = 0x09,
    FailVersionUpdate = 0x0A,
    FailInvalidServer = 0x0B,
    FailSuspended = 0x0C,
    FailNoAccess = 0x0D,
    SuccessSurvey = 0x0E,
    FailParentControl = 0x0F,
    FailLockedEnforced = 0x10,
    FailTrialEnded = 0x11,
    FailUseBattleNet = 0x12,
}

