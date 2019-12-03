use std::fs;
use std::env;
//use std::io;

fn main() {
    println!("Hello, world!!!");
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    let thing = fs::read_to_string(path).expect("Couldn't find the file");

    let ints = thing.lines().map(|line| line.parse::<i32>().unwrap());
    let intsvec: Vec<i32> = ints.collect();

    for thisint in intsvec.iter() {
        println!("{} -> {}", thisint, calc_advanced(&thisint));
    }

    let processed = intsvec.iter().map(calc_advanced);
    let sum = processed.sum::<i32>();
    println!("total: {}", sum);

    //println!("total: {}", blah);
    //println!("{}", thing);
}

fn calc_basic(mass: i32) -> i32 {
    let new = mass / 3 - 2;
    if new < 0 { return 0; }
    return new;
}

fn calc_advanced(mass: &i32) -> i32 {
    let mass = *mass;
    if mass <= 0 { return 0; }
    let new = calc_basic(mass);
    return new + calc_advanced(&new);
}
