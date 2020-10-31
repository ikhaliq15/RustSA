use crate::random;
use gmp::mpz::Mpz;

pub fn divide(a: &Mpz, b: &Mpz) -> (Mpz, Mpz) {
    if b.eq(&Mpz::zero()) {
        panic!("Cannot divide by zero!");
    }
    (a / b, a % b)
}

pub fn egcd(a: &Mpz, b: &Mpz) -> (Mpz, Mpz, Mpz) {
    if b.eq(&Mpz::zero()) {
        return (a.clone(), Mpz::one(), Mpz::zero());
    }
    let mut u = Mpz::one();
    let mut g = a.clone();
    let mut x = Mpz::zero();
    let mut y = b.clone();
    while y.ne(&Mpz::zero()) {
        let (q, r) = divide(&g, &y);
        let s = u - q * &x;
        u = x;
        g = y;
        x = s;
        y = r;
    }
    let v = (&g - (a * &u)) / b;
    (g, u, v)
}

pub fn fast_power(g: &Mpz, A: &Mpz, N: &Mpz) -> Mpz {
    let mut p = A.clone();
    let mut a = g.clone();
    let mut b = Mpz::one();
    while p.gt(&Mpz::zero()) {
        if &p % 2 == Mpz::one() {
            b = (&b * &a) % N;
        }
        a = a.pow(2) % N;
        p = p / 2;
    }
    b % N
}

pub fn miller_rabin(a: &Mpz, n: &Mpz) -> bool {
    let mut m: Mpz = n.clone() - 1;
    let mut k: Mpz = Mpz::zero();
    while &m % 2 == Mpz::zero() {
        m = m / 2;
        k += 1;
    }
    let mut w = fast_power(a, &m, n);
    if w == Mpz::one() {
        return false
    }
    let mut i = Mpz::zero();
    while i.lt(&k) {
        if w == n - 1 {
            return false;
        }
        w = w.pow(2) % n;
        i += 1;
    }
    true
}

pub fn probably_prime(n: &Mpz, reps: i32) -> bool {
    let n1 = n.clone();
    if n1 == Mpz::from(2) || n1 == Mpz::from(3) {
        return true;
    }
    if n % 2 == Mpz::zero() {
        return false;
    }
    let mut rng = random::new_rng();
    for _ in 1..reps {
        let a = random::random_number(&mut rng, &Mpz::from(2), &(n - 1));
        if miller_rabin(&a, &n1) {
            return false;
        }
    }
    true
}

pub fn find_prime(lower_bound: &Mpz, upper_bound: &Mpz) -> Mpz {
    let mut rng = random::new_rng();
    loop {
        let candidate = random::random_number(&mut rng, lower_bound, upper_bound);
        if probably_prime(&candidate, 30) {
            return candidate;
        }
    }
}

pub fn wrap(a: &Mpz, n: &Mpz) -> Mpz {
    let b = a % n;
    if b.lt(&Mpz::zero()) {
        b + n
    } else {
        b
    }
}
