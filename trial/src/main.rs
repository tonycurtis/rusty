use shmem;

#[cfg(test)]
mod tests {

    #[test]
    fn api_check() {
        let (maj, min) = shmem::info_get_version();

        assert_eq!(maj, shmem::MAJOR_VERSION);
        assert_eq!(min, shmem::MINOR_VERSION);
    }

}

fn main() {

    shmem::init();

    let me = shmem::my_pe();
    let n = shmem::n_pes();

    println!("PE {} of {}: version \"{}\" {}.{}",
             me, n,
             shmem::info_get_name(),
             shmem::MAJOR_VERSION,
             shmem::MINOR_VERSION);

    let bs = shmem::malloc(128);

    if me == 1 {
        let foo: shmem::SymmMemAddr = shmem::ptr(bs, 0);

        println!("{}: {:p}", me, foo);
    }

    shmem::free(bs);

    shmem::finalize();
}
