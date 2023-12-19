## XMTP Postal Service (XPS) Gateway

![XPS](xps.png)

This *postal service* gateway implements the XMTP transport for registration, inbox and conversations.

## Quick Start (Development)

- [READ THE DOCS](https://xmtp.github.io/xps-gateway)

### Dev Containers Development

This contract supports containerized development. From Visual Studio Code Dev Containers extension

`Reopen in Container`

or

Command line build using docker

```bash
$ docker build . -t xps-contract:1
```

## Testing (command line)

```bash
$ cargo test
```
