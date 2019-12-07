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

fn pass_valid(digits: &Vec<u32>) -> bool {
    if digits.len() != 6 { return false; }
    if digits.iter().any(|d| d >= &10) { return false; }
    let mut dupe_found = false;
    for i in 1..6 {
        //println!("{} < {}: {}", digits[i], digits[i-1], digits[i] < digits[i-1]);
        if digits[i] < digits[i-1] { return false; }
        if digits[i] == digits[i-1] { dupe_found = true; }
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
    fn valid111111() {
        assert_eq!(pass_valid(&split_digits(111111)), true);
    }
    
    #[test]
    fn valid223450() {
        assert_eq!(pass_valid(&split_digits(223450)), false);
    }

    #[test]
    fn valid123789() {
        assert_eq!(pass_valid(&split_digits(123789)), false);
    }

    #[test]
    fn valid234566() {
        assert_eq!(pass_valid(&split_digits(234566)), true);
    }

}