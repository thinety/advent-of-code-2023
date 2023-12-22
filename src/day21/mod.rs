fn parse(input: &str) -> ((usize, usize), Vec<Vec<u8>>) {
    let mut input = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let n = input.len();
    let m = input[0].len();

    let mut start = None;
    for i in 0..n {
        for j in 0..m {
            if input[i][j] == b'S' {
                input[i][j] = b'.';
                assert!(start.replace((i, j)).is_none());
            }
        }
    }
    let start = start.unwrap();

    (start, input)
}

fn expand(
    start: (usize, usize),
    grid: &Vec<Vec<u8>>,
    factor: usize,
) -> ((usize, usize), Vec<Vec<u8>>) {
    let (i, j) = start;
    let (n, m) = (grid.len(), grid[0].len());

    let (new_i, new_j) = (factor * n + i, factor * m + j);
    let new_grid = grid
        .iter()
        .map(|line| {
            line.iter()
                .copied()
                .cycle()
                .take((2 * factor + 1) * m)
                .collect::<Vec<_>>()
        })
        .cycle()
        .take((2 * factor + 1) * n)
        .collect::<Vec<_>>();

    ((new_i, new_j), new_grid)
}

pub fn solve(start: (usize, usize), grid: &Vec<Vec<u8>>, iterations: usize) -> Vec<usize> {
    let (i, j) = start;
    let (n, m) = (grid.len(), grid[0].len());

    let mut visited = vec![vec![false; m]; n];
    visited[i][j] = true;

    let mut positions = vec![start];
    let mut ans = vec![1];

    for _ in 0..iterations {
        let mut new_positions = Vec::new();
        for (i, j) in positions {
            if i > 0 && grid[i - 1][j] == b'.' && !visited[i - 1][j] {
                visited[i - 1][j] = true;
                new_positions.push((i - 1, j));
            }
            if i < n - 1 && grid[i + 1][j] == b'.' && !visited[i + 1][j] {
                visited[i + 1][j] = true;
                new_positions.push((i + 1, j));
            }
            if j > 0 && grid[i][j - 1] == b'.' && !visited[i][j - 1] {
                visited[i][j - 1] = true;
                new_positions.push((i, j - 1));
            }
            if j < m - 1 && grid[i][j + 1] == b'.' && !visited[i][j + 1] {
                visited[i][j + 1] = true;
                new_positions.push((i, j + 1));
            }
        }
        positions = new_positions;
        let n = ans.len();
        if n < 2 {
            ans.push(0);
        } else {
            ans.push(ans[n - 2]);
        }
        ans[n] += positions.len();
    }

    ans
}

pub fn part1(input: &str) -> usize {
    let (start, grid) = parse(input);
    let ans = solve(start, &grid, 64);
    *ans.last().unwrap()
}

pub fn part2(input: &str) -> Vec<usize> {
    let (start, grid) = parse(input);
    let (start, grid) = expand(start, &grid, 10);
    solve(start, &grid, grid.len() / 2)
}

crate::samples! {
    (part1_puzzle, part1, "puzzle.in", 3853),
}
