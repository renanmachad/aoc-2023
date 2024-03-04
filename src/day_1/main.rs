use std::{fmt::Debug, fs, u32};

fn main() {
    let file_contents = match fs::read_to_string("input_day_1.txt") {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return; // Exit the program if there's an error
        }
    };

    println!("result {}", process(file_contents.as_str()));
}

pub fn process(_input: &str) -> String {
    let output: u32 = _input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|character| character.to_digit(10));
            let first = it.next().expect("should be a number");

            match it.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("expcted a valid number")
        })
        .sum::<u32>();

    return output.to_string();
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_process() {
        let file_contents = match fs::read_to_string("input1.txt") {
            Ok(contents) => contents,
            Err(err) => {
                eprintln!("Error reading file: {}", err);
                return; // Exit the program if there's an error
            }
        };

        assert_eq!("142", process(file_contents.as_str()));
    }
}
