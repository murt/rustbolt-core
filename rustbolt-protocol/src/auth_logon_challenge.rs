/// Logon request from the client
pub struct AuthLogonChallengeReq {
    /// Name of user attempting logon (provided in uppercase)
    pub username: String,
}

impl AuthLogonChallengeReq {
    /// Constructor from byte string
    pub fn new(/*bytes: [u8; 64]*/) -> AuthLogonChallengeReq {
        return AuthLogonChallengeReq { username: String::from("HEY") }
    }
}

/// Response to a logon request.
pub struct AuthLogonChallengeRes {
    pub cmd: u8,
    pub error: u8,
    pub size: u16,
    pub game_name: [u8; 4],
    pub version_major: u8,
    pub version_minor: u8,
    pub version_patch: u8,
    pub build: u16,
    pub platform: [u8; 4],
    pub os: [u8; 4],
    pub country: [u8; 4],
    pub timezone_bias: u32,
    pub ip: u32,
    pub i_len: u8,
    pub i: [u8; 1],
}
