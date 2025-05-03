pub fn is_armstrong_number(num: u32) -> bool {
    let mut num_ref = num;
    let number_count: u32 = num.to_string().chars().count() as u32;
    let mut sum: u32 = 0;
    loop {
        if num_ref <= 0 {
            break;
        }
        sum += (num_ref % 10).pow(number_count);
        num_ref /= 10
    }
    sum == num
}
