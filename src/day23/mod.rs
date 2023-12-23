fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn dfs((i, j): (usize, usize), grid: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>) -> Option<u32> {
    let (n, m) = (grid.len(), grid[0].len());

    if i == n - 1 && j == m - 2 {
        return Some(0);
    }

    let mut ans: Option<u32> = None;

    let mut f = |i: usize, j: usize| {
        if grid[i][j] != b'#' && !visited[i][j] {
            visited[i][j] = true;
            if let Some(d) = dfs((i, j), grid, visited) {
                ans = Some(ans.map_or(d+1, |ans| ans.max(d+1)));
            }
            visited[i][j] = false;
        }
    };
    if (grid[i][j] == b'.' || grid[i][j] == b'^') && i > 0 {
        f(i - 1, j);
    }
    if (grid[i][j] == b'.' || grid[i][j] == b'v') && i < n - 1 {
        f(i + 1, j);
    }
    if (grid[i][j] == b'.' || grid[i][j] == b'<') && j > 0 {
        f(i, j - 1);
    }
    if (grid[i][j] == b'.' || grid[i][j] == b'>') && j < m - 1 {
        f(i, j + 1);
    }

    ans
}

pub fn part1(input: &str) -> u32 {
    let grid = parse(input);
    let (n, m) = (grid.len(), grid[0].len());

    let mut visited = vec![vec![false; m]; n];
    visited[0][1] = true;
    dfs((0, 1), &grid, &mut visited).unwrap()
}

pub fn part2(input: &str) -> u32 {
    let mut grid = parse(input);
    let (n, m) = (grid.len(), grid[0].len());

    for i in 0..n {
        for j in 0..m {
            if let b'^' | b'v' | b'<' | b'>' = grid[i][j] {
                grid[i][j] = b'.';
            }
        }
    }

    let mut visited = vec![vec![false; m]; n];
    visited[0][1] = true;
    dfs((0, 1), &grid, &mut visited).unwrap()
}

crate::samples! {
    (part1_sample, part1, "sample.in", 94),
    (part1_puzzle, part1, "puzzle.in", 2110),
    (part2_sample, part2, "sample.in", 154),
    (part2_puzzle, part2, "puzzle.in", 6514),
}
