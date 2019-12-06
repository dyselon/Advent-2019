use std::fs;
use std::env;
use std::convert::TryInto;

mod point;
mod instruction;

use point::Point;
use instruction::Instruction;

fn main() {
    println!("Hello, world!!!");

    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    let file_contents = fs::read_to_string(path).expect("Couldn't find the file");

    //println!("{}", file_contents);
    let mut lines = file_contents.lines();
    let first = parse_wire(lines.next().expect("First line didn't exist")).expect("Failed to parse first wire");
    let second = parse_wire(lines.next().expect("Second line didn't exist")).expect("Failed to parse second wire");
    println!("{:?}", first);
    println!("{:?}", second);

    let mut closest: Option<(Point, i32)> = None;

    iterate_wire(&first, |pt_one, steps_one| {
        //if closest.is_some() && closest.unwrap().dist_raw() < pt_one.dist_raw() { return; }
        iterate_wire(&second, |pt_two, steps_two| {
            //println!("Evaluating {:?} vs. {:?}", pt_one, pt_two);
            if pt_one == pt_two {
                let latency = steps_one + steps_two;
                println!("Collision found at {:?} with latency {:?}", pt_one, latency);
                match closest {
                    None => closest = Some((pt_one, latency)),
                    Some((_, closest_latency)) => if latency < closest_latency {
                        closest = Some((pt_one, latency));
                    }
                }
            }
        });
    });

    let (closest_pt, closest_latency) = closest.expect("Collision not found");
    println!("Closest: {:?} w/ latency {}", closest_pt, closest_latency);

    // parse the first line into a series of segments
    // parse the second line into a series of segments
    // loop through the first set of segments
        // loop through the second set of segments
            // if they intersect
                // add them to the list of collisions

    // get the minimum distance
}

// given two line segments, do they intersect?

// what is the manhattan distance between two points

fn iterate_wire<F>(wire: &Vec<Instruction>, mut f: F) where F: FnMut(Point, i32) {
    let origin = Point { x: 0, y: 0 };
    let mut current = origin;
    let mut steps = 0;

    for inst in wire {
        for _ in 0..inst.mag {
            steps += 1;
            current = current + inst.dir;
            f(current, steps);
        }
    }
}

fn parse_wire(line: &str) -> Result<Vec<Instruction>, Box<dyn std::error::Error>> {
    line.split(',')
        .map(|x| x.try_into())
        .collect()
}