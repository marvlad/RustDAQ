# RustDAQ

This a HEP's tool for data acquisition (DAQ) using rust language. The idea so far is to use ANNIE's ToolDAQ as starting point written in C++.

So far we have
```
├── Cargo.toml
└── src
    ├── bin
    │   ├── receiver.rs
    │   └── sender.rs
    ├── lib.rs
    └── service_discovery.rs
```
To test let's run
```
cargo build 
cargo run --bin sender
cargo run --bin receiver
```
