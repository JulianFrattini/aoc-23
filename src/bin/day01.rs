use std::fs::read_to_string;
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    // define a variable to store the sum in
    let mut sum: i32 = 0;

    // read the input file
    let input = read_lines("data/day01.txt");

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

    println!("{sum}");
}
