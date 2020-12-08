#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use regex::Regex;

lazy_static! {
    static ref RE_NAME: Regex = Regex::new(r"([\w ]+) bags contain").unwrap();
    static ref RE_CONTENTS: Regex = Regex::new(r"(\d+) ([\w ]+) bags?").unwrap();
}

fn baggins(entries: &Vec<String>, name: String) -> Vec<String> {
    let mut types: Vec<String> = Vec::new();
    for bag in entries {
        let bagname = RE_NAME.captures(&bag).unwrap();
        for content in RE_CONTENTS.captures_iter(&bag) {
            if content[2] == name {
                let srcname = &bagname[1];
                if types.contains(&srcname.to_string()) == false {
                    types.push(srcname.to_string());
                }
                let parents = baggins(&entries, srcname.to_string());
                for parent in parents {
                    if types.contains(&parent.to_string()) == false {
                        types.push(parent.to_string());
                    }
                }
            }
        }
    }
    return types
}

fn baggins2(entries : &Vec<String>, name: String, number: u32) -> u32 {
    let mut total = 0;
    for bag in entries {
        for bagname in RE_NAME.captures_iter(&bag) {
            if bagname[1] == name {
                for content in RE_CONTENTS.captures_iter(&bag) {
                    total += number * baggins2(
                        &entries, content[2].to_string(), content[1].parse::<u32>().unwrap()
                    );
                }
            }
        }
    }
    return total + number;
}

fn main() {
    let mut entries: Vec<String> = Vec::new();
    let f = File::open("../day07/input").unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        entries.push(line.unwrap().to_string());
    }

    println!("part1: {}", baggins(&entries, "shiny gold".to_string()).len());
    println!("part2: {}", baggins2(&entries, "shiny gold".to_string(), 1) - 1);

}