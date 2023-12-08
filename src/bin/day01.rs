use std::fs::read_to_string;
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn task1(filename: &str) -> i32 {
    // define a variable to store the sum in
    let mut sum: i32 = 0;

    // read the input file
    let input = read_lines(filename);

    for line in input {
        // find the first digit in a line
        let re_first = Regex::new(r"^[^0-9]*([0-9])").unwrap();
        let mut regex_match = re_first.find(&line).unwrap().as_str();
        let first_digit = regex_match.chars().last().unwrap();

        // find the last digit in a line
        let re_last = Regex::new(r"([0-9])[^0-9]*$").unwrap();
        let regex_match = re_last.find(&line).unwrap().as_str();
        let last_digit = regex_match.chars().next().unwrap();

        // concatenate the two chars to a string
        let mut number = String::from(first_digit);
        number.push(last_digit);
        
        // cast the string to an integer and add it to the sum
        sum = sum + number.parse::<i32>().unwrap();
    }

    sum
}

fn number_to_int(strnumber: &str) -> &str {
    let mut result: &str = "0";

    if strnumber.parse::<i32>().is_ok() {
        result = strnumber;
    } else {
        match strnumber {
            "one" => result="1",
            "two" => result="2",
            "three" => result="3",
            "four" => result="4",
            "five" => result="5",
            "six" => result="6",
            "seven" => result="7",
            "eight" => result="8",
            "nine" => result="9",
            _ => result="0"
        };
    }

    return result;
}

fn task2(filename: &str) -> i32 {
    // define a variable to store the sum in
    let mut sum: i32 = 0;

    // read the input file
    let input = read_lines(filename);

    for line in input {
        // find the first digit in a line
        let re_first = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
        let mut regex_match = re_first.find(&line).unwrap().as_str();
        let first_digit = number_to_int(regex_match);

        // find the last digit in a line
        let re_last = Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|[0-9])").unwrap();
        let line_rev = line.chars().rev().collect::<String>();
        regex_match = re_last.find(&line_rev).unwrap().as_str();
        let regex_match_rev = regex_match.chars().rev().collect::<String>();
        let last_digit = number_to_int(&regex_match_rev);

        // concatenate the two chars to a string
        let mut number = String::from(first_digit);
        number.push_str(last_digit);
        //println!("{line} -> {number}");
        
        // cast the string to an integer and add it to the sum
        sum = sum + number.parse::<i32>().unwrap();
    }

    sum
}

fn main() {
    let sum: i32 = task2("data/day01.txt");
    println!("{sum}");
}
