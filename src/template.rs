use crate::lines;

// TODO: set correct values
type PreparedData = ();
type CalculationResult = u32;

const DAY: u32 = 1;
#[allow(dead_code)]
const TEST_RESULT_1: CalculationResult = 1;
#[allow(dead_code)]
const TEST_RESULT_2: CalculationResult = 2;

struct Data {
    data: Vec<PreparedData>
}

impl From<Vec<String>> for Data {
    fn from(lines: Vec<String>) -> Self {
        // TODO: implement
        drop(lines);
        Data {
            data: vec![]
        }
    }
}

impl Data {
    fn calculate1(&mut self) -> CalculationResult {
        // TODO: implement
        TEST_RESULT_1
    }

    fn calculate2(&mut self) -> CalculationResult {
        // TODO: implement
        TEST_RESULT_2
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