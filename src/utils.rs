use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;

pub fn read_input(path: &str) -> Result<Vec<i32>> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    Ok(buf_reader
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>())
}
