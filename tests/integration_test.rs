use mpi::{
    collective::SystemOperation,
    traits::{Communicator, Root},
};
use mpi_test::mpi_test;

#[mpi_test(4)]
fn test_simple_reduce() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    // Test that the correct number of processes are spawned
    assert_eq!(world.size(), 4);

    // Do a simple sum reduction to test that the processes are working
    let rank = world.rank();
    let root_process = world.process_at_rank(0);
    let op = SystemOperation::sum();
    let buffer = [rank];
    if rank != 0 {
        root_process.reduce_into(&buffer, op);
    } else {
        let mut result = [0];
        root_process.reduce_into_root(&buffer, &mut result, op);
        assert_eq!(result[0], 0 + 1 + 2 + 3);
    }
}
