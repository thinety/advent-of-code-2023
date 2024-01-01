fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn neighbor_count((i, j): (usize, usize), grid: &Vec<Vec<u8>>) -> u32 {
    let (n, m) = (grid.len(), grid[0].len());

    let mut neighbor_count = 0;

    if i > 0 && grid[i - 1][j] != b'#' {
        neighbor_count += 1;
    }
    if i < n - 1 && grid[i + 1][j] != b'#' {
        neighbor_count += 1;
    }
    if j > 0 && grid[i][j - 1] != b'#' {
        neighbor_count += 1;
    }
    if j < m - 1 && grid[i][j + 1] != b'#' {
        neighbor_count += 1;
    }

    neighbor_count
}

// 0 is start, 1 is end
fn make_graph(grid: &Vec<Vec<u8>>) -> Vec<Vec<(usize, u32)>> {
    let (n, m) = (grid.len(), grid[0].len());

    let mut relevant = Vec::new();
    let mut index = vec![vec![None; m]; n];
    let mut neighbors = Vec::new();

    relevant.push((0, 1));
    index[0][1] = Some(neighbors.len());
    neighbors.push(Vec::new());

    relevant.push((n - 1, m - 2));
    index[n - 1][m - 2] = Some(neighbors.len());
    neighbors.push(Vec::new());

    while let Some((i, j)) = relevant.pop() {
        fn dfs(
            (idx, i, j): (usize, usize, usize),
            d: u32,
            grid: &Vec<Vec<u8>>,
            visited: &mut Vec<Vec<bool>>,
            relevant: &mut Vec<(usize, usize)>,
            index: &mut Vec<Vec<Option<usize>>>,
            neighbors: &mut Vec<Vec<(usize, u32)>>,
        ) {
            let (n, m) = (grid.len(), grid[0].len());

            let mut f = |ni: usize, nj: usize| {
                if grid[ni][nj] == b'#' || visited[ni][nj] {
                    return;
                }
                visited[ni][nj] = true;

                if neighbor_count((ni, nj), grid) != 2 {
                    assert!(grid[ni][nj] == b'.');

                    if index[ni][nj].is_none() {
                        relevant.push((ni, nj));
                        index[ni][nj] = Some(neighbors.len());
                        neighbors.push(Vec::new());
                    }

                    let neighbor_idx = index[ni][nj].unwrap();
                    neighbors[idx].push((neighbor_idx, d + 1));
                    return;
                }

                dfs(
                    (idx, ni, nj),
                    d + 1,
                    grid,
                    visited,
                    relevant,
                    index,
                    neighbors,
                );
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

        let idx = index[i][j].unwrap();

        let mut visited = vec![vec![false; m]; n];
        visited[i][j] = true;

        dfs(
            (idx, i, j),
            0,
            grid,
            &mut visited,
            &mut relevant,
            &mut index,
            &mut neighbors,
        );
    }

    neighbors
}

fn dfs(
    i: usize,
    neighbors: &Vec<Vec<(usize, u32)>>,
    visited: &mut Vec<bool>,
    distance: &mut Vec<u32>,
    ans: &mut u32,
) {
    let d = distance[i];

    if i == 1 {
        *ans = (*ans).max(d);
    }

    for &(ni, ec) in neighbors[i].iter() {
        if visited[ni] {
            continue;
        }
        visited[ni] = true;
        distance[ni] = d + ec;
        dfs(ni, neighbors, visited, distance, ans);
        visited[ni] = false;
    }
}

pub fn part1(input: &str) -> u32 {
    let grid = parse(input);

    let neighbors = make_graph(&grid);
    let n = neighbors.len();

    let mut ans = 0;

    let mut visited = vec![false; n];
    let mut distance = vec![0; n];

    visited[0] = true;
    distance[0] = 0;
    dfs(0, &neighbors, &mut visited, &mut distance, &mut ans);

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

    let mut visited = vec![false; n];
    let mut distance = vec![0; n];

    visited[0] = true;
    distance[0] = 0;
    dfs(0, &neighbors, &mut visited, &mut distance, &mut ans);

    ans
}

crate::samples! {
    (part1_sample, part1, "sample.in", 94),
    (part1_puzzle, part1, "puzzle.in", 2110),
    (part2_sample, part2, "sample.in", 154),
    (part2_puzzle, part2, "puzzle.in", 6514),
}
