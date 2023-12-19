use criterion::black_box;
use criterion::Criterion;

macro_rules! bench {
    ($c:expr, $day:literal, $part: literal, $func:path) => {
        let input = include_str!(concat!("../src/", $day, "/puzzle.in"));
        $c.bench_function(concat!($day, "-", $part), |b| {
            b.iter(|| $func(black_box(input)))
        });
    };
}

fn main() {
    let mut criterion = Criterion::default().configure_from_args();

    bench!(criterion, "day01", "part1", aoc2023::day01::part1);
    bench!(criterion, "day01", "part2", aoc2023::day01::part2);
    bench!(criterion, "day02", "part1", aoc2023::day02::part1);
    bench!(criterion, "day02", "part2", aoc2023::day02::part2);
    bench!(criterion, "day03", "part1", aoc2023::day03::part1);
    bench!(criterion, "day03", "part2", aoc2023::day03::part2);
    bench!(criterion, "day04", "part1", aoc2023::day04::part1);
    bench!(criterion, "day04", "part2", aoc2023::day04::part2);
    bench!(criterion, "day05", "part1", aoc2023::day05::part1);
    bench!(criterion, "day05", "part2", aoc2023::day05::part2);
    bench!(criterion, "day06", "part1", aoc2023::day06::part1);
    bench!(criterion, "day06", "part2", aoc2023::day06::part2);
    bench!(criterion, "day07", "part1", aoc2023::day07::part1);
    bench!(criterion, "day07", "part2", aoc2023::day07::part2);
    bench!(criterion, "day08", "part1", aoc2023::day08::part1);
    bench!(criterion, "day08", "part2", aoc2023::day08::part2);
    bench!(criterion, "day09", "part1", aoc2023::day09::part1);
    bench!(criterion, "day09", "part2", aoc2023::day09::part2);
    bench!(criterion, "day10", "part1", aoc2023::day10::part1);
    bench!(criterion, "day10", "part2", aoc2023::day10::part2);
    bench!(criterion, "day11", "part1", aoc2023::day11::part1);
    bench!(criterion, "day11", "part2", aoc2023::day11::part2);
    bench!(criterion, "day12", "part1", aoc2023::day12::part1);
    bench!(criterion, "day12", "part2", aoc2023::day12::part2);
    bench!(criterion, "day13", "part1", aoc2023::day13::part1);
    bench!(criterion, "day13", "part2", aoc2023::day13::part2);
    bench!(criterion, "day14", "part1", aoc2023::day14::part1);
    bench!(criterion, "day14", "part2", aoc2023::day14::part2);
    bench!(criterion, "day15", "part1", aoc2023::day15::part1);
    bench!(criterion, "day15", "part2", aoc2023::day15::part2);
    bench!(criterion, "day16", "part1", aoc2023::day16::part1);
    bench!(criterion, "day16", "part2", aoc2023::day16::part2);
    bench!(criterion, "day17", "part1", aoc2023::day17::part1);
    bench!(criterion, "day17", "part2", aoc2023::day17::part2);
    bench!(criterion, "day18", "part1", aoc2023::day18::part1);
    bench!(criterion, "day18", "part2", aoc2023::day18::part2);

    criterion.final_summary();
}
