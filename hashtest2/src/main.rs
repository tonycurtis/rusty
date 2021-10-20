// See https://docs.rs/state/0.5.2/state/struct.Storage.html
// Cargo.toml dependency: state = "0.5"

use std::collections::HashMap;
use std::sync::Mutex;
use state::Storage;
use libc::c_void;

// wrap the raw C pointer in a struct, then we can implement Send/Sync
// on it (SA == Symmetric Address)
//

struct SA {
    p: *mut c_void
}

unsafe impl Send for SA {}
unsafe impl Sync for SA {}

//
// map of names -> pointer-addresses
//
static GM: Storage<Mutex<HashMap<&'static str, SA>>> = Storage::new();

fn insert(s: &'static str, a: SA) {
    // pull out inner map; mutable because we're updating
    let mut map = GM.get().lock().unwrap();

    map.insert(s, a);
}

fn remove(s: &'static str) {
    let mut map = GM.get().lock().unwrap();

    map.remove(s);
}

fn show() {
    println!("{}", "=".repeat(48));

    // pull out inner map; immutable because we're only looking at it
    let map = GM.get().lock().unwrap();

    for (name, ptr) in map.iter() {
        println!("{} is {:p}", name, ptr.p); // unpack SymmetricAddress
    }
}

fn main() {
    // create the hashmap, and put it in the container
    GM.set(Mutex::new(HashMap::new()));

    let xp;
    let yp;

    unsafe {
        // just to fake a couple of raw C pointers
        xp = libc::malloc(32);
        yp = libc::malloc(16);
    }

    insert("tony", SA { p: xp });
    insert("mary", SA { p: yp });

    show();

    remove("tony");

    show();

    unsafe {
        // clean-up
        libc::free(xp);
        libc::free(yp);
    }
}
