use std::fs;

fn read_lines() -> Vec<String> {
    let input = fs::read_to_string("./src/input.txt").expect("Failed to read input.txt");

    return input.lines().map(|s| s.to_string()).collect();
}

fn count_safe_report(lines: &mut Vec<String>) -> i32 {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    let mut counter = 0;

    for line in lines {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        reports.push(numbers);
    }

    for report in reports {
        let differences: Vec<i32> = report.windows(2).map(|w| w[0] - w[1]).collect();
        let all_positives = differences.iter().all(|&d| d > 0);
        let all_negatives = differences.iter().all(|&d| d < 0);

        if all_positives || all_negatives {
            if differences.iter().all(|&d| d.abs() <= 3) {
                counter += 1;
            }
        }
    }

    return counter;
}

fn count_safe_report_2(lines: &mut Vec<String>) -> i32 {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    let mut counter = 0;

    for line in lines {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        reports.push(numbers);
    }

    for report in reports {
        let differences: Vec<i32> = report.windows(2).map(|w| w[0] - w[1]).collect();
        let all_positives = differences.iter().all(|&d| d > 0);
        let all_negatives = differences.iter().all(|&d| d < 0);

        if all_positives || all_negatives {
            if differences.iter().all(|&d| d.abs() <= 3) {
                counter += 1;
            }
        } else {
            let one_positive = differences.iter().filter(|&&d| d >= 0).count() == 1;
            let one_negative = differences.iter().filter(|&&d| d < 0).count() == 1;

            if one_positive || one_negative {
                if differences.iter().all(|&d| d.abs() <= 3) {
                    println!("{:?}", differences);
                    counter += 1;
                }
            }
        }
    }

    return counter;
}

fn main() {
    let mut lines = read_lines();

    let counter = count_safe_report(&mut lines);

    println!("Safe reports: {}", counter);

    let counter_2 = count_safe_report_2(&mut lines);

    println!("Safe reports 2: {}", counter_2);
}
