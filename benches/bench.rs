#![feature(test)]
extern crate test;

use test::black_box;
use test::Bencher;

macro_rules! bench {
    ($name:ident, $func:path, $input:literal) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let input = include_str!($input);
            b.iter(|| black_box($func(black_box(input))));
        }
    };
}

bench! { day01_part1, aoc2023::day01::part1, "../src/day01/puzzle.in" }
bench! { day01_part2, aoc2023::day01::part2, "../src/day01/puzzle.in" }
bench! { day02_part1, aoc2023::day02::part1, "../src/day02/puzzle.in" }
bench! { day02_part2, aoc2023::day02::part2, "../src/day02/puzzle.in" }
bench! { day03_part1, aoc2023::day03::part1, "../src/day03/puzzle.in" }
bench! { day03_part2, aoc2023::day03::part2, "../src/day03/puzzle.in" }
bench! { day04_part1, aoc2023::day04::part1, "../src/day04/puzzle.in" }
bench! { day04_part2, aoc2023::day04::part2, "../src/day04/puzzle.in" }
bench! { day05_part1, aoc2023::day05::part1, "../src/day05/puzzle.in" }
bench! { day05_part2, aoc2023::day05::part2, "../src/day05/puzzle.in" }
bench! { day06_part1, aoc2023::day06::part1, "../src/day06/puzzle.in" }
bench! { day06_part2, aoc2023::day06::part2, "../src/day06/puzzle.in" }
bench! { day07_part1, aoc2023::day07::part1, "../src/day07/puzzle.in" }
bench! { day07_part2, aoc2023::day07::part2, "../src/day07/puzzle.in" }
bench! { day08_part1, aoc2023::day08::part1, "../src/day08/puzzle.in" }
bench! { day08_part2, aoc2023::day08::part2, "../src/day08/puzzle.in" }
bench! { day09_part1, aoc2023::day09::part1, "../src/day09/puzzle.in" }
bench! { day09_part2, aoc2023::day09::part2, "../src/day09/puzzle.in" }
bench! { day10_part1, aoc2023::day10::part1, "../src/day10/puzzle.in" }
bench! { day10_part2, aoc2023::day10::part2, "../src/day10/puzzle.in" }
bench! { day11_part1, aoc2023::day11::part1, "../src/day11/puzzle.in" }
bench! { day11_part2, aoc2023::day11::part2, "../src/day11/puzzle.in" }
bench! { day12_part1, aoc2023::day12::part1, "../src/day12/puzzle.in" }
bench! { day12_part2, aoc2023::day12::part2, "../src/day12/puzzle.in" }
bench! { day13_part1, aoc2023::day13::part1, "../src/day13/puzzle.in" }
bench! { day13_part2, aoc2023::day13::part2, "../src/day13/puzzle.in" }
bench! { day14_part1, aoc2023::day14::part1, "../src/day14/puzzle.in" }
bench! { day14_part2, aoc2023::day14::part2, "../src/day14/puzzle.in" }
bench! { day15_part1, aoc2023::day15::part1, "../src/day15/puzzle.in" }
bench! { day15_part2, aoc2023::day15::part2, "../src/day15/puzzle.in" }
bench! { day16_part1, aoc2023::day16::part1, "../src/day16/puzzle.in" }
bench! { day16_part2, aoc2023::day16::part2, "../src/day16/puzzle.in" }
bench! { day17_part1, aoc2023::day17::part1, "../src/day17/puzzle.in" }
bench! { day17_part2, aoc2023::day17::part2, "../src/day17/puzzle.in" }
bench! { day18_part1, aoc2023::day18::part1, "../src/day18/puzzle.in" }
bench! { day18_part2, aoc2023::day18::part2, "../src/day18/puzzle.in" }
bench! { day19_part1, aoc2023::day19::part1, "../src/day19/puzzle.in" }
bench! { day19_part2, aoc2023::day19::part2, "../src/day19/puzzle.in" }
bench! { day20_part1, aoc2023::day20::part1, "../src/day20/puzzle.in" }
bench! { day21_part1, aoc2023::day21::part1, "../src/day21/puzzle.in" }
bench! { day22_part1, aoc2023::day22::part1, "../src/day22/puzzle.in" }
bench! { day22_part2, aoc2023::day22::part2, "../src/day22/puzzle.in" }
bench! { day23_part1, aoc2023::day23::part1, "../src/day23/puzzle.in" }
bench! { day23_part2, aoc2023::day23::part2, "../src/day23/puzzle.in" }
