mod utils;

use std::io::Error;
use std::result::Result;

use utils::read_lines_as_int_vector;

fn fuel_required_for_mass(mass: i32) -> i32 {
    mass / 3 - 2
}

fn fuel_for_modules(module_masses: Vec<i32>) -> i32 {
    module_masses
        .into_iter()
        .map(fuel_required_for_mass)
        .fold(0, |acc, f| acc + f)
}

fn additional_fuel_for_modules(module_masses: Vec<i32>) -> i32 {
    let additional_fuel_calculator = |mass| {
        let mut fuel_required = fuel_required_for_mass(mass);
        let mut total_fuel_required = 0;
        loop {
            if fuel_required <= 0 {
                break;
            }
            total_fuel_required += fuel_required;
            fuel_required = fuel_required_for_mass(fuel_required);
        }
        total_fuel_required
    };
    module_masses
        .into_iter()
        .map(additional_fuel_calculator)
        .fold(0, |acc, f| acc + f)
}

fn main() -> Result<(), Error> {
    let input = read_lines_as_int_vector("input/day01")?;
    println!("Day 01, Part 1: {}", fuel_for_modules(input.clone()));
    println!(
        "Day 01, Part 2: {}",
        additional_fuel_for_modules(input.clone())
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_for_module_test() {
        assert_eq!(34241, fuel_for_modules(&vec!(12, 14, 1969, 100756)));
    }

    #[test]
    fn additional_fuel_for_module_test() {
        assert_eq!(51314, additional_fuel_for_modules(&vec!(14, 1969, 100756)));
    }
}
