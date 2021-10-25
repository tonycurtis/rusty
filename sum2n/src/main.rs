use shmem;
use std::mem;

fn main() {
    shmem::init();

    let counter = shmem::malloc(1 * mem::size_of::<i32>()) as *mut i32;
    unsafe {
	    *counter = 0;
    }

    shmem::barrier_all();

    // counter is now 0 everywhere

    // add me+1 to PE 0
    let me = shmem::my_pe();

    shmem::int_atomic_add(counter, me + 1, 0);

    shmem::barrier_all();

    if me == 0 {
	    let n = shmem::n_pes();

	    unsafe {
	        println!("Sum from 1 to {} = {}", n, *counter);
	    }
    }

    shmem::free(counter as shmem::SymmMemAddr);

    shmem::finalize();
}
