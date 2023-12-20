use std::io::Read;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    problem: Problem,
}

macro_rules! problems {
    ( $(($name:ident, $func:path)),* $(,)? ) => {
        #[derive(Subcommand)]
        enum Problem {
            $(
                $name,
            )*
        }

        impl Problem {
            fn run(&self, input: &str) {
                use Problem::*;
                match self {
                    $(
                        $name => {
                            println!("{}", $func(input));
                        }
                    )*
                }
            }
        }
    };
}

problems! {
    (Day01Part1, aoc2023::day01::part1),
    (Day01Part2, aoc2023::day01::part2),
    (Day02Part1, aoc2023::day02::part1),
    (Day02Part2, aoc2023::day02::part2),
    (Day03Part1, aoc2023::day03::part1),
    (Day03Part2, aoc2023::day03::part2),
    (Day04Part1, aoc2023::day04::part1),
    (Day04Part2, aoc2023::day04::part2),
    (Day05Part1, aoc2023::day05::part1),
    (Day05Part2, aoc2023::day05::part2),
    (Day06Part1, aoc2023::day06::part1),
    (Day06Part2, aoc2023::day06::part2),
    (Day07Part1, aoc2023::day07::part1),
    (Day07Part2, aoc2023::day07::part2),
    (Day08Part1, aoc2023::day08::part1),
    (Day08Part2, aoc2023::day08::part2),
    (Day09Part1, aoc2023::day09::part1),
    (Day09Part2, aoc2023::day09::part2),
    (Day10Part1, aoc2023::day10::part1),
    (Day10Part2, aoc2023::day10::part2),
    (Day11Part1, aoc2023::day11::part1),
    (Day11Part2, aoc2023::day11::part2),
    (Day12Part1, aoc2023::day12::part1),
    (Day12Part2, aoc2023::day12::part2),
    (Day13Part1, aoc2023::day13::part1),
    (Day13Part2, aoc2023::day13::part2),
    (Day14Part1, aoc2023::day14::part1),
    (Day14Part2, aoc2023::day14::part2),
    (Day15Part1, aoc2023::day15::part1),
    (Day15Part2, aoc2023::day15::part2),
    (Day16Part1, aoc2023::day16::part1),
    (Day16Part2, aoc2023::day16::part2),
    (Day17Part1, aoc2023::day17::part1),
    (Day17Part2, aoc2023::day17::part2),
    (Day18Part1, aoc2023::day18::part1),
    (Day18Part2, aoc2023::day18::part2),
    (Day19Part1, aoc2023::day19::part1),
    (Day19Part2, aoc2023::day19::part2),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let problem = cli.problem;

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    problem.run(&input);

    Ok(())
}
