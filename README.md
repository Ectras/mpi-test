# mpi_test

Provides MPI testing capabilities to Rust framework. The `mpi_test` proc macro can be used to annotate functions to be tested with MPI, replacing the usual `#[test]`. As argument, the number of processed to spawn needs to be given.

The macro expands the tested function to two functions, one with the suffix `_internal`. Note that this internal method will show up as ignored when running `cargo test`.

## Example
```rust
use mpi::{collective::SystemOperation,traits::{Communicator, Root}};
use mpi_test::mpi_test;

#[mpi_test(4)]
fn test_simple_reduce() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    // Test that the correct number of processes are spawned
    assert_eq!(world.size(), 4);
}
```

## Running in Docker / CI
When running inside a Docker container, `mpiexec` might complain that it doesn't want to run as root. When using OpenMPI, set the flags `OMPI_ALLOW_RUN_AS_ROOT` and `OMPI_ALLOW_RUN_AS_ROOT_CONFIRM` to allow it.