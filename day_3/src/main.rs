use regex::Regex;

fn read_input() -> String {
    return std::fs::read_to_string("./src/input.txt").unwrap();
}

fn get_total_1(input: &str) -> i32 {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap(); 
    let re_number = Regex::new(r"\d{1,3},\d{1,3}").unwrap();

    let mut total = 0;

    for caps in re.find_iter(&input) {
        let str_numbers = re_number.find(&caps.as_str()).unwrap().as_str();        

        let numbers: Vec<i32> = str_numbers
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        
        total += numbers.iter().product::<i32>();
    }

    return total;
}

fn get_total_2(input: &str) -> i32 {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();

    let mut do_action = true;
    
    let mut total = 0;

    for caps in re.find_iter(&input) {
        if caps.as_str() == "do()" {
            do_action = true;
        } else if caps.as_str() == "don't()" {
            do_action = false;
        } else {
            if do_action {
                let re_number = Regex::new(r"\d{1,3},\d{1,3}").unwrap();
                let str_numbers = re_number.find(&caps.as_str()).unwrap().as_str();        

                let numbers: Vec<i32> = str_numbers
                    .split(',')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();
                
                total += numbers.iter().product::<i32>();            
            }
        }
    }

    return total;
}

fn main() {
    let input = read_input();

    let mut total = get_total_1(&input);
 
    println!("Total: {}", total);

    total = get_total_2(&input);

    println!("Total: {}", total);
}
