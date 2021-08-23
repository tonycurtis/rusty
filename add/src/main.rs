use shmem;
use std::mem;
use uname::uname;

fn main() {
    let node = uname().unwrap().nodename;

    shmem::init();

    let me = shmem::my_pe();
    let n = shmem::n_pes();

    let dest = shmem::malloc(1 * mem::size_of::<i32>()) as *mut i32;
    unsafe {
	*dest = 6;
    }
    shmem::barrier_all();
    
    if me == 0 {
	shmem::int_atomic_add(dest, 4, n - 1);
    }

    shmem::barrier_all();

    unsafe {
	println!("{}: PE {:>6} dest = {:>6}", node, me, *dest);
    }

    shmem::free(dest as shmem::SymmMemAddr);

    shmem::finalize();
}
