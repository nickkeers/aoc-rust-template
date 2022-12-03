use aoc_helpers::{AocSession, Day};
use std::{convert::Infallible, str::FromStr};
use anyhow::Result;

#[derive(Debug, Clone, Copy)]
enum DayType {
    Something
}

#[derive(Debug, Clone, Copy)]
struct Command {
    val: DayType
}

impl FromStr for Command {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ => Ok(Command{val: DayType::Something})
        }
    }
}

struct Day{{day}}(Vec<Command>);

impl Day for Day{{day}} {
    const DAY: u8 = {{day}};

    fn from_input(input: String) -> Self {
        let lines: Vec<Command> = input.trim().split('\n').map(|line| {
            Command::from_str(line).unwrap()
        }).collect();

        return Self(lines);
    }

    fn first_part(&mut self) -> String {
        println!("do something with {:?}", self.0[0].val);
        unimplemented!()
    }

    fn second_part(&mut self) -> String {
        unimplemented!()
    }
}


fn main() -> Result<(), anyhow::Error> {
    AocSession::new(2022)?.day::<Day{{day}}>();
    Ok(())
}
