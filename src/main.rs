mod rsa;
mod math_helpers;
mod random;

use std::env;
use std::str::FromStr;
use std::process;
use gmp::mpz::Mpz;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 && args[1].eq_ignore_ascii_case("new_key") {
        let key_size: u32 = args[2].parse().unwrap();
        if key_size <= 4 {
            eprintln!("Key size must be greater than 4.");
            process::exit(2);
        }
        let (pub_key, priv_key) = rsa::generate_rsa_key(key_size);
        println!("Public Key = {:?}", pub_key);
        println!("");
        println!("Private Key = {:?}", priv_key);
    } else if args.len() == 5 && args[1].eq_ignore_ascii_case("encrypt") {
        let message = Mpz::from_str(&args[2]).unwrap();
        let N = Mpz::from_str(&args[3]).unwrap();
        let e = Mpz::from_str(&args[4]).unwrap();
        let cipher = rsa::rsa_encrypt((N, e), &message);
        println!("Encrypted cipher: {}", cipher);
    } else if args.len() == 5 && args[1].eq_ignore_ascii_case("decrypt") {
        let cipher = Mpz::from_str(&args[2]).unwrap();
        let N = Mpz::from_str(&args[3]).unwrap();
        let d = Mpz::from_str(&args[4]).unwrap();
        let message = rsa::rsa_decrypt((N, d), &cipher);
        println!("Decrypted message: {}", message);
    } else {
        eprintln!("Usage: {} mode [parameters]\n{}\n  {}\n  {}\n  {}",
            args[0],
            "- modes: new_key | encrypt | decrypt",
            "- new_key: one parameter which is the size of the secret primes in bits (must be greater than 4)",
            "- encrypt: three parameters. the message, as a number, to encrypt, and the two components of the public key, N then e",
            "- decrypt: three parameters. the cipher, as a number, to decrypt, and the two components of the private key, N then d",
        );
        process::exit(1);
    }
}
