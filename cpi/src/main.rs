use std::f64::consts::PI;
use std::mem;
// use std::cmp;
use shmem;

pub const N: i64 = 10000;

fn f(a: f64) -> f64 {
    4.0 / (1.0 + a * a)
}

fn main() {
    shmem::init();
    let me = shmem::my_pe();
    let npes = shmem::n_pes();

    let pi = shmem::malloc(mem::size_of::<f64>()) as *mut f64;
    let mypi = shmem::malloc(mem::size_of::<f64>()) as *mut f64;

    let pwrk =
        shmem::malloc(shmem::REDUCE_MIN_WRKDATA_SIZE * mem::size_of::<f64>())
        as *mut f64;

    let psync = shmem::malloc(shmem::SYNC_SIZE * mem::size_of::<i64>())
        as *mut i64;

    unsafe {
        for i in 0 .. shmem::BCAST_SYNC_SIZE - 1 {
            *psync.add(i) = shmem::SYNC_VALUE;
        }
    }

    let h: f64 = 1.0 / N as f64;
    let mut sum: f64 = 0.0;

    for i in (me + 1..N as i32).step_by(npes as usize) {
        let x = h * ((i as f64) - 0.5);

        sum += f(x);
    }

    unsafe {
        *mypi = h * sum;
    }

    shmem::barrier_all();

    shmem::double_sum_to_all(pi, mypi, 1, 0, 0, npes, pwrk, psync);

    unsafe {
        *pi = *mypi * npes as f64; // fudge
    }

    if me == 0 {
        unsafe {
            println!("pi = {}, epsilon = {}", *pi, (*pi - PI).abs());
        }
    }

    shmem::free(psync as shmem::SymmMemAddr);
    shmem::free(pwrk as shmem::SymmMemAddr);
    shmem::free(pi as shmem::SymmMemAddr);

    shmem::finalize();
}
