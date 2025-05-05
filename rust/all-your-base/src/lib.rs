use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    let mut base_ten: usize = 0;
    for i in 0..number.len() {
        if number[i] >= from_base {
            return Err(Error::InvalidDigit(number[i]));
        }
        base_ten += (from_base.pow((number.len() - i - 1) as u32) * number[i]) as usize;
    }
    let mut ans: VecDeque<u32> = VecDeque::new();
    loop {
        if base_ten == 0 {
            break;
        }
        ans.push_front((base_ten % to_base as usize) as u32);
        base_ten /= to_base as usize;
    }

    Ok(if ans.is_empty() { vec![0] } else { ans.into() })
}
