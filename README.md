![example workflow](https://github.com/Ectras/mpi_test/actions/workflows/test.yml/badge.svg)

# mpi_test

Provides MPI testing capabilities to Rust. The `mpi_test` proc macro can be used to annotate functions to be tested with MPI, replacing the usual `#[test]`. As an argument, the number of processes to spawn needs to be given.

The macro expands the tested function to two functions, one with the suffix `_internal`. Note that this internal method will show up as ignored when running `cargo test`.

> [!WARNING]
> This crate is a cheap workaround: It creates a second test that uses [std::process::Command](https://doc.rust-lang.org/std/process/struct.Command.html) to launch the test binary with `mpiexec`.

## Example
```rust
use mpi::{collective::SystemOperation, traits::{Communicator, Root}};
use mpi_test::mpi_test;

#[mpi_test(4)]
fn test_simple_reduce() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    // Test that the correct number of processes are spawned
    assert_eq!(world.size(), 4);
}
```

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed under the terms of both the Apache License, Version 2.0 and the MIT license without any additional terms or conditions.
