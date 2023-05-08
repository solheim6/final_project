use rand::Rng;
use rand::thread_rng;
use std::fs::File;
use std::io::prelude::*;

pub fn read_file(path: &str, r: usize) -> (usize, Vec<(usize, usize)>) {
    let mut first_line = true;
    let mut result: Vec<(usize, usize)> = Vec::new();
    let mut num_edges = 0usize;
    let mut rng = rand::thread_rng();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        if first_line {
            num_edges = line_str.parse::<usize>().unwrap();
            first_line = false;
        } 
        else {
            let a = rng.gen_range(0..num_edges);
            if a < r{ //Generates a random sample of approx size r
                println!("{} < {}",&a, &r);
                let v: Vec<&str> = line_str.trim().split(' ').collect();
                let x: usize = v[0].parse::<usize>().unwrap();
                let y: usize = v[1].parse::<usize>().unwrap();
                result.push((x, y));
            }
        }
    }
    return (num_edges, result);
}