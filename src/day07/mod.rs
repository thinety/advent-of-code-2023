mod part1;
pub use part1::part1;

mod part2;
pub use part2::part2;

crate::samples! {
    (part1, part1_sample, "sample.in", "sample.out1"),
    (part1, part1_puzzle, "puzzle.in", "puzzle.out1"),
    (part2, part2_sample, "sample.in", "sample.out2"),
    (part2, part2_puzzle, "puzzle.in", "puzzle.out2"),
}
