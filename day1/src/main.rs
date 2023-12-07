use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part one answer: {}", decode(&input));
    let mut modified_input = input.clone();
    for (word, number) in generate_number_string_hashmap() {
        modified_input = modified_input.replace(word, number);
    }
    println!("Part two answer: {}", decode(&modified_input))
}

fn decode(input : &String) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let mut num = String::new();
        for character in line.chars() {
            if character.is_numeric() {
                num.push(character);
                break
            }
        }
        for character in line.chars().rev() {
            if character.is_numeric() {
                num.push(character);
                break
            }
        }

        let num_as_number: u32 = num.parse().unwrap_or(0);
        // println!("{}", num_as_number);
        sum += num_as_number;
    }
    sum
}

fn generate_number_string_hashmap() -> HashMap<&'static str, &'static str>{
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("one", "o1e");
    map.insert("two", "t2o");
    map.insert("three", "t3e");
    map.insert("four", "f4r");
    map.insert("five", "f5e");
    map.insert("six", "s6x");
    map.insert("seven", "s7n");
    map.insert("eight", "e8t");
    map.insert("nine", "n9e");
    map
}