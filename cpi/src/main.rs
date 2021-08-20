use std::f64::consts::PI;
use std::mem;
use uname::uname;
// use std::cmp;
use shmem;

pub const N: i64 = 10000;

fn f(a: f64) -> f64 {
    4.0 / (1.0 + a * a)
}

fn main() {
    let node = uname().unwrap().nodename;

    shmem::init();
    let me = shmem::my_pe();
    let npes = shmem::n_pes();

    let pi = shmem::malloc(mem::size_of::<f64>()) as *mut f64;

    let h: f64 = 1.0 / N as f64;
    let mut sum: f64 = 0.0;

    for i in (me + 1..N as i32).step_by(npes as usize) {
        let x = h * ((i as f64) - 0.5);

        sum += f(x);
    }

    let mypi = h * sum;

    shmem::barrier_all();

    // shmem::double_sum_to_all(pi, mypi, 1, 0, 0, npes);

    unsafe {
        *pi = mypi * npes as f64; // fudge
    }

    shmem::barrier_all();

    unsafe {
        println!("PE {}/{} on \"{}\" pi = {}", me, npes, node, *pi);
    }

    shmem::free(pi as shmem::SymmMemAddr);
    shmem::finalize();
}
