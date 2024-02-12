//! Trait Interface Definitions for XPS JSON-RPC

use ethers::core::types::Signature;
use ethers::prelude::*;
use jsonrpsee::{proc_macros::rpc, types::ErrorObjectOwned};

use gateway_types::{GrantInstallationResult, KeyPackageResult, WalletBalance};
use gateway_types::{Message, SendMessageResult};
use lib_didethresolver::types::XmtpAttribute;

/// XPS JSON-RPC Interface Methods
#[rpc(server, client, namespace = "xps")]
pub trait Xps {
    /// # Documentation for JSON RPC Endpoint: `sendGroupMessage`
    ///
    /// ## Overview
    ///
    /// The `sendGroupMessage` method is used to send a message within a specified conversation. It is an external function that is part of a larger system managing communications between users or entities. This method requires two parameters: a unique identifier for the conversation (`conversationId`) and the message content (`payload`).
    ///
    /// ## JSON RPC Endpoint Specification
    ///
    /// ### Request:
    ///
    /// - **Method:** `POST`
    /// - **URL:** `/rpc/v1/sendGroupMessage`
    /// - **Headers:**
    ///   - `Content-Type: application/json`
    /// - **Body:**
    ///   - **JSON Object:**
    ///     - `jsonrpc`: `"2.0"`
    ///     - `method`: `"sendGroupMessage"`
    ///     - `params`: Array (optional parameters as required)
    ///     - `id`: Request identifier (integer or string)
    ///
    /// ### Method Name
    /// `sendGroupMessage`
    ///
    /// ### Request Parameters
    /// 1. `conversationId`: A unique identifier for the conversation. This is a 32-byte string, typically in hexadecimal format.
    /// 2. `payload`: The content of the message. This is a variable-length byte array that can hold any form of data, such as text, images, or other binary formats.
    /// 3. `identity`: Message sender address.
    /// 4. `sigV`: The signature V
    /// 5. `sigR`: The signature R
    /// 6. `sigS`: The signature S
    ///
    /// ### Request Format
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "method": "sendGroupMessage",
    ///   "params": {
    ///     "conversationId": "<conversationId>",
    ///     "payload": "<payload>",
    ///     "identity": "<identity>",
    ///      "V": "<V>",
    ///      "R": "<R>",
    ///      "S": "<S>"
    ///   },
    ///   "id": 1
    /// }
    /// ```
    ///
    /// - `jsonrpc`: Specifies the version of the JSON RPC protocol being used. Always "2.0".
    /// - `method`: The name of the method being called. Here it is "sendMessage".
    /// - `params`: A structured value holding the parameters necessary for the method. It contains:
    ///   - `conversationId`: The unique identifier for the conversation.
    ///   - `payload`: The message content in bytes.
    ///   - `identity`: The identity of the sender.
    ///   - `V`: The signature V
    ///   - `R`: The signature R
    ///   - `S`: The signature S
    /// - `id`: A unique identifier established by the client that must be number or string. Used for correlating the response with the request.
    ///
    /// ### Response Format
    /// The response will typically include the result of the operation or an error if the operation was unsuccessful.
    ///
    /// #### Success Response
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "result": "status",
    ///   "tx": "<tx receipt>",
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
    /// - `code`: A numeric error code.
    /// - `message`: A human-readable string describing the error.
    ///
    /// ### Example Usage
    ///
    /// #### Request
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "method": "sendGroupMessage",
    ///   "params": {
    ///     "conversationId": "0x1234abcd...",
    ///     "payload": "SGVsbG8sIHdvcmxkIQ==",
    ///     "identity": "0xAddress",
    ///     "V": "####",
    ///     "R": "#####",
    ///     "S": "#####"
    ///   },
    ///   "id": 42
    /// }
    /// ```
    ///
    /// #### Response
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "result": "Message sent successfully",
    ///   "tx": "<tx receipt>",
    ///   "id": 42
    /// }
    /// ```
    #[method(name = "sendMessage")]
    async fn send_message(&self, _message: Message) -> Result<SendMessageResult, ErrorObjectOwned>;

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

    /// ### Documentation for JSON RPC Endpoint: `balance`
    /// ---
    /// #### Endpoint Name: `balance`
    /// #### Description:
    /// The `balance` endpoint retrieves the current balance of the internal wallet managed by the server. This endpoint is essential for applications that need to display or monitor the wallet's balance, especially in the context of cryptocurrency transactions or account management.
    /// #### Request:
    /// - **Method:** `POST`
    /// - **URL:** `/rpc/v1/balance`
    /// - **Headers:**
    /// - `Content-Type: application/json`
    /// - **Body:**
    /// - **JSON Object:**
    ///     - `jsonrpc`: `"2.0"`
    ///     - `method`: `"balance"`
    ///     - `params`: Array (optional parameters as required)
    ///     - `id`: Request identifier (integer or string)
    /// **Example Request Body:**
    /// ```json
    /// {
    /// "jsonrpc": "2.0",
    /// "method": "balance",
    /// "params": [],
    /// "id": 1
    /// }
    /// ```
    /// #### Response:
    /// - **Success Status Code:** `200 OK`
    /// - **Error Status Codes:**
    /// - `400 Bad Request` - Invalid request format or parameters.
    /// - `500 Internal Server Error` - Server or wallet-related error.
    /// **Success Response Body:**
    /// ```json
    /// {
    /// "jsonrpc": "2.0",
    /// "result": {
    ///     "balance": "100.0 ETH",
    ///     "unit": "ETH"
    /// },
    /// "id": 1
    /// }
    /// ```
    /// **Error Response Body:**
    /// ```json
    /// {
    /// "jsonrpc": "2.0",
    /// "error": {
    ///     "code": -32602,
    ///     "message": "Invalid parameters"
    /// },
    /// "id": 1
    /// }
    /// ```
    /// #### Error Handling:
    /// - **Invalid Parameters:** Check if the request body is properly formatted and includes valid parameters.
    /// - **Wallet or Server Errors:** Ensure that the server and wallet are operational. Consult server logs for detailed error information.
    /// #### Security Considerations:
    /// - **Authentication and Authorization:** Implement robust authentication and authorization checks to ensure only authorized users can access wallet balance information.
    /// - **Secure Communication:** Utilize HTTPS to encrypt data in transit and prevent eavesdropping.
    /// #### Usage Example:
    /// ```javascript
    /// const requestBody = {
    /// jsonrpc: "2.0",
    /// method: "balance",
    /// params: [],
    /// id: 1
    /// };
    /// fetch('https://server.example.com/rpc/v1/balance', {
    /// method: 'POST',
    /// headers: {
    ///     'Content-Type': 'application/json'
    /// },
    /// body: JSON.stringify(requestBody)
    /// })
    /// .then(response => response.json())
    /// .then(data => console.log('Wallet Balance:', data.result))
    /// .catch(error => console.error('Error:', error));
    /// ```
    /// </div>
    /// ```
    #[method(name = "balance")]
    async fn balance(&self) -> Result<WalletBalance, ErrorObjectOwned>;
}
