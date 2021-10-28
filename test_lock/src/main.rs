use shmem;
use uname::uname;
use std::mem;
extern crate getopts;
use getopts::Options;
use std::env;

fn main() {
    let node = uname().unwrap().nodename;
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("t", "test", "Use test_lock instead of set_lock");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(e) => { panic!("{}", e.to_string()) }
    };

    shmem::init();

    let me = shmem::my_pe();

    let count = shmem::malloc(mem::size_of::<i32>()) as *mut i32;

    if me == 0 {
        eprintln!("Using dynamic lock");
    }
    let lock = shmem::malloc(mem::size_of::<shmem::ShmemLock>())
        as shmem::ShmemLock;
    unsafe {
        *lock = 0;
    }
    shmem::barrier_all();

    //

    if matches.opt_present("t") {
        if me == 0 { eprintln!("Using test lock"); }
        while shmem::test_lock(lock) {};
    }
    else {
        if me == 0 { eprintln!("Using set lock"); }
        shmem::set_lock(lock);
    }

    let val = shmem::int_g(count, 0);

    println!("{:>8}: {:>4}: count is {}", node, me, val);

    shmem::int_p(count, val + 1, 0);

    shmem::clear_lock(lock);

    shmem::free(lock as shmem::SymmMemAddr);
    shmem::free(count as shmem::SymmMemAddr);

    shmem::finalize();
}
