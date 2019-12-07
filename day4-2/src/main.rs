use std::env;

fn main() {
    println!("Hello, world!!!");

    let args: Vec<String> = env::args().collect();

    let start: u32 = args[1].parse::<u32>().expect("Arg 1 (start) must be a number");
    let end: u32 = args[2].parse::<u32>().expect("Arg 2 (end) must be a number");

    let results = (start..end).filter(|i| pass_valid(&split_digits(*i))).count();

    println!("{}, {}, results: {}", start, end, results);

    //let file_contents = fs::read_to_string(path).expect("Couldn't find the file");
}

fn pass_valid(digits: &[u32]) -> bool {
    if digits.len() != 6 { return false; }
    if digits.iter().any(|d| d >= &10) { return false; }
    let mut dupe_found = false;
    let mut i = 0;
    while i < digits.len() {
        //println!("{} < {}: {}", digits[i], digits[i-1], digits[i] < digits[i-1]);
        if i > 0 && digits[i] < digits[i-1] { return false; }
        let consec = consecutive_digits(&digits[i..]);
        if consec == 2 { dupe_found = true; }
        i += consec;
    }
    if dupe_found == false { return false; }
    true
}

fn split_digits(num: u32) -> Vec<u32> {
    let mut remaining = num;
    let mut digits: Vec<u32> = Vec::new();
    digits.reserve(6);
    while remaining > 0 {
        digits.push(remaining % 10);
        remaining = remaining / 10;
    }
    digits.reverse();
    digits
}

fn consecutive_digits(digits: &[u32]) -> usize {
    if digits.len() == 0 { return 0; }
    let first_digit = digits[0];
    let mut consec = 1;
    for digit in &digits[1..] {
        if digit == &first_digit { consec += 1; }
        else { return consec; }
    }
    return consec;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn split1() {
        assert_eq!(split_digits(1), [1]);
    }

    #[test]
    fn split10() {
        assert_eq!(split_digits(10), [1, 0]);
    }

    #[test]
    fn split121199() {
        assert_eq!(split_digits(121199), [1, 2, 1, 1, 9, 9])
    }

    #[test]
    fn invalid111111() {
        assert_eq!(pass_valid(&split_digits(111111)), false);
    }
    
    #[test]
    fn invalid223450() {
        assert_eq!(pass_valid(&split_digits(223450)), false);
    }

    #[test]
    fn invalid123789() {
        assert_eq!(pass_valid(&split_digits(123789)), false);
    }

    #[test]
    fn valid234566() {
        assert_eq!(pass_valid(&split_digits(234566)), true);
    }

    #[test]
    fn valid112233() {
        assert_eq!(pass_valid(&split_digits(112233)), true);   
    }

    #[test]
    fn invalid123444() {
        assert_eq!(pass_valid(&split_digits(123444)), false);
    }

    #[test]
    fn valid111122() {
        assert_eq!(pass_valid(&split_digits(111122)), true);
    }

    #[test]
    fn consec_len0() {
        assert_eq!(consecutive_digits(&[]), 0);
    }

    #[test]
    fn consec_len1_end() {
        assert_eq!(consecutive_digits(&[6]), 1);
    }

    #[test]
    fn consec_len3_end() {
        assert_eq!(consecutive_digits(&[6, 6, 6]), 3)
    }

    #[test]
    fn consec_len1() {
        assert_eq!(consecutive_digits(&[6, 3]), 1);
    }

    #[test]
    fn consec_len3() {
        assert_eq!(consecutive_digits(&[6, 6, 6, 3]), 3)
    }
    
    #[test]
    fn consec2_dupe1() {
        assert_eq!(consecutive_digits(&[2, 2, 4, 2]), 2);
    }

    #[test]
    fn consec2_dupe3() {
        assert_eq!(consecutive_digits(&[2, 2, 4, 2, 2, 2]), 2);   
    }

    #[test]
    fn consec_multiple_dupes() {
        assert_eq!(consecutive_digits(&[2, 2, 6, 2, 4, 4, 4, 2]), 2);
    }
}