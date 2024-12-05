use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: &mut R) -> Result<i32> {
        let rules = read_rules(reader)?;

        let pages = read_pages(reader);

        let mut answer = 0;
        for line in &pages {
            if is_line_correct(line, &rules) {
                answer += find_middle_number(line);
            }
        }

        Ok(answer)
    }

    assert_eq!(143, part1(&mut BufReader::new(TEST.as_bytes()))?);

    let input_file = &mut BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Debug)]
struct Rule {
    // pub first: String,
    // pub second: String,
    regex: Regex,
}

impl Rule {
    pub fn from_line(line: &str) -> Result<Rule> {
        // let regex = Regex::new(line)?;
        let numbers: Vec<&str> = line.split("|").collect();
        let first = String::from(numbers[0]);
        let second = String::from(numbers[1]);

        let regex = Regex::new(&format!("{}.+{}", second, first))?;

        Ok(Rule { regex })
    }

    pub fn is_line_safe(&self, line: &str) -> bool {
        if self.regex.is_match(line) {
            return false;
        }
        true
    }
}

fn read_rules<R: BufRead>(reader: &mut R) -> Result<Vec<Rule>> {
    let mut rules = Vec::new();

    for line in reader.lines().flatten() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }
        rules.push(Rule::from_line(&line)?);
    }
    Ok(rules)
}

fn read_pages<R: BufRead>(reader: &mut R) -> Vec<String> {
    reader.lines().flatten().collect::<Vec<String>>()
}

fn is_line_correct(line: &str, rules: &Vec<Rule>) -> bool {
    for rule in rules {
        if !rule.is_line_safe(&line) {
            return false;
        }
    }
    true
}

fn find_middle_number(line: &str) -> i32 {
    let numbers = line.split(',').collect::<Vec<&str>>();

    let middle_index = numbers.len() / 2;
    numbers[middle_index].parse::<i32>().unwrap()
}
