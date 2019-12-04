use crate::crate01::module::Module;
use crate::crate01::spacecraft::Spacecraft;

pub trait FuelCalculator {
    fn compute_fuel_requirement(&self) -> i32;
}

impl FuelCalculator for Module {
    fn compute_fuel_requirement(&self) -> i32 {
        let mut additional_fuel_mass= (self.get_mass() / 3)as i32 - 2;
        let mut total_fuel_mass= 0;
        while additional_fuel_mass > 0 {
            total_fuel_mass += additional_fuel_mass;
            additional_fuel_mass = (additional_fuel_mass / 3)as i32 - 2;
        }
        total_fuel_mass
    }
}

impl FuelCalculator for Spacecraft {
    fn compute_fuel_requirement(&self) -> i32 {
        let mut mass = 0;
        for &module in self.get_modules().iter() {
            mass += module.compute_fuel_requirement();
        }
        mass
    }
}