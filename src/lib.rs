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
