//
// !!! OSHMEM CURRENTLY DOESN'T SUPPORT TEAMS !!!
//

use shmem;
use std::env;
use uname::uname;

fn main() {
    let node = uname().unwrap().nodename;

    shmem::init();

    let w_me = shmem_team_my_pe(SHMEM_TEAM_WORLD);
    let w_npes = shmem_team_n_pes(SHMEM_TEAM_WORLD);

    let s_me = shmem_team_my_pe(SHMEM_TEAM_SHARED);
    let s_npes = shmem_team_n_pes(SHMEM_TEAM_SHARED);

    println!("{}: PE {:>4} of {:>4} shared team PE {:>4} of {:>4}",
             node,
             w_me, w_npes,
             s_me, s_npes);

    shmem::finalize();
}
