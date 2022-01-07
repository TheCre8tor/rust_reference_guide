fn pow(base: i64, exponent: usize) -> i64 {
    let mut res = 1;

    if exponent == 0 {
        return 1;
    }

    for a in 0..exponent {
        println!("count: {}", a);
        res = res * base;
        println!("Result: {}", res);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::pow;

    #[test]
    fn minus_two_raised_three_is_minus_eight() {
        assert_eq!(pow(-2, 3), -8);
    }
}
