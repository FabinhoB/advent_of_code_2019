use crate::crate01::module::Module;
use crate::crate01::spacecraft::Spacecraft;
use crate::crate01::fuel_calculator::FuelCalculator;
use std::fs;

pub fn main01() {

    let contents = fs::read_to_string("./input/01/input.txt")
        .expect("Something went wrong reading the file");

    let mass_list: Vec<i32> = contents
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut modules: Vec<Module> = Vec::with_capacity(mass_list.len());
    for &mass in mass_list.iter() {
        modules.push(Module::new(mass));
    }

    let my_spacecraft = Spacecraft::new(modules);
    println !("The spacecraft has a fuel requirement of {}", my_spacecraft.compute_fuel_requirement());

}