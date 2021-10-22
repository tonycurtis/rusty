// See https://docs.rs/state/0.5.2/state/struct.Storage.html
// Cargo.toml dependency: state = "0.5"

use std::collections::HashMap;
use std::sync::Mutex;
use state::Storage;

//
// map of names -> pointer-addresses
//
static GM: Storage<Mutex<HashMap<usize, &'static str>>> = Storage::new();

fn insert(a: usize, s: &'static str) {
    // pull out inner map; mutable because we're updating
    let mut map = GM.get().lock().unwrap();

    map.insert(a, s);
}

fn remove(a: usize) {
    let mut map = GM.get().lock().unwrap();

    map.remove(&a);             // need to borrow?
}

fn show() {
    println!("{}", "=".repeat(48));

    // pull out inner map; immutable because we're only looking at it
    let map = GM.get().lock().unwrap();

    for (ptr, name) in map.iter() {
        println!("{:p} is {}", ptr, name);
    }
}

fn main() {
    // create the hashmap, and put it in the container
    GM.set(Mutex::new(HashMap::new()));

    // build a couple of raw C pointers
    let xp;
    let yp;

    unsafe {
        xp = libc::malloc(32);
        yp = libc::malloc(16);
    }

    // turn raw pointers into uint64 (in this case) so we can use them
    // as hash keys
    let xi = xp as usize;
    let yi = yp as usize;

    insert(xi, "tony");
    insert(yi, "mary");

    show();

    remove(xi);

    show();

    // clean-up
    unsafe {
        libc::free(xp);
        libc::free(yp);
    }
}
