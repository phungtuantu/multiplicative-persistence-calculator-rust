pub fn calculate_multiplicative_function_result(value: u64, base: u64) -> u64{
    let mut current_value:u64 = value;
    let mut res:u64 = 1;
    while current_value > 0{
        res = res*(current_value%base);
        current_value /= base;
    }
    return res;
}

pub fn calculate_multiplicative_persistence(value: u64, base: u64) -> u8{
    let mut current_value:u64 = value;
    let mut steps = 0;
    while current_value > base {
        current_value = calculate_multiplicative_function_result(current_value, base);
        steps+=1;
    }
    return steps;
}

pub fn find_next_value(current_value: u64, base: u64) -> u64{
    // TODO: Find the next value for base 10
    return 10;
}
