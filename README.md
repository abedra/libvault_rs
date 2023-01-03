[![Test Status](https://github.com/abedra/libvault_rs/workflows/tests/badge.svg?event=push)](https://github.com/abedra/libvault_rs/actions)
[![Crate](https://img.shields.io/crates/v/libvault_rs.svg)](https://crates.io/crates/libvault_rs)
[![API](https://docs.rs/libvault_rs/badge.svg)](https://docs.rs/libvault_rs)

# libvault_rs

A Rust library for [Hashicorp Vault](https://www.vaultproject.io/)

## Usage

Add this to your `Cargo.toml`

```toml
[dependencies]
libvault_rs = "0.0.4"
```

## Feature Support

The following tables show support for each of the secret backends, auth methods, and system endpoints. Because the surface area is so large, endpoints are implemented as needed. Pull requests are welcome. Feel free to file an issue or submit a pull request for additional support.

| Secret Backend   | Implemented | Example |
|------------------|-------------|-------- |
| Key/Value V1     | 🚧         | ✅     |
| Key/Value V2     | 🚧         | ✅     |

| Auth Method       | Implemented | Example |
|-------------------|-------------|---------|
| AppRole           | 🚧         | ✅      |

| System Backend              | Implemented | Example |
|-----------------------------|-------------|---------|
| /sys/health                 | 🚧         | ⛔      |

\* Requires an enterprise license and cannot be integration tested
