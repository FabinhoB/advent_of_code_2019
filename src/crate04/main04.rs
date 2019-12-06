use std::fs;

pub fn main04() {
    let contents = fs::read_to_string("./input/04/input.txt")
        .expect("Something went wrong reading the file");

    let code_inputs: Vec<i32> = contents
        .split('-')
        .map(|s| s.parse().unwrap())
        .collect();

    let code_inf: i32 = code_inputs[0];
    let code_sup: i32 = code_inputs[1];
    let mut codes: Vec<i32> = Vec::with_capacity(0);
    let base: i32 = 10;

    for code in code_inf..code_sup+1 {
        let mut is_valid_code = false;
        let mut is_double = true;
        let mut last_digit = code%10;
        for digit_index in 1..6 {
            let digit = (code/base.pow(digit_index) as i32)%10;
            let next_digit = (code/base.pow(digit_index+1) as i32)%10;
            if digit>last_digit {
                is_valid_code = false;
                break;
            } else if digit==last_digit {
                if digit != next_digit && is_double {
                    is_valid_code = true;
                } else {
                    is_double= false;
                    continue;
                }
            }
            is_double = true;
            last_digit=digit;
        }
        if is_valid_code {
            codes.push(code);
        }
    }
    println!("The code are ({}): {:?}", codes.len(), codes);
}