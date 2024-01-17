fn main() {
    let p: u64 = 61;
    let q: u64 = 53;
    let (n, e, d) = generate_keys(p, q);

    println!("Public Key: (e: {}, n: {})", e, n);
    println!("Private Key: (d: {}, n: {})", d, n);

    let message = 42;
    let encrypted = encrypt(message, e, n);
    let decrypted = decrypt(encrypted, d, n);

    println!("Original message: {}", message);
    println!("Encrypted message: {}", encrypted);
    println!("Decrypted message: {}", decrypted);
}

fn generate_keys(p: u64, q: u64) -> (u64, u64, u64) {
    let n = p * q;
    let phi = (p - 1) * (q - 1);
    let e = 65537; // Using 65537 as e
    let d = mod_inverse(e, phi).expect("Modular inverse does not exist.");

    (n, e, d)
}

fn mod_inverse(e: u64, phi: u64) -> Option<u64> {
    let (mut a, mut b, mut x0, mut x1) = (phi, e, 0u64, 1u64);

    while b > 0 {
        let q = a / b;
        (a, b) = (b, a % b);
        (x0, x1) = (x1, x0.wrapping_sub(x1.wrapping_mul(q)));
    }

    if a > 1 {
        None // No modular inverse if a is not 1
    } else {
        Some(x0.wrapping_add(phi) % phi)
    }
}

fn mod_exp(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 { return 0 }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 { result = result * base % modulus }
        exp >>= 1;
        base = base * base % modulus;
    }
    result
}

fn encrypt(message: u64, e: u64, n: u64) -> u64 {
    mod_exp(message, e, n)
}

fn decrypt(ciphertext: u64, d: u64, n: u64) -> u64 {
    mod_exp(ciphertext, d, n)
}