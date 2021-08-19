use uname::uname;
use std::mem;
use shmem;

fn main() {
    let node = uname().unwrap().nodename;

    shmem::init();

    // let mut r: shmem::RDMA;

    let me = shmem::my_pe();
    let n = shmem::n_pes();

    let nextpe = (me + 1) % n;

    // how to wrap this?
    let dest = shmem::malloc(1 * mem::size_of::<i32>()) as *mut i32;

    shmem::int_p(dest, nextpe, nextpe);

    // let xx = shmem::RdmaOp { dest: dest, src: nextpe, pe: nextpe };
    // shmem::p(xx);

    shmem::barrier_all();

    // raw pointer deref
    unsafe {
        print!("{}: {:>6}: got {:>6}", node, me, *dest);
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
