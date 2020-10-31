# RustSA
A CLI for [RSA](https://en.wikipedia.org/wiki/RSA_(cryptosystem)) implemented in [Rust](https://www.rust-lang.org/).

This an implementation of the RSA encryption algorithm for the command line written in rust. All of the mathematical algorithms, including the extended euclidean algorithm, the fast powering algorithm, and Miller-Rabin primality testing, have been coded from first principles using the 5 basic arithmetic operations (additons, subtraction, multiplication, division, and remainder). 

The three basic operations this program has implemented are key creation, encrypting message, and decrypting cipher texts. To use them, refer to this table

| Command   | Usage                          | Description                                                                                                                                          |
|-----------|--------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------|
| `new_key` | `./RustSA new_key key_size`    | Creates a new public/private key pair, with primes being `key_size` bits in size.                                                                    |
| `encrypt` | `./RustSA encrypt message N e` | Encrypts `message`, which is a number, using the public key `(N, e)`, where `N` is the product of the secret primes and `e` is the public exponent.  |
| `decrypt` | `./RustSA encrypt message N d` | Decrypts `cipher`, which is a number, using the private key `(N, d)`, where `N` is the product of the secret primes and `d` is the private exponent. |
