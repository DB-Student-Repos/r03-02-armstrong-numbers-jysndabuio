pub fn is_armstrong_number(num: u32) -> bool {
    // For single digit => all armstrong number
    
    match num.to_string().len() {
        1 => return true,
        _ => check(num),
    }
}

fn check(num: u32) -> bool {
    let num_str = num.to_string();
    let num_digits = num_str.len() as u32;
    let armstrong_number = num_str
        .chars()
        .map(|x| x.to_digit(10).unwrap().pow(num_digits))
        .fold(0u32, |acc, x| acc.wrapping_add(x)); 
        // acc stands for accumulator/holder 
    num == armstrong_number
}