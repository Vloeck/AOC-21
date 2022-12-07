use crate::lines;

type PreparedData = usize;
type CalculationResult = usize;

const DAY: u32 = 1;
#[allow(dead_code)]
const TEST_RESULT_1: CalculationResult = 7;
#[allow(dead_code)]
const TEST_RESULT_2: CalculationResult = 5;

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
    fn calculate_with_len(&mut self, len: usize) -> CalculationResult {
        let mut c: usize = 0;
        let mut i: usize = len + 1;
        while i <= self.data.len() {
            let first = self.data[i - 1 - len..i - 1].iter().fold(0usize, |a, i| a + *i);
            let second = self.data[i - len..i].iter().fold(0usize, |a, i| a + *i);
            // println!("{first} < {second} = {}", first < second);
            if first < second {
                c += 1;
            }
            i += 1;
        }
        c
    }

    fn calculate1(&mut self) -> CalculationResult {
        self.calculate_with_len(1)
    }

    fn calculate2(&mut self) -> CalculationResult {
        self.calculate_with_len(3)
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