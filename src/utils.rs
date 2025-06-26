pub fn even_odd(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}

pub fn fibonacci(num: u32) -> u64 {
    if num == 0 {
        return 0;
    } else if num == 1 {
        return 1;
    }
    return fibonacci(num - 1) + fibonacci(num - 2)
}
