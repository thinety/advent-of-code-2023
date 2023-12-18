use std::io::Read;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    println!("{}", aoc2023::day18::part2(&input));

    Ok(())
}
