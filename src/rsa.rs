use crate::math_helpers;
use crate::random;
use gmp::mpz::{Mpz};

pub fn generate_rsa_key(key_size: u32) -> ((Mpz, Mpz), (Mpz, Mpz)) {
    let p = math_helpers::find_prime(&Mpz::from(2).pow(key_size - 1), &Mpz::from(2).pow(key_size));
    let mut q = p.clone();
    while q.eq(&p) {
        q = math_helpers::find_prime(&Mpz::from(2).pow(key_size - 1), &Mpz::from(2).pow(key_size));
    }
    let N = &p * &q;
    let N0 = (p - 1) * (q- 1);
    let mut rng = random::new_rng();
    loop {
        let e = random::random_number(&mut rng, &Mpz::from(2), &(&N0 - 1));
        let (g, d, _) = math_helpers::egcd(&e, &N0);
        if g == Mpz::one() {
            return ((N.clone(), e), (N.clone(), math_helpers::wrap(&d, &N0)))
        }
    }
}

pub fn rsa_encrypt(pub_key: (Mpz, Mpz), message: &Mpz) -> Mpz {
    let (N, e) = pub_key;
    math_helpers::fast_power(&message, &e, &N)
}

pub fn rsa_decrypt(priv_key: (Mpz, Mpz), cipher: &Mpz) -> Mpz {
    let (N, d) = priv_key;
    math_helpers::fast_power(cipher, &d, &N)
}
