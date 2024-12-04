// Solución para problem1 del día 01

use std::io::{self, BufReader, BufRead};
use std::fs::File;

fn read_input() -> io::Result<Vec<Vec<i32>>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut all = Vec::<Vec<i32>>::new();
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    for line in reader.lines() {
        if let Ok(l) = line {
            let nums: Vec<i32> = l
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();

            if nums.len() >= 2 {
                col1.push(nums[0]);
                col2.push(nums[1]);
            }
        } else {
            println!("Error al leer una línea.");
        }
    }

    all.push(col1);
    all.push(col2);
    Ok(all)
}

fn sort_v(v: &mut Vec<i32>) {
    v.sort();
}

fn solve(col1: &Vec<i32>, col2: &Vec<i32>) -> i32 {
    let mut sum = 0;
    let min_len = col1.len().min(col2.len());

    for i in 0..min_len {
        sum += (col1[i] - col2[i]).abs();
    }

    sum
}

fn main() {
    match read_input() {
        Ok(mut all) => {
            if let Some(col1) = all.get_mut(0) {
                sort_v(col1);
            }

            if let Some(col2) = all.get_mut(1) {
                sort_v(col2);
            }


            let result = solve(&all[0], &all[1]);


            println!("El resultado de la suma de las restas absolutas es: {}", result);
        }
        Err(e) => println!("Error al leer el archivo: {}", e),
    }
}
