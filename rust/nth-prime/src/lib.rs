pub fn is_prime(n: u32) -> bool {
    for i in 2..n-1 {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let mut iterator: u32 = 2;
    let mut count_left: u32 = n + 1;
    loop {
        if is_prime(iterator) {
            count_left -= 1;
            if count_left == 0 {
                break;
            }
        }
        iterator += 1;
    }
    iterator
}
