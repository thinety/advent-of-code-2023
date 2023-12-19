mod part1;
pub use part1::part1;

mod part2;
pub use part2::part2;

crate::samples! {
    (part1_sample, part1, "sample.in", 6440),
    (part1_puzzle, part1, "puzzle.in", 248453531),
    (part2_sample, part2, "sample.in", 5905),
    (part2_puzzle, part2, "puzzle.in", 248781813),
}
