# cargo-next

[![Maintenance](https://img.shields.io/badge/maintenance-actively%20maintained-brightgreen.svg)](https://github.com/conventional-commits-rs/cargo-next)
[![crates.io](https://img.shields.io/crates/v/cargo-next.svg)](https://crates.io/crates/cargo-next)
[![crates.io](https://img.shields.io/crates/d/cargo-next)](https://crates.io/crates/cargo-next)
[![Documentation](https://docs.rs/cargo-next/badge.svg)](https://docs.rs/cargo-next)

> A cargo subcommand to set the next version of a crate.

## Installation

```text
cargo install cargo-next --locked
```

## Usage

### Binary

```text
$ cargo next --minor
$ cargo next 0.1.5
$ ./emits-new-version.sh | cargo next
```

### Library

```rust
use cargo_next::{bump_version, get_version, set_version, SemVer};

let path_to_toml = ...;

// Bump the version by a semver component.
let _res = bump_version(&path_to_toml, SemVer::Minor);
// Set the version directly.
let _res = set_version(&path_to_toml, "0.1.2");
// Or get the version of a crate.
let _res = get_version(&path_to_toml);
```

#### License

<sup>
Licensed under either of <a href="license-apache">Apache License, Version
2.0</a> or <a href="license-mit">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
