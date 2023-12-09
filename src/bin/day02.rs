use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn task1(filename: &str) -> i32 {
    let input: Vec<String> = read_lines(filename);

    let mut index_sum: i32 = 0;

    for line in input {
        let mut line_parts: std::str::Split<'_, &str> = line.split(":");

        // determine the index
        let game_string: &str = line_parts.next().unwrap();
        let game_index_str: &str = game_string.split(" ").last().unwrap();
        let game_index: i32 = game_index_str.parse::<i32>().unwrap();
        
        // determine the game sets
        let sets_string = line_parts.next().unwrap();
        let sets = sets_string.trim().split(";");

        let mut violation_detected: bool = false;

        for set in sets {
            let draws = set.trim().split(",");

            for draw in draws {
                let mut draw_strings = draw.split_whitespace();
                let draw_number = draw_strings.next().unwrap().parse::<i32>().unwrap();
                let draw_color: &str = draw_strings.next().unwrap();

                if (draw_color == "red" && draw_number > 12) || (draw_color == "green" && draw_number > 13) || (draw_color == "blue" && draw_number > 14) {
                    violation_detected = true;
                }
            }
        }

        if !violation_detected {
            index_sum += game_index;
        }
    }

    return index_sum;
}

fn task2(filename: &str) -> i32 {
    let input: Vec<String> = read_lines(filename);

    let mut sum: i32 = 0;

    for line in input {
        let mut line_parts: std::str::Split<'_, &str> = line.split(":");

        // determine the index
        let game_string: &str = line_parts.next().unwrap();
        let game_index_str: &str = game_string.split(" ").last().unwrap();
        let game_index: i32 = game_index_str.parse::<i32>().unwrap();
        
        // determine the game sets
        let sets_string = line_parts.last().unwrap();
        let sets = sets_string.trim().split(";");

        let mut min_red: i32 = 0;
        let mut min_blue: i32 = 0;
        let mut min_green: i32 = 0;

        for set in sets {
            let draws = set.trim().split(",");

            for draw in draws {
                let mut draw_strings = draw.split_whitespace();
                let draw_number = draw_strings.next().unwrap().parse::<i32>().unwrap();
                let draw_color: &str = draw_strings.next().unwrap();

                if draw_color == "red" && min_red < draw_number {
                    min_red = draw_number;
                } else if draw_color == "blue" && min_blue < draw_number {
                    min_blue = draw_number;
                } else if draw_color == "green" && min_green < draw_number {
                    min_green = draw_number;
                }
            }
        }

        sum += min_red*min_green*min_blue;
    }

    return sum;
}

fn main() {
    let index_sum = task2("data/day02.txt");

    println!("{index_sum}");
}