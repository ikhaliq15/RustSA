use gmp::mpz::Mpz;
use std::time::SystemTime;

pub fn random_number(rng: &mut gmp::rand::RandState, lower_bound: &Mpz, upper_bound: &Mpz) -> Mpz {
    (lower_bound + rng.urandom(upper_bound) % (upper_bound - lower_bound)) - 1
}

pub fn new_rng() -> gmp::rand::RandState {
    let sys_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Duration since UNIX_EPOCH failed");
    let mut rng = gmp::rand::RandState::new();
    let seed = Mpz::from(sys_time.subsec_millis());
    rng.seed(seed);
    rng
}
