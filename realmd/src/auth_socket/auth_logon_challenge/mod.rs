use core::borrow::Borrow;

use mysql::prelude::{FromRow, Queryable};
use srp::groups::G_2048;
use srp::server::{SrpServer, UserRecord};

use auth_socket::auth_logon_challenge::auth_types::{Account, AuthCmds};
use mysql::Row;
use std::ops::Deref;

pub mod auth_types;

pub fn handleAuthLogonChallenge(buf: Vec<u8>, pool: mysql::Pool) -> Vec<u8> {
    //    let header = auth_types::get_header(buf, *cmd);
    //    let challenge = auth_types::get_logon_challenge(buf, header);
    let challenge = auth_types::AUTH_LOGON_CHALLENGE_C::from(buf);
    let mut pkt: Vec<u8> = vec![];
    println!("challenge: {}", challenge);
    //    ///- Normalize account name
    //    //TODO: utf8ToUpperOnlyLatin(_login); -- client already send account in expected form
    //
    //    //Escape the user login to avoid further SQL injection
    //    //Memory will be freed on AuthSocket object destruction
    //    _safelogin = _login;
    //    //TODO: LoginDatabase.escape_string(_safelogin);
    //
    pkt.push(auth_types::AuthCmds::LogonChallenge as u8);
    pkt.push(0x00);
    let account = getAccount(challenge.username, pool);

    //        result = LoginDatabase.PQuery("SELECT sha_pass_hash,id,locked,last_ip,v,s,security FROM account WHERE username = '%s'",_safelogin.c_str ());
    // - Get the password from the account table, upper it, and make the SRP6 calculation
    //                    std::string rI = fields[0].GetCppString();
    //
    //                    ///- Don't calculate (v, s) if there are already some in the database
    //                    std::string databaseV = fields[4].GetCppString();
    //                    std::string databaseS = fields[5].GetCppString();
    //
    //                    DEBUG_LOG("database authentication values: v='%s' s='%s'", databaseV.c_str(), databaseS.c_str());
    //
    //                    // multiply with 2, bytes are stored as hexstring
    //                    if(databaseV.size() != s_BYTE_SIZE*2 || databaseS.size() != s_BYTE_SIZE*2)
    //                    _SetVSFields(rI);
    //                    else
    //                    {
    //                        s.SetHexStr(databaseS.c_str());
    //                        v.SetHexStr(databaseV.c_str());
    //                    }
    //
    //                    b.SetRand(19 * 8);
    //                    BigNumber gmod = g.ModExp(b, N);
    //                    B = ((v * 3) + gmod) % N;
    //
    //                    MANGOS_ASSERT(gmod.GetNumBytes() <= 32);
    //
    //                    BigNumber unk3;
    //                    unk3.SetRand(16 * 8);
    //
    //                    ///- Fill the response packet with the result
    //                    pkt << uint8(WOW_SUCCESS);
    //
    //                    // B may be calculated < 32B so we force minimal length to 32B
    //                    pkt.append(B.AsByteArray(32));      // 32 bytes
    //                    pkt << uint8(1);
    //                    pkt.append(g.AsByteArray());
    //                    pkt << uint8(32);
    //                    pkt.append(N.AsByteArray(32));
    //                    pkt.append(s.AsByteArray());        // 32 bytes
    //                    pkt.append(unk3.AsByteArray(16));

    //

    //                        pkt << uint8(0);
    //
    //
    //                    _localizationName.resize(4);
    //                    for(int i = 0; i < 4; ++i)
    //                    _localizationName[i] = ch->country[4-i-1];
    //
    //                    LoadAccountSecurityLevels(account_id);
    //                    BASIC_LOG("[AuthChallenge] account %s is using '%c%c%c%c' locale (%u)", _login.c_str (), ch->country[3], ch->country[2], ch->country[1], ch->country[0], GetLocaleByName(_localizationName));
    //
    //                    _accountId = account_id;
    //
    //                    ///- All good, await client's proof
    //                    _status = STATUS_LOGON_PROOF;

    return match account {
        Ok(acc) =>  {
            do_srp(&acc);
            acc.sha_pass_hash.as_bytes().to_vec()
        },
        Err(e) => vec![],
    };
}

fn getAccount(username: String, pool: mysql::Pool) -> Result<Account, &'static str> {
    let r: Option<Row> = pool.get_conn().unwrap().query_first("SELECT * FROM account WHERE username = ".to_string() + &*username).unwrap();
    return Ok(Account::from_row(r.expect("Couldn't unwrap row result!")));
}
fn do_srp(account: &Account) {
    set_vs_fields(&account.sha_pass_hash);
//    let user = UserRecord {
//        username: account.username,
//        salt: [],
//        verifier: []
//    };
//    let server = SrpServer::<Sha256>::new(&user, &a_pub, &b, &G_2048)?;
    //                    ///- Don't calculate (v, s) if there are already some in the database
    //                    std::string databaseV = fields[4].GetCppString();
    //                    std::string databaseS = fields[5].GetCppString();
    //
    //                    DEBUG_LOG("database authentication values: v='%s' s='%s'", databaseV.c_str(), databaseS.c_str());
    //
    //                    // multiply with 2, bytes are stored as hexstring
    //                    if(databaseV.size() != s_BYTE_SIZE*2 || databaseS.size() != s_BYTE_SIZE*2)
    //                    _SetVSFields(rI);
    //                    else
    //                    {
    //                        s.SetHexStr(databaseS.c_str());
    //                        v.SetHexStr(databaseV.c_str());
    //                    }
    //
    //                    b.SetRand(19 * 8);
    //                    BigNumber gmod = g.ModExp(b, N);
    //                    B = ((v * 3) + gmod) % N;
    //
    //                    MANGOS_ASSERT(gmod.GetNumBytes() <= 32);
    //
    //                    BigNumber unk3;
    //                    unk3.SetRand(16 * 8);
    //
    //                    ///- Fill the response packet with the result
    //                    pkt << uint8(WOW_SUCCESS);
    //
    //                    // B may be calculated < 32B so we force minimal length to 32B
    //                    pkt.append(B.AsByteArray(32));      // 32 bytes
    //                    pkt << uint8(1);
    //                    pkt.append(g.AsByteArray());
    //                    pkt << uint8(32);
    //                    pkt.append(N.AsByteArray(32));
    //                    pkt.append(s.AsByteArray());        // 32 bytes
    //                    pkt.append(unk3.AsByteArray(16));

    //
}
fn set_vs_fields(hashed_password: &String) {
    use num::bigint::BigInt;
    use rand::prelude::*;
    let mut rng = rand::thread_rng();
    let mut b = [0u8; 64];
    rng.fill_bytes(&mut b);
    let N = num::BigInt::from_bytes_be(num::bigint::Sign::NoSign,"894B645E89E1535BBDAD5B8B290650530801B18EBFBF5E8FAB3C82872A3E9BB7".as_bytes());
//    let N = &"".as_bytes();
    let mut sha = sha1::Sha1::new();
    sha.update(&b);
    sha.update("0".as_bytes()); //todo: pull from account
    sha.update(&hashed_password.as_bytes());



    // Make the SRP6 calculation from hash in dB
//    void AuthSocket::_SetVSFields(const std::string& rI)
//    {
//      [x]  s.SetRand(s_BYTE_SIZE * 8);
//
//        BigNumber I;
//        I.SetHexStr(rI.c_str());
//
//        // In case of leading zeros in the rI hash, restore them
//        uint8 mDigest[SHA_DIGEST_LENGTH];
//        memset(mDigest, 0, SHA_DIGEST_LENGTH);
//        if (I.GetNumBytes() <= SHA_DIGEST_LENGTH)
//        { memcpy(mDigest, I.AsByteArray(), I.GetNumBytes()); }
//
//        std::reverse(mDigest, mDigest + SHA_DIGEST_LENGTH);
//
//     [x]   Sha1Hash sha;
//     [x]   sha.UpdateData(s.AsByteArray(), s.GetNumBytes());
//        sha.UpdateData(mDigest, SHA_DIGEST_LENGTH);
//        sha.Finalize();
//        BigNumber x;
//        x.SetBinary(sha.GetDigest(), sha.GetLength());
//        v = g.ModExp(x, N);
//        // No SQL injection (username escaped)
//        const char* v_hex, *s_hex;
//        v_hex = v.AsHexStr();
//        s_hex = s.AsHexStr();
//        LoginDatabase.PExecute("UPDATE account SET v = '%s', s = '%s' WHERE username = '%s'", v_hex, s_hex, _safelogin.c_str());
//        OPENSSL_free((void*)v_hex);
//        OPENSSL_free((void*)s_hex);
//    }
}
