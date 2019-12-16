use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;

pub fn read_lines_as_int_vector(path: &str) -> Result<Vec<i32>> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    Ok(buf_reader
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>())
}

pub fn read_file_as_int_vector(path: &str) -> Result<Vec<i32>> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents
        .split(",")
        .map(|l| l.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>())
}
