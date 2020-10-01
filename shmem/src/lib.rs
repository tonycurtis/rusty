use shmemlib;
use std::string::String;

// pass through, will have to look at parsing "pub const" decls.

pub const MAJOR_VERSION: u32 = shmemlib::SHMEM_MAJOR_VERSION;
pub const MINOR_VERSION: u32 = shmemlib::SHMEM_MINOR_VERSION;
pub const VENDOR_STRING: &'static [u8; 9usize] = shmemlib::SHMEM_VENDOR_STRING;

pub type SymmMemAddr = *mut libc::c_void;

//
// == initialize and finalize ============================================
//

pub fn init() {
    unsafe {
        shmemlib::shmem_init();
    }
}

pub fn finalize() {
    unsafe {
        shmemlib::shmem_finalize();
    }
}

//
// == Library query ======================================================
//

pub fn info_get_version() -> (u32, u32) {
    let mut a : i32 = 0;
    let mut b : i32 = 0;

    unsafe {
        shmemlib::shmem_info_get_version(&mut a, &mut b);
    }

    (a as u32, b as u32)
}

pub fn info_get_name() -> String {
    // unpack into UFT vector
    let vvs = shmemlib::SHMEM_VENDOR_STRING.to_vec();

    // turn UTF vector into string
    String::from_utf8(vvs).unwrap()
}

//
// == Global ranks =======================================================
//

pub fn my_pe() -> i32 {
    unsafe {
        return shmemlib::shmem_my_pe();
    }
}

pub fn n_pes() -> i32 {
    unsafe {
        return shmemlib::shmem_n_pes();
    }
}

pub fn pe_accessible(pe: i32) -> bool {
    unsafe {
        shmemlib::shmem_pe_accessible(pe) == 1
    }
}

pub fn addr_accessible(addr: SymmMemAddr, pe: i32) -> bool {
    unsafe {
        shmemlib::shmem_addr_accessible(addr, pe) == 1
    }
}

//
// == puts and gets ======================================================
//

pub fn int_p(dest: *mut i32, src: i32, pe: i32) {
    unsafe {
        shmemlib::shmem_int_p(dest, src, pe);
    }
}
pub fn int_put(dest: *mut i32, src: *const i32, n: u64, pe: i32) {
    unsafe {
        shmemlib::shmem_int_put(dest, src, n, pe);
    }
}

pub fn float_p(dest: *mut f32, src: f32, pe: i32) {
    unsafe {
        shmemlib::shmem_float_p(dest, src, pe);
    }
}
pub fn float_put(dest: *mut f32, src: *const f32, n: u64, pe: i32) {
    unsafe {
        shmemlib::shmem_float_put(dest, src, n, pe);
    }
}

// etc.

pub fn int_get(dest: *mut i32, src: *const i32, n: u64, pe: i32) {
    unsafe {
        shmemlib::shmem_int_get(dest, src, n, pe);
    }
}

// etc.

pub trait RDMAApi<T> {
    fn p(&self, dest: *mut T, src: T, pe: i32);
}

impl dyn RDMAApi<i32> {
    fn p(&self, dest: *mut i32, src: i32, pe: i32) {
        unsafe {
            shmemlib::shmem_int_p(dest, src, pe);
        }
    }
}

//
// == memory management ==================================================
//

// so `sizeof` gives us `usize` as the amount of memory to allocate.
// this doesn't match the type that bindgen dumped out for us, so we
// have to convert

pub fn malloc(n: usize) -> SymmMemAddr {
    unsafe {
        shmemlib::shmem_malloc(n as u64)
    }
}

pub fn calloc(n: u64, s: usize) -> SymmMemAddr {
    unsafe {
        shmemlib::shmem_calloc(n, s as u64)
    }
}

pub fn realloc(m: SymmMemAddr, n: usize) -> SymmMemAddr {
    unsafe {
        shmemlib::shmem_realloc(m, n as u64)
    }
}

pub fn align(a: u64, n: usize) -> SymmMemAddr  {
    unsafe {
        shmemlib::shmem_align(a, n as u64)
    }
}

pub fn free(m: SymmMemAddr) {
    unsafe {
        shmemlib::shmem_free(m);
    }
}

pub fn malloc_with_hints(n: usize, h: u64) -> SymmMemAddr {
    unsafe {
        shmemlib::shmem_malloc_with_hints(n as u64, h as i64)
    }
}

pub fn ptr(m: SymmMemAddr, pe: i32) -> SymmMemAddr {
    unsafe {
        shmemlib::shmem_ptr(m, pe)
    }
}

//
// == ordering and completion ============================================
//

pub fn fence() {
    unsafe {
        shmemlib::shmem_fence();
    }
}

pub fn quiet() {
    unsafe {
        shmemlib::shmem_quiet();
    }
}

pub fn barrier_all() {
    unsafe {
        shmemlib::shmem_barrier_all();
    }
}

//
// == ====================================================================
//
