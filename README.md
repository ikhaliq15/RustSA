# RustSA
A CLI for [RSA](https://en.wikipedia.org/wiki/RSA_(cryptosystem)) implemented in [Rust](https://www.rust-lang.org/).

This an implementation of the RSA encryption algorithm for the command line written in rust. All of the mathematical algorithms, including the extended euclidean algorithm, the fast powering algorithm, and Miller-Rabin primality testing, have been coded from first principles using the 5 basic arithmetic operations (additons, subtraction, multiplication, division, and remainder). Uses [`rust-gmp`](https://crates.io/crates/rust-gmp) crate to allow for abitrary integer precision, instead of limiting the program to 128 bit integers that are the default maximum in Rust.

## Commands
The three basic operations this program has implemented are key creation, encrypting message, and decrypting cipher texts. To use them, refer to this table

| Command   | Usage                          | Description                                                                                                                                              |
|-----------|--------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------|
| `new_key` | `./RustSA new_key key_size`    | Creates a new public/private key pair, with primes being `key_size` bits in size. `key_size` must be greater than 4.                                     |
| `encrypt` | `./RustSA encrypt message N e` | Encrypts `message`, which is a number, using the public key `(N, e)`, where `N` is the product of the<br>secret primes and `e` is the public exponent.   |
| `decrypt` | `./RustSA encrypt message N d` | Decrypts `cipher`, which is a number, using the private key `(N, d)`, where `N` is the product of the <br>secret primes and `d` is the private exponent. |

## Example

Create a public and private key using 16 bit primes.
```
$ ./RustSA new_key 16
Public Key = (2996203831, 2258472011)
Private Key = (2996203831, 1701586155)
```

Encrypt the message `31415` with a public key.
```
$ ./RustSA encrypt 31415 2996203831 2258472011
Encrypted cipher: 2698985146
```

Decrypt the ciphertext `2698985146` with a private key.
```
$ ./RustSA decrypt 2698985146 2996203831 1701586155
Decrypted message: 31415
```

## Possible Extensions

- Implement built-in ascci to integer encoding and decoding for easier message sharing.
- Implement the RSA Digtial Signature algorithm, mainly signing and verifying algorithms.
  - Also include reading/signing/verifying documents from the file system along with some hashing method.
- Optimize the existing algorithms for speed and make benchmarks for the program.
