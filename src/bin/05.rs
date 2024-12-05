use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

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

        let pages = read_pages(reader)?;

        let mut answer = 0;
        for page in &pages {
            if is_page_correct(page, &rules) {
                answer += get_middle_number(page);
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
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: &mut R) -> Result<i32> {
        // The idea of solution is to sort the pages based on a custom ordering

        let rules = read_rules(reader)?;
        let pages = read_pages(reader)?;

        let mut answer = 0;

        for page in pages {
            let sorted_page: Vec<_> = page
                .clone()
                .into_iter()
                .sorted_by(|a, b| {
                    if rules.is_ordered_pair(*a, *b) {
                        return std::cmp::Ordering::Less;
                    }
                    return std::cmp::Ordering::Greater;
                })
                .collect();

            if page != sorted_page {
                answer += get_middle_number(&sorted_page)
            }
        }

        Ok(answer)
    }

    assert_eq!(123, part2(&mut BufReader::new(TEST.as_bytes()))?);

    let input_file = &mut BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn read_rules<R: BufRead>(reader: &mut R) -> Result<RuleSet> {
    let mut rule_set = RuleSet::new();

    for line in reader.lines().flatten() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }
        rule_set.insert(Rule::from_line(&line)?);
    }
    Ok(rule_set)
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Rule {
    left: i32,
    right: i32,
}

impl Rule {
    fn new(left: i32, right: i32) -> Rule {
        Rule { left, right }
    }
    fn from_line(line: &str) -> Result<Rule> {
        let numbers: Vec<i32> = line
            .split('|')
            .map(|s| s.trim().parse::<i32>())
            .collect::<Result<Vec<i32>, ParseIntError>>()?;
        let left = numbers[0];
        let right = numbers[1];
        Ok(Rule { left, right })
    }
}

struct RuleSet {
    rules: HashSet<Rule>,
}

impl RuleSet {
    fn new() -> RuleSet {
        RuleSet {
            rules: HashSet::new(),
        }
    }

    fn insert(&mut self, rule: Rule) {
        self.rules.insert(rule);
    }

    fn contains(&self, rule: &Rule) -> bool {
        self.rules.contains(rule)
    }

    fn is_ordered_pair(&self, a: i32, b: i32) -> bool {
        if self.contains(&Rule::new(a, b)) {
            return true;
        }
        false
    }
}

fn read_pages<R: BufRead>(reader: &mut R) -> Result<Vec<Vec<i32>>> {
    let mut pages = Vec::new();
    for line in reader.lines().flatten() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }
        let numbers: Vec<i32> = line
            .split(',')
            .map(|s| s.trim().parse::<i32>())
            .collect::<Result<Vec<i32>, ParseIntError>>()?;

        pages.push(numbers);
    }
    Ok(pages)
}

fn is_page_correct(page: &Vec<i32>, rules: &RuleSet) -> bool {
    let mut prev_number = &page[0];
    for next_number in &page[1..] {
        if !rules.is_ordered_pair(*prev_number, *next_number) {
            return false;
        }
        prev_number = next_number;
    }
    true
}

fn get_middle_number(page: &Vec<i32>) -> i32 {
    let middle_index = page.len() / 2;
    page[middle_index]
}
