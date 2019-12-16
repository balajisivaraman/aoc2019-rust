mod utils;

use std::io::Error;
use std::result::Result;

use utils::read_file_as_int_vector;

fn intcode_program(intcode: Vec<i32>) -> Vec<i32> {
    let mut index = 0;
    let mut intcode = intcode;
    loop {
        let instruction = intcode[index];
        if instruction == 99 {
            break;
        }
        let position1 = intcode[index + 1];
        let position2 = intcode[index + 2];
        let target_position = intcode[index + 3];
        let v1 = intcode[position1 as usize];
        let v2 = intcode[position2 as usize];
        match instruction {
            1 => intcode[target_position as usize] = v1 + v2,
            2 => intcode[target_position as usize] = v1 * v2,
            _ => break,
        }
        index += 4;
    }
    intcode
}

fn main() -> Result<(), Error> {
    let input = read_file_as_int_vector("input/day02")?;
    println!("Day 02, Part 1: {}", intcode_program(input.clone())[0]);
    println!("Day 02, Part 2: {}", 2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intcode_program() {
        let intcode = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        assert_eq!(vec!(30, 1, 1, 4, 2, 5, 6, 0, 99), intcode_program(intcode));
    }
}
