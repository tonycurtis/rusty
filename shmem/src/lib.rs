use shmemlib;
use std::string::String;
// use std::mem;

// pass through, will have to look at parsing "pub const" decls.

pub const MAJOR_VERSION: u32 = shmemlib::SHMEM_MAJOR_VERSION;
pub const MINOR_VERSION: u32 = shmemlib::SHMEM_MINOR_VERSION;
pub const VENDOR_STRING: &'static [u8; 9usize] = shmemlib::SHMEM_VENDOR_STRING;

pub type ThreadLevel = i32;

pub const THREAD_SINGLE: ThreadLevel = shmemlib::SHMEM_THREAD_SINGLE as ThreadLevel;
pub const THREAD_FUNNELED: ThreadLevel = shmemlib::SHMEM_THREAD_FUNNELED as ThreadLevel;
pub const THREAD_SERIALIZED: ThreadLevel = shmemlib::SHMEM_THREAD_SERIALIZED as ThreadLevel;
pub const THREAD_MULTIPLE: ThreadLevel = shmemlib::SHMEM_THREAD_MULTIPLE as ThreadLevel;

pub type SymmMemAddr = *mut libc::c_void;

// TEAMS: don't like this, can't extend to derived teams.  just a
// stop-gap
//

pub type TeamType = shmemlib::shmem_team_t;

pub fn team_world() -> TeamType {
    unsafe { shmemlib::SHMEM_TEAM_WORLD }
}
pub fn team_shared() -> TeamType {
    unsafe { shmemlib::SHMEM_TEAM_SHARED }
}
pub fn team_invalid() -> TeamType {
    unsafe { shmemlib::SHMEM_TEAM_INVALID }
}

//
// == initialize and finalize ============================================
//

pub fn init() {
    unsafe {
        shmemlib::shmem_init();
    }
}

pub fn init_thread(req: ThreadLevel) -> ThreadLevel {
    unsafe {
        let mut prov: i32 = -1;

        shmemlib::shmem_init_thread(req, &mut prov);

        prov as ThreadLevel
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
    let mut a: i32 = 0;
    let mut b: i32 = 0;

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
    unsafe { shmemlib::shmem_my_pe() }
}

pub fn n_pes() -> i32 {
    unsafe { shmemlib::shmem_n_pes() }
}

pub fn team_my_pe(t: shmemlib::shmem_team_t) -> i32 {
    unsafe { shmemlib::shmem_team_my_pe(t) }
}

pub fn team_n_pes(t: shmemlib::shmem_team_t) -> i32 {
    unsafe { shmemlib::shmem_team_n_pes(t) }
}

pub fn pe_accessible(pe: i32) -> bool {
    unsafe { shmemlib::shmem_pe_accessible(pe) == 1 }
}

pub fn addr_accessible(addr: SymmMemAddr, pe: i32) -> bool {
    unsafe { shmemlib::shmem_addr_accessible(addr, pe) == 1 }
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

// pub trait RDMAApi<T> {
//     fn p(&self, dest: *mut T, src: T, pe: i32);
// }

// impl dyn RDMAApi<i32> {
//     fn p(&self, dest: *mut i32, src: i32, pe: i32) {
//         unsafe {
//             shmemlib::shmem_int_p(dest, src, pe);
//         }
//     }
// }

//
// == memory management ==================================================
//

// so `sizeof` gives us `usize` as the amount of memory to allocate.
// this doesn't match the type that bindgen dumped out for us, so we
// have to convert

pub fn malloc(n: usize) -> SymmMemAddr {
    unsafe { shmemlib::shmem_malloc(n as u64) }
}

pub fn calloc(n: usize, s: usize) -> SymmMemAddr {
    unsafe { shmemlib::shmem_calloc(n as u64, s as u64) }
}

pub fn realloc(m: SymmMemAddr, n: usize) -> SymmMemAddr {
    unsafe { shmemlib::shmem_realloc(m, n as u64) }
}

pub fn align(a: u64, n: usize) -> SymmMemAddr {
    unsafe { shmemlib::shmem_align(a, n as u64) }
}

pub fn free(m: SymmMemAddr) {
    unsafe {
        shmemlib::shmem_free(m);
    }
}

pub fn malloc_with_hints(n: usize, h: u64) -> SymmMemAddr {
    unsafe { shmemlib::shmem_malloc_with_hints(n as u64, h as i64) }
}

pub fn ptr(m: SymmMemAddr, pe: i32) -> SymmMemAddr {
    unsafe { shmemlib::shmem_ptr(m, pe) }
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
