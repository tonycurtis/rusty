use shmem;
use std::mem;

fn main() {
    shmem::init();

    let race_winner = shmem::malloc(1 * mem::size_of::<i32>()) as *mut i32;

    unsafe {
        *race_winner = -1;
    }

    shmem::barrier_all();

    let me = shmem::my_pe();

    let oldval = shmem::int_atomic_compare_swap(race_winner, -1, me, 0);

    if oldval == -1 {
        println!("PE {} was first", me);
    }

    shmem::free( race_winner as shmem::SymmMemAddr);

    shmem::finalize();
}
