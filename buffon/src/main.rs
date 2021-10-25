use shmem;
use std::mem;

extern crate rand;
use rand::thread_rng;
use rand::Rng;

use std::f64::consts::PI;

fn make_random_value() -> f64 {
    // let rand_max: f64 = 2147483647.0; // as in <stdlib.h>

    let mut rng = thread_rng();
    // let rv: f64 = rng.gen_range(0.0 .. rand_max);
    let rv = rng.gen::<f64>();

    rv
}

fn buffon_laplace_simulate(a: f64, b: f64, l: f64, trial_num: i32) -> i32 {
    let mut hits = 0;

    for _ in 1 .. trial_num {
        //
        // Randomly choose the location of the eye of the needle in
        // [0,0]x[A,B],
        // and the angle the needle makes.
        //
        let x1 = a * make_random_value();
        let y1 = b * make_random_value();
        let angle = 2.0 * PI * make_random_value();
        //
        // Compute the location of the point of the needle.
        //
        let x2 = x1 + l * angle.cos();
        let y2 = y1 + l * angle.sin();
        //
        // Count the end locations that lie outside the cell.
        //
        if x2 <= 0.0 || a <= x2 || y2 <= 0.0 || b <= y2 {
            hits += 1;
        }
    }

    hits
}

fn r8_huge() -> f64 {
    1.0e+30
}

fn main() {
    let a: f64 = 1.0;
    let b: f64 = 1.0;
    let l: f64 = 1.0;

    shmem::init();

    let me = shmem::my_pe();
    let npes = shmem::n_pes();

    let hit_total = shmem::malloc(1 * mem::size_of::<i32>()) as *mut i32;
    let hit_num = shmem::malloc(1 * mem::size_of::<i32>()) as *mut i32;

    let pwrk =
        shmem::malloc(shmem::REDUCE_MIN_WRKDATA_SIZE * mem::size_of::<i32>())
        as *mut i32;

    let psync = shmem::malloc(shmem::SYNC_SIZE * mem::size_of::<i64>())
        as *mut i64;

    unsafe {
        *hit_num = 0;

        for i in 0 .. shmem::BCAST_SYNC_SIZE - 1 {
            *psync.add(i) = shmem::SYNC_VALUE;
        }
    }

    if me == 0 {
        println!();
        println!("BUFFON_LAPLACE - Master process:");
        println!("  Rust version");
        println!();
        println!("  A SHMEM example program to estimate PI");
        println!("  using the Buffon-Laplace needle experiment.");
        println!("  On a grid of cells of  width A and height B,");
        println!("  a needle of length L is dropped at random.");
        println!("  We count the number of times it crosses");
        println!("  at least one grid line, and use this to estimate ");
        println!("  the value of PI.");
        println!();
        println!("  The number of processes is {}", npes);
        println!();
        println!("  Cell width A =    {}", a);
        println!("  Cell height B =   {}", b);
        println!("  Needle length L = {}", l);
        println!();
    }

    // remove rather pointless C++ output from here

    let trial_num = 100000;

    unsafe {
        *hit_num = buffon_laplace_simulate(a, b, l, trial_num);
    }

    shmem::barrier_all();

    shmem::int_sum_to_all(hit_total, hit_num, 1, 0, 0, npes, pwrk, psync);

    if me == 0 {
        let trial_total = trial_num * npes;

        let (pdf_estimate, pi_estimate);

        unsafe {
            pdf_estimate = *hit_total as f64 / trial_total as f64;

            if *hit_total == 0 {
                pi_estimate = r8_huge();
            }
            else {
                pi_estimate = l * (2.0 * (a + b) - l) / (a * b * pdf_estimate);
            }
        }

        let pi_error = (PI - pi_estimate).abs();

        println!();
        println!("{:>8}  {:>8}  {:>16}  {:>16}  {:>16}",
                 "Trials",
                 "Hits",
                 "Estimated PDF",
                 "Estimated Pi",
                 "Error"
        );
        println!();

        unsafe {
            println!("{:>8}  {:>8}  {:>16.5}  {:>16.5}  {:>16.5}",
                     trial_total,
                     *hit_total,
                     pdf_estimate,
                     pi_estimate,
                     pi_error
            );
        }
    }

    if me == 0 {
        println!();
        println!("BUFFON_LAPLACE - Master process:");
        println!("Normal end of execution.");
    }

    shmem::free(pwrk as shmem::SymmMemAddr);
    shmem::free(psync as shmem::SymmMemAddr);

    shmem::finalize();
}
