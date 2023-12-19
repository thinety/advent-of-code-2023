#[derive(Clone, Copy)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn solve(input: &Vec<Vec<u8>>, start: (usize, usize, Direction)) -> u32 {
    let n = input.len();
    let m = input[0].len();

    let mut visited = vec![vec![[false; 4]; m]; n];

    let mut stack = Vec::new();
    stack.push(start);

    while let Some((i, j, direction)) = stack.pop() {
        if visited[i][j][direction as usize] {
            continue;
        }
        visited[i][j][direction as usize] = true;

        match direction {
            Direction::Up => {
                if ((input[i][j] == b'/') || (input[i][j] == b'-')) && j < m - 1 {
                    stack.push((i, j + 1, Direction::Right));
                }
                if ((input[i][j] == b'\\') || (input[i][j] == b'-')) && j > 0 {
                    stack.push((i, j - 1, Direction::Left));
                }
                if ((input[i][j] == b'.') || (input[i][j] == b'|')) && i > 0 {
                    stack.push((i - 1, j, Direction::Up));
                }
            }
            Direction::Down => {
                if ((input[i][j] == b'/') || (input[i][j] == b'-')) && j > 0 {
                    stack.push((i, j - 1, Direction::Left));
                }
                if ((input[i][j] == b'\\') || (input[i][j] == b'-')) && j < m - 1 {
                    stack.push((i, j + 1, Direction::Right));
                }
                if ((input[i][j] == b'.') || (input[i][j] == b'|')) && i < n - 1 {
                    stack.push((i + 1, j, Direction::Down));
                }
            }
            Direction::Left => {
                if ((input[i][j] == b'/') || (input[i][j] == b'|')) && i < n - 1 {
                    stack.push((i + 1, j, Direction::Down));
                }
                if ((input[i][j] == b'\\') || (input[i][j] == b'|')) && i > 0 {
                    stack.push((i - 1, j, Direction::Up));
                }
                if ((input[i][j] == b'.') || (input[i][j] == b'-')) && j > 0 {
                    stack.push((i, j - 1, Direction::Left));
                }
            }
            Direction::Right => {
                if ((input[i][j] == b'/') || (input[i][j] == b'|')) && i > 0 {
                    stack.push((i - 1, j, Direction::Up));
                }
                if ((input[i][j] == b'\\') || (input[i][j] == b'|')) && i < n - 1 {
                    stack.push((i + 1, j, Direction::Down));
                }
                if ((input[i][j] == b'.') || (input[i][j] == b'-')) && j < m - 1 {
                    stack.push((i, j + 1, Direction::Right));
                }
            }
        }
    }

    visited
        .into_iter()
        .flat_map(|line| {
            line.into_iter()
                .filter_map(|tile| tile.into_iter().any(|x| x).then_some(1))
        })
        .sum()
}

pub fn part1(input: &str) -> u32 {
    let input = parse(input);
    solve(&input, (0, 0, Direction::Right))
}

pub fn part2(input: &str) -> u32 {
    let input = parse(input);

    let n = input.len();
    let m = input[0].len();

    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(solve(&input, (i, 0, Direction::Right)));
        ans = ans.max(solve(&input, (i, m - 1, Direction::Left)));
    }
    for j in 0..m {
        ans = ans.max(solve(&input, (0, j, Direction::Down)));
        ans = ans.max(solve(&input, (n - 1, j, Direction::Up)));
    }
    ans
}

crate::samples! {
    (part1_sample, part1, "sample.in", 46),
    (part1_puzzle, part1, "puzzle.in", 7860),
    (part2_sample, part2, "sample.in", 51),
    (part2_puzzle, part2, "puzzle.in", 8331),
}
