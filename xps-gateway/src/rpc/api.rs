//! Trait Interface Definitions for XPS JSON-RPC

use jsonrpsee::{proc_macros::rpc, types::ErrorObjectOwned};

use gateway_types::{Message, Signature, XmtpAttributeType};

/// XPS JSON-RPC Interface Methods
#[rpc(server, client, namespace = "xps")]
pub trait Xps {
    // Placeholder for send_message, see [the discussion](https://github.com/xmtp/xps-gateway/discussions/11)
    #[method(name = "sendMessage")]
    async fn send_message(&self, _message: Message) -> Result<(), ErrorObjectOwned>;

    /// removes the contact bundle for the XMTP device installation. Request must be made to a
    /// valid DID with an XMTP profile.
    ///
    /// # Arguments
    ///
    /// * `did` - the DID of the XMTP device installation
    /// * `name` - the name of the contact bundle
    /// * `value` - the value of the contact bundle
    /// * `signature` - the signature of the contact bundle
    #[method(name = "revokeInstallation")]
    async fn revoke_installation(
        &self,
        did: String,
        name: XmtpAttributeType,
        value: Vec<u8>,
        signature: Signature,
    ) -> Result<(), ErrorObjectOwned>;
    /// # Documentation for JSON RPC Endpoint: `status`

    /// ## Overview

    /// The `status` method is used to query the gateway status.

    /// ## JSON RPC Endpoint Specification

    /// ### Method Name
    /// `status`

    /// ### Request Parameters
    /// none

    /// ### Request Format
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "method": "status",
    ///   "id": 1
    /// }
    /// ```

    /// - `jsonrpc`: Specifies the version of the JSON RPC protocol being used. Always "2.0".
    /// - `method`: The name of the method being called. Here it is "status".
    /// - `id`: A unique identifier established by the client that must be number or string. Used for correlating the response with the request.

    /// ### Response Format
    /// The response will typically include the result of the operation or an error if the operation was unsuccessful.

    /// #### Success Response
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "result": "OK",
    ///   "id": 1
    /// }
    /// ```

    /// - `result`: Contains data related to the success of the operation. The nature of this data can vary based on the implementation.

    /// #### Error Response
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "error": {
    ///     "code": <error_code>,
    ///     "message": "<error_message>"
    ///   },
    ///   "id": 1
    /// }
    /// ```

    /// - `error`: An object containing details about the error.
    ///   - `code`: A numeric error code.
    ///   - `message`: A human-readable string describing the error.

    /// ### Example Usage

    /// #### Request
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "method": "status",
    ///   "id": 42
    /// }
    /// ```

    /// #### Response
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "result": "OK",
    ///   "id": 42
    /// }
    /// ```

    /// ### Command Line Example
    /// ```bash
    /// $ $ curl -H "Content-Type: application/json" -d '{"id":7000, "jsonrpc":"2.0", "method":"xps_status"}' http:///localhost:34695
    /// {"jsonrpc":"2.0","result":"OK","id":7000}
    /// ```

    /// ### Notes
    /// - The system should have proper error handling to deal with invalid requests, unauthorized access, and other potential issues.
    #[method(name = "status")]
    async fn status(&self) -> Result<String, ErrorObjectOwned>;
}
