// Solución para problem1 del día 02
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
            return false; // Si la diferencia no está entre 1 y 3, no es seguro
        }
        if window[1] > window[0] {
            increasing = true;
        } else if window[1] < window[0] {
            decreasing = true;
        }
    }

    // Un reporte es seguro si es solo creciente o solo decreciente, no ambos.
    increasing != decreasing
}

fn solve(reports: &Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    for report in reports {
        if is_safe_report(report) {
            ans += 1;
        }
    }
    ans
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
