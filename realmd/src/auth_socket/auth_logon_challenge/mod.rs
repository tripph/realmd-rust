pub mod auth_types;

pub fn handleAuthLogonChallenge(buf: &Vec<u8>) -> Vec<u8> {
    let header = auth_types::get_header(buf);
    let challenge = auth_types::get_logon_challenge(buf, header);
    println!("challenge: {}", challenge);
    return vec!(40, 1, 40);
}
