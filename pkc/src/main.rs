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