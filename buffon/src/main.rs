// WORK IN PROGRESS

use shmem;
// use std::mem;

extern crate rand;
use rand::thread_rng;
use rand::Rng;

fn make_random_value() -> f64 {
    let rand_max: u64 = 2147483647; // as in <stdlib.h>

    let mut rng = thread_rng();
    let rv: f64 = rng.gen_range(0.0 .. rand_max as f64);

    rv
}

fn main() {
    let a: f64 = 1.0;
    let b: f64 = 1.0;
    let l: f64 = 1.0;

    shmem::init();

    let me = shmem::my_pe();
    let n = shmem::n_pes();

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
        println!("  The number of processes is {}", n);
        println!();
        println!("  Cell width A =    {}", a);
        println!("  Cell height B =   {}", b);
        println!("  Needle length L = {}", l);
    }

    shmem::barrier_all();

    let random_value = make_random_value();

    println!("Random value = {}", random_value);

    shmem::finalize();
}
