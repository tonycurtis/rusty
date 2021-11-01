use std::f64::consts::PI;
use std::mem;
use shmem;
use std::time::Instant;
use std::env;

// default no. of iterations
pub const N: i64 = 10000;

fn f(a: f64) -> f64 {
    4.0 / (1.0 + a * a)
}

fn main() {
    let argv: Vec<String> = env::args().collect();

    let niters: i64 = if argv.len() > 1 {
        argv[1].parse().unwrap()
    } else {
        N
    };

    shmem::init();
    let me = shmem::my_pe();
    let npes = shmem::n_pes();

    let start_time = Instant::now();

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

    let h: f64 = 1.0 / niters as f64;
    let mut sum: f64 = 0.0;

    for i in (me + 1..niters as i32).step_by(npes as usize) {
        let x = h * ((i as f64) - 0.5);

        sum += f(x);
    }

    unsafe {
        *mypi = h * sum;
    }

    shmem::barrier_all();

    shmem::double_sum_to_all(pi, mypi, 1, 0, 0, npes, pwrk, psync);

    if me == 0 {
        unsafe {
            println!("pi = {}, epsilon = {}, in {:?}",
                     *pi, (*pi - PI).abs(), start_time.elapsed());
        }
    }

    shmem::free(psync as shmem::SymmMemAddr);
    shmem::free(pwrk as shmem::SymmMemAddr);
    shmem::free(pi as shmem::SymmMemAddr);

    shmem::finalize();
}
