# sol-to-uml
A Rust service for generating Unified Modeling Language (UML) class diagrams for Solidity contracts based on Node.js package
[sol2uml](https://github.com/naddison36/sol2uml).
## Usage
### Running
The service runs in a Docker container, which creates two images when it is built. One of them is intermediate and can be removed.
Use `docker-compose` to build the service.
### Requests
Service request contains JSON with single key `sources`, value for this key is map with **relative file path - file content** pairs.

Example:
```
{
  "sources": {
    "src/test1.sol": "...first contract code..."
    "src/test2.sol": "...second contract code..."
  }
}
```
### Responses
Service response contains JSON with key `uml_diagram`, which value is the bytecode of UML diagram in svg format.
## Testing
For now it is only possible to test service using `cargo test`. For this you need to install [sol2uml](https://github.com/naddison36/sol2uml)
globally on your device as mentioned in the repo instructions. Notice that the current version of `sol2uml` supported by the service is 2.0, so tests
may fail with other versions and service may not work correctly.

For testing on **Windows** you need to rewrite some code due to the way the service is implemented. Change
[44 line in `src/lib.rs`](https://github.com/blockscout/sol-to-uml/blob/main/src/lib.rs#L44) with
`let status = Command::new("cmd").arg("/C").arg("sol2uml")`. `/C` should be replaced with the drive where sol2uml is installed.
