use std::mem;
use shmem;

fn main() {

    shmem::init();

    // let mut r: shmem::RDMA;

    let me = shmem::my_pe();
    let n = shmem::n_pes();

    let nextpe = (me + 1) % n;

    let dest = shmem::malloc(mem::size_of::<i32>()) as *mut i32;

    shmem::int_p(dest, nextpe, nextpe);

    // let xx = shmem::RdmaOp { dest: dest, src: nextpe, pe: nextpe };
    // shmem::p(xx);

    shmem::barrier_all();

    // raw pointer deref
    unsafe {
        print!("{}: got {}", me, *dest);
        if *dest == me {
            println!("  CORRECT");
        }
        else {
            println!("  WRONG, expected {}", me);
        }
    }

    shmem::free(dest as shmem::SymmMemAddr);
    shmem::finalize();
}
