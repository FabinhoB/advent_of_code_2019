use crate::crate03::point::Point;
use crate::crate03::wire::Wire;
use std::fs;

pub fn main03() {
    let contents = fs::read_to_string("./input/03/input.txt")
        .expect("Something went wrong reading the file");

    let directions_list: Vec<String> = contents
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    let directions_wire_1: Vec<String> = directions_list[0]
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let directions_wire_2: Vec<String> = directions_list[1]
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let wire_1: Wire = Wire::new(&directions_wire_1);
    let wire_2: Wire = Wire::new(&directions_wire_2);

    let origin = Point::new(0,0);
    let points1 = wire_1.get_points();
    let points2 = wire_2.get_points();
    let mut min_distance = 0;
    let mut min_steps = points1.len() + points2.len();
    for i1 in 1..points1.len() {
        for i2 in 1..points2.len() {
            let p1 = points1[i1];
            let p2 = points2[i2];
            if p1.equals(&p2){
                let new_distance = origin.manhattan_distance(&p2);
                if new_distance < min_distance || min_distance == 0 {
                    min_distance = new_distance;

                    if i1+i2 < min_steps {
                        min_steps = i1+i2;
                    }
                }
            }
        }
    }

    println!("The min distance is {}", min_distance);
    println!("The min time is {}", min_steps);
}
