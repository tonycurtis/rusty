use uname::uname;
use shmem;

fn main() {
    let node = uname().unwrap().nodename;
    
    shmem::init();

    let me = shmem::my_pe();
    let n = shmem::n_pes();

    println!("{}: PE {} of {}: version \"{}\" {}.{}",
	     node,
	     me, n,
	     shmem::info_get_name(),
             shmem::MAJOR_VERSION,
             shmem::MINOR_VERSION);

    shmem::finalize();
}
