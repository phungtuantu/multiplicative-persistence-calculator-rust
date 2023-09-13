struct NumberFinderParameters{
    base: u64,
    forbidden_digits: Vec<&u64>,
    find_prime_factors: fn(value: u64) -> u64,
    find_forbidden_digits: fn(value: u64) -> u64,
}

fn find_prime_factors(value: u64) -> Vec<&u64>{
    # TODO
}

fn find_forbidden_digits(value: u64) -> Vec<&u64>{
    # TODO
}

impl NumberFinderParameters {
    fn new(base: u64) -> NumberFinderParameters {
        NumberFinderParameters {
            base,
            find_prime_factors,
            find_forbidden_digits,
            forbidden_digits: find_forbidden_digits(base)
        }
    }
}

pub fn find_next_value(current_value: u64, base: u64) -> u64{
    // TODO: Find the next value for base 10
    return 10;
}
