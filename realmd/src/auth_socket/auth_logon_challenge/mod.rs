use auth_socket::auth_logon_challenge::auth_types::AuthCmds;

pub mod auth_types;

pub fn handleAuthLogonChallenge(buf: &Vec<u8>, cmd: &AuthCmds) -> Vec<u8> {
    let header = auth_types::get_header(buf, *cmd);
    let challenge = auth_types::get_logon_challenge(buf, header);
    println!("challenge: {}", challenge);
    return vec!(40, 1, 40);
}
