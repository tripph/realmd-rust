#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct AUTH_LOGON_CHALLENGE_C {
    pub cmd: u8,
    pub error: u8,
    pub size: u16,
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
    pub I: [u8;1]
}

pub fn from_packet(packet: &Vec<u8>) -> AUTH_LOGON_CHALLENGE_C{
    AUTH_LOGON_CHALLENGE_C{
        cmd: packet[0],
        error: packet[1],
        size: (packet[2] + packet[3]) as u16,
        gamename: [packet[4], packet[5], packet[6], packet[7]],
        version1: packet[8],
        version2: packet[9],
        version3: packet[10],
        build: (packet[11] + packet[12]) as u16,
        platform: [packet[13], packet[14], packet[15], packet[16]],
        os:  [packet[17], packet[18], packet[19], packet[20]],
        country: [packet[21], packet[22], packet[23], packet[24]],
        timezone_bias: (packet[25] + packet[26] + packet[27] + packet[28]) as u32,
        ip: (packet[29] + packet[30] + packet[31] + packet[32]) as u32,
        I_len: packet[33],
        I: [packet[34]],
    }
}

