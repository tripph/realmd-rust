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
    XferCancel = 0x34
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
    FailUseBattleNet = 0x12
}