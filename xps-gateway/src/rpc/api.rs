//! Trait Interface Definitions for XPS JSON-RPC

use ethers::core::types::Signature;
use ethers::prelude::*;
use jsonrpsee::{proc_macros::rpc, types::ErrorObjectOwned};

use gateway_types::Message;
use gateway_types::{GrantInstallationResult, KeyPackageResult};
use lib_didethresolver::types::XmtpAttribute;

/// XPS JSON-RPC Interface Methods
#[rpc(server, client, namespace = "xps")]
pub trait Xps {
    // Placeholder for send_message, see [the discussion](https://github.com/xmtp/xps-gateway/discussions/11)
    #[method(name = "sendMessage")]
    async fn send_message(&self, _message: Message) -> Result<(), ErrorObjectOwned>;

    /// # Documentation for JSON RPC Endpoint: `grantInstallation`
    ///
    /// ## Overview
    ///
    /// The `grantInstallation` method is used to register an installation on the network and associate the installation with a concrete identity.
    ///
    /// ## JSON RPC Endpoint Specification
    ///
    /// ### Method Name
    /// `grantInstallation`
    ///
    /// ### Request Parameters
    /// did: string
    /// name: String,
    /// value: String,
    /// signature: Signature,
    ///
    /// ### Request Format
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "method": "status",
    ///   "id": 1
    /// }
    /// ```

    /// - `jsonrpc`: Specifies the version of the JSON RPC protocol being used. Always "2.0".
    /// - `method`: The name of the method being called. Here it is "grantInstallation".
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
    ///
    /// - `result`: Contains data related to the success of the operation. The nature of this data can vary based on the implementation.
    ///
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
    ///
    /// - `error`: An object containing details about the error.
    ///   - `code`: A numeric error code.
    ///   - `message`: A human-readable string describing the error.
    ///
    /// ### Example Usage
    ///
    /// #### Request
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "method": "status",
    ///   "id": 42
    /// }
    /// ```
    ///
    /// #### Response
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "result": "OK",
    ///   "id": 42
    /// }
    /// ```
    ///
    /// ### Command Line Example
    /// ```bash
    /// $ $ curl -H "Content-Type: application/json" -d '{"id":7000, "jsonrpc":"2.0", "method":"xps_status"}' http:///localhost:34695
    /// {"jsonrpc":"2.0","result":"OK","id":7000}
    /// ```
    ///
    /// ### Notes
    /// - The system should have proper error handling to deal with invalid requests, unauthorized access, and other potential issues.
    #[method(name = "grantInstallation")]
    async fn grant_installation(
        &self,
        did: String,
        name: XmtpAttribute,
        value: Vec<u8>,
        signature: Signature,
    ) -> Result<GrantInstallationResult, ErrorObjectOwned>;

    /// # Documentation for JSON RPC Endpoint: `revoke_installation`
    ///
    /// ## JSON RPC Endpoint Specification
    ///
    /// #### Request:
    ///
    /// - **Method:** `POST`
    /// - **URL:** `/rpc/v1/revokeInstallation`
    /// - **Headers:**
    ///   - `Content-Type: application/json`
    /// - **Body:**
    ///   - **JSON Object:**
    ///     - `jsonrpc`: `"2.0"`
    ///     - `method`: `"revokeInstallation"`
    ///     - `params`: Array (optional parameters as required)
    ///     - `id`: Request identifier (integer or string)
    ///
    /// ### Endpoint: `revokeInstallation`
    ///
    /// #### Description
    /// The `revokeInstallation` endpoint is responsible for removing the contact bundle for the XMTP device installation.   The request must be made to a valid did with an XMTP profile.
    ///
    /// #### Request
    /// The request for this endpoint should contain the necessary information to authenticate and validate the installation request including the wallet signed payload
    ///
    /// ##### Parameters:
    /// - `DID` (string): Unique XMTP identifier for the user requesting the revocation.
    /// - `name` (string): Unique identifier naming bundled contents variant.
    /// - `value` (bytes): Installation bundle bytes payload
    /// - `V` (int): signature V
    /// - `R` (bytes): signature R
    /// - `S` (bytes): signature S
    ///
    /// ##### Example Request:
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "method": "revokeInstallation",
    ///   "params": {
    ///     "did": "12345",
    ///     "name": "xmtp/contact_installation",
    ///     "value": "#######",
    ///     "signature": {
    ///       "V": "12345",
    ///       "R": "valueR",
    ///       "S": "valueS"
    ///     }
    ///   },
    ///   "id": 1
    /// }
    /// ```
    ///
    /// #### Response
    /// The response will indicate whether the installation is revoked and may include additional information or instructions.
    ///
    /// ##### Result Fields:
    /// - `status` (string): The status of the request, e.g., 'completed'.
    /// - `message` (string, optional): Additional information or reason for the decision.
    /// - `tx` (string, optional): transaction receipt
    ///
    /// ##### Example Response:
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "result": {
    ///     "status": "completed",
    ///     "message": "Installation revoked.",
    ///     "tx": "<receipt>"
    ///   },
    ///   "id": 1
    /// }
    /// ```
    ///
    /// #### Error Handling
    /// In case of an error, the response will include an error object with details.
    ///
    /// ##### Error Object Fields:
    /// - `code` (integer): Numeric code representing the error type.
    /// - `message` (string): Description of the error.
    ///
    /// ##### Example Error Response:
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "error": {
    ///     "code": 403,
    ///     "message": "User not authorized for installation."
    ///   },
    ///   "id": 1
    /// }
    /// ```

    /// removes the contact bundle for the XMTP device installation. Request must be made to a
    /// valid DID with an XMTP profile.
    ///
    /// # Arguments
    ///
    /// * `did` - the DID of the XMTP device installation
    /// * `name` - the name of the contact bundle variant
    /// * `value` - the value of the contact bundle
    /// * `signature` - the signature of the contact bundle
    #[method(name = "revokeInstallation")]
    async fn revoke_installation(
        &self,
        did: String,
        name: XmtpAttribute,
        value: Vec<u8>,
        signature: Signature,
    ) -> Result<(), ErrorObjectOwned>;

    /// ## JSON-RPC Endpoint Documentation
    ///
    /// #### Request:
    ///
    /// - **Method:** `POST`
    /// - **URL:** `/rpc/v1/fetchKeyPackages`
    /// - **Headers:**
    ///   - `Content-Type: application/json`
    /// - **Body:**
    ///   - **JSON Object:**
    ///     - `jsonrpc`: `"2.0"`
    ///     - `method`: `"fetchKeyPackages"`
    ///     - `params`: Array (optional parameters as required)
    ///     - `id`: Request identifier (integer or string)
    ///
    /// ### Endpoint: `fetchKeyPackages`
    ///
    /// #### Description
    ///
    /// The `fetchKeyPackages` endpoint is responsible for retrieving the contact bundle for the XMTP device installations. The request must be made to a valid did with an XMTP profile.
    ///
    /// #### Request
    ///
    /// The request for this endpoint should contain a valid DID. All returned information is public.
    ///
    /// ##### Parameters:
    ///
    /// -   `DID` (string): Unique XMTP identifier for the user requesting the installation.
    ///
    /// ##### Example Request:
    ///
    /// ```json
    /// {
    ///     "jsonrpc": "2.0",
    ///     "method": "fetchKeyPackages",
    ///     "params": {
    ///         "did": "12345"
    ///     },
    ///     "id": 1
    /// }
    /// ```
    ///
    /// #### Response
    ///
    /// The response will provide an optionally empty list of installation bundles.
    ///
    /// ##### Result Fields:
    ///
    /// -   `status` (string): The status of the request, e.g., 'success'.
    /// -   `installation` (array): Array of installation bundles.
    ///
    /// ##### Example Response:
    ///
    /// ```json
    /// {
    ///     "jsonrpc": "2.0",
    ///     "result": {
    ///         "status": "success",
    ///         "installation": ["bundle1...", "bundle2..."]
    ///     },
    ///     "id": 1
    /// }
    /// ```
    ///
    /// #### Error Handling
    ///
    /// In case of an error, the response will include an error object with details.
    ///
    /// ##### Error Object Fields:
    ///
    /// -   `code` (integer): Numeric code representing the error type.
    /// -   `message` (string): Description of the error.
    ///
    /// ##### Example Error Response:
    ///
    /// ```json
    /// {
    ///     "jsonrpc": "2.0",
    ///     "error": {
    ///         "code": 403,
    ///         "message": "User not authorized for installation."
    ///     },
    ///     "id": 1
    /// }
    /// ```
    #[method(name = "fetchKeyPackages")]
    async fn fetch_key_packages(&self, did: String) -> Result<KeyPackageResult, ErrorObjectOwned>;

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
    ///
    /// ### Notes
    /// - The system should have proper error handling to deal with invalid requests, unauthorized access, and other potential issues.
    #[method(name = "status")]
    async fn status(&self) -> Result<String, ErrorObjectOwned>;

    #[method(name = "walletAddress")]
    async fn wallet_address(&self) -> Result<Address, ErrorObjectOwned>;
}
