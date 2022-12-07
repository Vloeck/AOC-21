use std::str::FromStr;
use crate::day2::Command::{Down, Forward, Up};
use crate::lines;

// TODO: set correct values
type PreparedData = Command;
type CalculationResult = isize;

const DAY: u32 = 2;
#[allow(dead_code)]
const TEST_RESULT_1: CalculationResult = 150;
#[allow(dead_code)]
const TEST_RESULT_2: CalculationResult = 900;

#[derive(Debug)]
enum Command {
    Forward(isize),
    Up(isize),
    Down(isize),
}

#[derive(Debug)]
struct Position {
    x: isize,
    y: isize,
    aim: isize,
}

impl Position {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            aim: 0,
        }
    }

    fn move_position(&self, command: &PreparedData) -> Self {
        match command {
            Forward(x) => Self {
                x: self.x + x,
                y: self.y,
                aim: 0,
            },
            Up(y) => Self {
                x: self.x,
                y: self.y - y,
                aim: 0,
            },
            Down(y) => Self {
                x: self.x,
                y: self.y + y,
                aim: 0,
            }
        }
    }

    fn move_aim(&self, command: &PreparedData) -> Self {
        match command {
            Forward(x) => Self {
                x: self.x + x,
                y: self.y + x * self.aim,
                aim: self.aim,
            },
            Up(y) => Self {
                x: self.x,
                y: self.y,
                aim: self.aim - y,
            },
            Down(y) => Self {
                x: self.x,
                y: self.y,
                aim: self.aim + y,
            }
        }
    }

    fn value(&self) -> isize {
        self.x * self.y
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(" ").collect::<Vec<&str>>();
        assert_eq!(split.len(), 2);
        let x: isize = split[1].parse().unwrap();
        match split[0] {
            "forward" => Ok(Forward(x)),
            "up" => Ok(Up(x)),
            "down" => Ok(Down(x)),
            _ => Err(())
        }
    }
}

struct Data {
    data: Vec<PreparedData>,
}

impl From<Vec<String>> for Data {
    fn from(lines: Vec<String>) -> Self {
        Data {
            data: lines.iter().filter_map(|s| s.parse().ok()).collect::<Vec<PreparedData>>()
        }
    }
}

impl Data {
    fn calculate1(&mut self) -> CalculationResult {
        self.data.iter()
            .fold(Position::new(), |a, i| a.move_position(i))
            .value()
    }

    fn calculate2(&mut self) -> CalculationResult {
        self.data.iter()
            .fold(Position::new(), |a, i| a.move_aim(i))
            .value()
    }
}

pub(crate) fn main() {
    let lines = lines::read_lines(format!("resources/day{DAY}.txt"));
    if let Ok(lines) = lines {
        let mut data: Data = lines.into();
        let result = data.calculate1();
        println!("Day {DAY}: result = {result}");
        let result = data.calculate2();
        println!("Day {DAY}: result = {result}");
    }
    println!()
}

#[cfg(test)]
mod tests {
    use crate::lines;
    use super::{Data, CalculationResult, TEST_RESULT_1, TEST_RESULT_2, DAY};

    fn test_data() -> Vec<String> {
        lines::read_lines(format!("test_resources/day{DAY}.txt")).unwrap()
    }

    #[test]
    fn test1() {
        let lines = test_data();
        let mut data: Data = lines.into();
        let result = data.calculate1();
        let expected: CalculationResult = TEST_RESULT_1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let lines = test_data();
        let mut data: Data = lines.into();
        let result = data.calculate2();
        let expected: CalculationResult = TEST_RESULT_2;
        assert_eq!(result, expected);
    }
}