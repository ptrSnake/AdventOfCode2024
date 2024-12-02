use std::fs;

fn read_input() -> (Vec<i32>, Vec<i32>) {
    let input = fs::read_to_string("./src/input.txt").expect("Failed to read input.txt");

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();

        left.push(numbers[0].parse::<i32>().unwrap());
        right.push(numbers[1].parse::<i32>().unwrap());
    }

    return (left, right);
}

fn calculate_distance(left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32 {
    left.sort();
    right.sort();

    let mut distances: Vec<i32> = Vec::new();

    for (a, b) in left.iter().zip(right.iter()) {
        distances.push(a - b);
    }

    let mut total_distances = 0;

    for distance in distances.iter() {
        total_distances += distance.abs();
    }

    return total_distances;
}

fn calculate_similarity(left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32 {
    left.sort();
    right.sort();

    let mut similarity = 0;

    for &a in left.iter() {
        similarity += a * right.iter().filter(|&&b| a == b).count() as i32;
    }

    return similarity;
}

fn main() {
    let (mut left, mut right) = read_input();

    let total_distances = calculate_distance(&mut left, &mut right);

    println!("Total distance: {}", total_distances);

    let similarity = calculate_similarity(&mut left, &mut right);

    println!("Similarity: {}", similarity);
}
