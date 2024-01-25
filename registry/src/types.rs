/// Represents the components of an Ethereum-style digital signature.
///
/// In Ethereum, a signature is typically comprised of three parts: `v`, `r`, and `s`.
/// This struct provides a way to store these components in a structured format.
///
/// # Fields
/// * `v` - An `i64` representing the recovery byte. The `v` value is part of the signature
///   which, along with `r` and `s`, helps to recover the public key from the signature.
/// * `r` - A `Vec<u8>` representing the first 32 bytes of the signature. In the context of
///   ECDSA, `r` is part of the (r, s) value pair that constitutes the actual signature.
/// * `s` - A `Vec<u8>` representing the second 32 bytes of the signature, working alongside
///   `r` to form the signature pair.
///
pub struct Signature {
    /// Signature of V
    pub v: i64,
    /// Signature of R
    pub r: Vec<u8>,
    /// Signature of S
    pub s: Vec<u8>,
}

/// GrantInstallationResult represents the result of a grant installation operation in the DID registry.
///
/// This struct encapsulates the outcome of an attempt to grant an installation,
/// providing details about the operation's status, a descriptive message, and the
/// transaction identifier associated with the blockchain transaction.
///
/// # Fields
/// * `status` - A `String` indicating the outcome status of the operation. Typically, this
///   would be values like "Success" or "Failure".
/// * `message` - A `String` providing more detailed information about the operation. This
///   can be a success message, error description, or any other relevant information.
/// * `transaction` - A `String` representing the unique identifier of the transaction on the
///   blockchain. This can be used to track the transaction in a blockchain explorer.
///
pub struct GrantInstallationResult {
    pub status: String,
    pub message: String,
    pub transaction: String,
}
