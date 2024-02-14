# xps-cli

### Tool to use XPS from the command line


##### Install Rust & clone this repository:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && git clone --branch insipx/xps-signed-tx https://github.com/xmtp/xps-gateway
```

##### Navigate to the project directory and build the gateway
```bash
cd xps-gateway && cargo build --release
```

##### Build the CLI
```bash
cargo build --release --bin xps-cli
```

##### Run the gateway in a separate terminal window
```bash
./target/release/xps --port 9944
```

##### Install ethereum to make use of the `ethkey` cli tool
```bash
brew install ethereum
```

##### Generate a JSON wallet ethereum wallet
FILE indicates a file containing the password for the wallet. A file is required on MacOS.
```bash
ethkey generate --passwordfile FILE --json
```

#### Run the xps-cli tool

##### Check gateway information with the `info` flag
```bash
./target/release/xps-cli info
```
Make sure the gateway has some funds on Sepolia Eth

##### Set an attribute
```bash
./target/release/xps-cli -w ./keyfile.json -g ws://localhost:9944 grant -v 0x0000000b7373682d6564323535313900000020b500aebaf8a22c7aa7f6d28deb972a2d9cad5cecb92a92628cfffd824cb13968
```

##### Revoke an Attribute
```bash
./target/release/xps-cli -w ./keyfile.json -g ws://localhost:9944 revoke -v 0x0000000b7373682d6564323535313900000020b500aebaf8a22c7aa7f6d28deb972a2d9cad5cecb92a92628cfffd824cb13968
```

##### Help Flag
use the `--help` flag to get more information about the CLI, and options to set
```bash
./target/release/xps-cli --help
```

```bash
Usage: xps-cli [-n <network>] [-c <contract>] [-w <wallet>] [-g <gateway>] <command> [<args>]

A simple CLI to interact with the EVM-Based DID Registries

Options:
  -n, --network     the HTTPS network RPC-URL to interact with
                    (default: `https://ethereum-sepolia.publicnode.com`)
  -c, --contract    address of the DID Registry contract to interact
                    with(default: Test deployment on Sepolia)
  -w, --wallet      path to a local JSON wallet. Ensure usage of a test wallet,
                    the security of this binary has not been verified. Use at
                    your own risk. (default: `./wallet.json`)
  -g, --gateway     URL of the XPS gateway. Default `ws://localhost:9944`
  --help            display usage information

Commands:
  grant             The Grant SubCommand
  revoke            The Revoke SubCommand
  info              Get information about the gateway, like wallet address and
                    current balance.
```
