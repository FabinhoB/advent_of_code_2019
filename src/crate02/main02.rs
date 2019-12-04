use std::fs;

pub fn main02() {
    let contents = fs::read_to_string("./input/02/input.txt")
        .expect("Something went wrong reading the file");

    for noun in 0..99 {
        for verb in 0..99 {
            let mut integers: Vec<usize> = contents
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();

            integers[1] = noun;
            integers[2] = verb;
            run_computer(&mut integers);
            if integers[0] == 19690720 {
                println!("Run successful, value is {}", 100 * noun + verb);
            }
        }
    }
}

fn run_computer(integers: &mut Vec<usize>) {
    let mut index: usize = 0;
    while integers[index] != 99 {
        match integers[index] {
            1 => run_addition(integers, index),
            2 => run_multiplication(integers, index),
            _ => panic!("1202 program alarm, command is {:?}", integers),
        }
        index += 4;
    }
}

fn run_addition(integers: &mut Vec<usize>, index: usize) {
    let index3 = integers[index+3];
    integers[index3] = integers[integers[index+1]] + integers[integers[index+2]];
}

fn run_multiplication(integers: &mut Vec<usize>, index: usize) {
    let index3 = integers[index+3];
    integers[index3] = integers[integers[index+1]] * integers[integers[index+2]];
}
