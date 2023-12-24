fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

// 0 is start, 1 is end
fn make_graph(grid: &Vec<Vec<u8>>) -> Vec<Vec<usize>> {
    fn dfs(
        (i, j): (usize, usize),
        grid: &Vec<Vec<u8>>,
        indexes: &mut Vec<Vec<Option<usize>>>,
        neighbors: &mut Vec<Vec<usize>>,
    ) {
        let (n, m) = (grid.len(), grid[0].len());
        let index = indexes[i][j].unwrap();

        let mut f = |ni: usize, nj: usize| {
            if grid[ni][nj] == b'#' {
                return;
            }

            let neighbor_index = indexes[ni][nj].unwrap_or_else(|| {
                neighbors.push(Vec::new());
                neighbors.len() - 1
            });
            neighbors[index].push(neighbor_index);

            if indexes[ni][nj].is_none() {
                indexes[ni][nj] = Some(neighbor_index);
                dfs((ni, nj), grid, indexes, neighbors);
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
    }

    let mut neighbors = vec![];

    let (n, m) = (grid.len(), grid[0].len());
    let mut indexes = vec![vec![None; m]; n];

    neighbors.push(Vec::new());
    indexes[0][1] = Some(neighbors.len() - 1);

    neighbors.push(Vec::new());
    indexes[n - 1][m - 2] = Some(neighbors.len() - 1);

    dfs((0, 1), grid, &mut indexes, &mut neighbors);
    dfs((n - 1, m - 2), grid, &mut indexes, &mut neighbors);

    neighbors
}

fn dfs(i: usize, neighbors: &Vec<Vec<usize>>, distance: &mut Vec<Option<u32>>, ans: &mut u32) {
    if i == 1 {
        *ans = (*ans).max(distance[i].unwrap());
    }

    for &ni in neighbors[i].iter() {
        if distance[ni].is_none() {
            distance[ni] = Some(distance[i].unwrap() + 1);
            dfs(ni, neighbors, distance, ans);
            distance[ni] = None;
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let grid = parse(input);

    let neighbors = make_graph(&grid);
    let n = neighbors.len();

    let mut ans = 0;
    let mut distance = vec![None; n];
    distance[0] = Some(0);
    dfs(0, &neighbors, &mut distance, &mut ans);
    ans
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

    let neighbors = make_graph(&grid);
    let n = neighbors.len();

    let mut ans = 0;
    let mut distance = vec![None; n];
    distance[0] = Some(0);
    dfs(0, &neighbors, &mut distance, &mut ans);
    ans
}

crate::samples! {
    (part1_sample, part1, "sample.in", 94),
    (part1_puzzle, part1, "puzzle.in", 2110),
    (part2_sample, part2, "sample.in", 154),
    (part2_puzzle, part2, "puzzle.in", 6514),
}
