# xps-cli

### Tool to use XPS from the command line


Install Rust & clone this repository:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && git clone --branch insipx/xps-signed-tx https://github.com/xmtp/xps-gateway
```

```bash
Usage: xps-cli [-n <network>] [-c <contract>] [-w <wallet>] [-g <gateway>] <command> [<args>]

A simple CLI to interact with the EVM-Based DID Registries

Options:
  -n, --network     the HTTPS network RPC-URL to interact with By-default,
                    `https://ethereum-sepolia.publicnode.com`
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
