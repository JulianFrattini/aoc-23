use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let input: Vec<String> = read_lines("data/day02.txt");

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

    println!("{index_sum}");
}