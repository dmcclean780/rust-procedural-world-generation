pub fn euclidean_mod(a: isize, b: usize) -> usize {
    let b = b as isize;
    ((a % b) + b) as usize % b as usize
}

pub fn div_floor(a: isize, b: isize) -> isize {
    let mut q = a / b;
    if (a ^ b) < 0 && a % b != 0 {
        q -= 1;
    }
    q
}