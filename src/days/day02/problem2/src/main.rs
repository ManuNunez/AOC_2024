// Solución para problem2 del día 02
use std::io::{self, BufReader, BufRead};
use std::fs::File;

fn read_input() -> io::Result<Vec<Vec<i32>>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut reports = Vec::new();

    for line in reader.lines() {
        if let Ok(l) = line {
            let levels: Vec<i32> = l
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
            reports.push(levels);
        } else {
            println!("Error al leer una línea.");
        }
    }

    Ok(reports)
}
fn is_safe_report(report: &Vec<i32>) -> bool {
    let mut increasing = false;
    let mut decreasing = false;

   
    for window in report.windows(2) {
        let diff = (window[1] - window[0]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
        if window[1] > window[0] {
            increasing = true;
        } else if window[1] < window[0] {
            decreasing = true;
        }
    }

    increasing != decreasing
}

fn can_be_safe_by_removal(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);
        if is_safe_report(&modified_report) {
            return true;
        }
    }
    false
}

fn solve(reports: &Vec<Vec<i32>>) -> i32 {
    let mut safe_count = 0;

    for report in reports {
        if is_safe_report(report) || can_be_safe_by_removal(report) {
            safe_count += 1;
        }
    }

    safe_count
}

fn main() {
    match read_input() {
        Ok(reports) => {
            let safe_count = solve(&reports);
            println!("Número de reportes seguros: {}", safe_count);
        }
        Err(e) => println!("Error al leer el archivo: {}", e),
    }
}
