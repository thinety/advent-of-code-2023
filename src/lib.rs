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
        $( ($func:ident, $name:ident, $input:literal, $answer:literal) ),* $(,)?
    ) => {
        $(
            #[cfg(test)]
            #[test]
            fn $name() {
                let input = include_str!($input);
                let output = format!("{}\n", $func(input));

                let answer = include_str!($answer);

                assert_eq!(output, answer);
            }
        )*
    };
}
