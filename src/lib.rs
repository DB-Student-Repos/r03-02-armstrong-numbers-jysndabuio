pub fn is_armstrong_number(num: u32) -> bool {
    // For single digit => all armstrong number
    let  num_chars = num.to_string().replace("_", "");

    if num_chars.len() == 1 {
        return true
    } else {
        check(&num_chars, num)
    }
}

fn check(num_chars: &String, num: u32) -> bool {
    let mut armstrong_number: u32 = 0;
    for x in num_chars.chars(){
        let digit = x.to_digit(10).unwrap();
        armstrong_number = armstrong_number.wrapping_add(digit.pow(num_chars.len() as u32));
    }
    if num == armstrong_number {
        return true
    } else {
        return false
    }
}