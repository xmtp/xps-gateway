pub struct Signature {
    /// Signature of V
    pub v: i64,
    /// Signature of R
    pub r: Vec<u8>,
    /// Signature of S
    pub s: Vec<u8>,
}

pub struct GrantInstallationResult {
    pub status: String,
    pub message: String,
    pub transaction: String,
}
