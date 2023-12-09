use std::fs;
use std::collections::HashMap;

fn main() {
    let mut input = fs::read_to_string("input.txt").unwrap();
    input = input.replace(":", "").replace(",", "").replace(";", "");
    let task_value_map = populate_first_task_hashmap();
    let first_answer = get_possible_games_sum(&input, task_value_map);
    println!("{}", first_answer);
}

fn get_possible_games_sum(input:&String, values:HashMap<&str, &u8>) -> u16 {
    let mut sum: u16 = 0;
    'outer : for line in input.lines() {
        let mut iterable = line.split_whitespace();
        let game_id = iterable.nth(1).unwrap().parse::<u16>().unwrap();

        // checking cycle
        let mut color: &str;
        let mut number: u8 = 0;
        for value in iterable.skip(2) {
            let value_as_number = value.parse::<u8>();
            match value_as_number {
                Ok(n) => number = n,
                Err(_) => {
                    color = value;
                    let color_value = values.get(color).unwrap();
                    if number > **color_value{
                        continue 'outer;
                    }
                }
            }
        }
        sum += game_id;
    }
    sum
}

fn populate_first_task_hashmap() -> HashMap<&'static str, &'static u8> {
    let mut task_values: HashMap<&str, &u8> = HashMap::new();
    task_values.insert("red", &12);
    task_values.insert("green", &13);
    task_values.insert("blue", &14);
    task_values
}
