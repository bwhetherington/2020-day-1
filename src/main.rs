use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

impl Policy {
    fn check_pwd_1(&self, pwd: &str) -> bool {
        let count = pwd.chars()
            .filter(|ch| *ch == self.letter)
            .count();
        self.min <= count && count <= self.max
    }

    fn check_pwd_2(&self, pwd: &str) -> bool {
        let a = pwd.chars().nth(self.min - 1).unwrap_or(' ');
        let b = pwd.chars().nth(self.max - 1).unwrap_or(' ');
        (a == self.letter) ^ (b == self.letter)
    }
}

fn parse_policy(input: &str, policy_regex: &Regex) -> Option<Policy> {
    let caps = policy_regex.captures(input)?;
    let min: usize = caps["min"].parse().ok()?;
    let max: usize = caps["max"].parse().ok()?;
    let letter: char = caps["letter"].parse().ok()?;
    Some(Policy { min, max, letter })
}

fn parse_line<'a, 'b>(line: &'a str, policy_regex: &'b Regex) -> Option<(Policy, &'a str)> {
    let sections: Vec<&str> = line.split(':')
        .map(|section| section.trim())
        .collect();
    if sections.len() == 2 {
        let policy = sections[0];
        let pwd = sections[1];
        let policy = parse_policy(policy, policy_regex)?;
        Some((policy, pwd))
    } else {
        None
    }
}

fn count_valid_pwds(filename: &str, policy_regex: &Regex) -> Option<usize> {
    let file = File::open(filename).ok()?;
    let mut reader = BufReader::new(file);
    let count = reader.lines()
        .flat_map(|line| line.ok())
        .flat_map(|line| {
            let (policy, pwd) = parse_line(&line, policy_regex)?;

            // Just swap this method call to switch between part 1 and part 2
            Some(policy.check_pwd_2(pwd))
        })
        .filter(|valid| *valid)
        .count();
    Some(count)
}

const POLICY_REGEX: &'static str = r"(?P<min>[0-9]+)-(?P<max>[0-9]+) (?P<letter>.)";

fn main() {
    let policy_regex = Regex::new(POLICY_REGEX).unwrap();
    let file = "./test.txt";
    let count = count_valid_pwds(file, &policy_regex).unwrap();
    println!("{}", count);
}
