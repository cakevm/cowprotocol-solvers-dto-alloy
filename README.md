# CoW Protocol Solvers DTO Alloy

This crate provides the [solvers-dto](https://github.com/cowprotocol/services/tree/main/crates/solvers-dto) but with the [alloy](https://github.com/alloy-rs/core) type system.

# Motivation
- Replace `web3` with `alloy` types
- Remove custom byte serialization
- Add `Clone` and `Serialize` / `Deserialize` traits to all types
  - Sometimes you like to store a response and for that it is better if you can `Serialize` it again

# Status
The test coverage is currently not great. I do not have access to the auction API because it is not public anymore (I used an old captured file and modified it). Please let me know if there are any issues with the current API.

# Acknowledgements
This project is based on the [solvers-dto](https://github.com/cowprotocol/services/tree/main/crates/solvers-dto) and all models are taken from there. Many thanks to the cowprotocol team for making there service open source. And many thanks to the [alloy-rs](https://github.com/alloy-rs) team.

# License
This project is dual licensed as the original project under the [Apache 2.0](./LICENSE-APACHE) or [MIT](./LICENSE-MIT).