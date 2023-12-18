pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;

#[macro_export]
macro_rules! samples {
    (
        $( ($name:ident, $func:expr, $input:literal, $answer:expr) ),* $(,)?
    ) => {
        $(
            #[cfg(test)]
            #[test]
            fn $name() {
                let input = include_str!($input);
                let output = format!("{}", $func(input));

                assert_eq!(output, $answer);
            }
        )*
    };
}
