use std::collections::HashSet;

struct Block {
    x1: u32,
    x2: u32,
    y1: u32,
    y2: u32,
    z1: u32,
    z2: u32,
}

fn parse(input: &str) -> Vec<Block> {
    input
        .lines()
        .map(|line| {
            let (p1, p2) = line.split_once('~').unwrap();

            let mut p1 = p1.split(',');
            let (x1, y1, z1) = (p1.next().unwrap(), p1.next().unwrap(), p1.next().unwrap());
            assert!(p1.next().is_none());

            let mut p2 = p2.split(',');
            let (x2, y2, z2) = (p2.next().unwrap(), p2.next().unwrap(), p2.next().unwrap());
            assert!(p2.next().is_none());

            Block {
                x1: x1.parse().unwrap(),
                x2: x2.parse().unwrap(),
                y1: y1.parse().unwrap(),
                y2: y2.parse().unwrap(),
                z1: z1.parse().unwrap(),
                z2: z2.parse().unwrap(),
            }
        })
        .collect()
}

// last node is ground
fn get_connections(blocks: &mut [Block]) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let n = blocks.len();

    let (mut max_x, mut max_y) = (0, 0);
    for block in blocks.iter_mut() {
        (block.x1, block.x2) = (block.x1.min(block.x2), block.x1.max(block.x2));
        (block.y1, block.y2) = (block.y1.min(block.y2), block.y1.max(block.y2));
        (block.z1, block.z2) = (block.z1.min(block.z2), block.z1.max(block.z2));
        (max_x, max_y) = (max_x.max(block.x2 as usize), max_y.max(block.y2 as usize));
    }
    blocks.sort_by_key(|block| block.z1);

    let mut bot = vec![Vec::new(); n + 1];
    let mut top = vec![Vec::new(); n + 1];

    let mut heights = vec![vec![1; max_y + 1]; max_x + 1];
    let mut indices = vec![vec![n; max_y + 1]; max_x + 1];
    for i in 0..n {
        let mut z1 = 0;
        for x in blocks[i].x1..=blocks[i].x2 {
            for y in blocks[i].y1..=blocks[i].y2 {
                z1 = z1.max(heights[x as usize][y as usize]);
            }
        }
        let z2 = blocks[i].z2 - blocks[i].z1 + z1;

        let mut set = HashSet::new();
        for x in blocks[i].x1..=blocks[i].x2 {
            for y in blocks[i].y1..=blocks[i].y2 {
                if heights[x as usize][y as usize] == z1 {
                    set.insert(indices[x as usize][y as usize]);
                }
                heights[x as usize][y as usize] = z2 + 1;
                indices[x as usize][y as usize] = i;
            }
        }
        bot[i].extend(&set);
        for &j in &set {
            top[j].push(i);
        }
    }

    (bot, top)
}

fn lca(mut u: usize, mut v: usize, depth: &[u32], up: &[Vec<usize>]) -> usize {
    let l = up[0].len();

    if depth[u] > depth[v] {
        (u, v) = (v, u);
    }

    let diff = depth[v] - depth[u];
    for i in (0..l).rev() {
        if (diff & (1 << i)) != 0 {
            v = up[v][i];
        }
    }

    if v == u {
        return u;
    }

    for i in (0..l).rev() {
        if up[u][i] != up[v][i] {
            u = up[u][i];
            v = up[v][i];
        }
    }

    up[u][0]
}

fn make_tree(
    i: usize,
    top: &[Vec<usize>],
    bot: &[Vec<usize>],
    visited: &mut [bool],
    children: &mut [Vec<usize>],
    depth: &mut [u32],
    up: &mut [Vec<usize>],
) {
    let l = up[0].len();

    for &ni in top[i].iter() {
        if visited[ni] {
            continue;
        }
        if bot[ni].iter().any(|&j| !visited[j]) {
            continue;
        }

        visited[ni] = true;

        let mut parent = bot[ni][0];
        for &j in bot[ni][1..].iter() {
            parent = lca(parent, j, depth, up);
        }

        children[parent].push(ni);
        depth[ni] = depth[parent] + 1;
        up[ni][0] = parent;
        for i in 1..l {
            up[ni][i] = up[up[ni][i - 1]][i - 1];
        }

        make_tree(ni, top, bot, visited, children, depth, up);
    }
}

fn dfs(i: usize, children: &[Vec<usize>], visited: &mut [bool], subtree: &mut [u32]) {
    for &ni in children[i].iter() {
        if visited[ni] {
            continue;
        }
        visited[ni] = true;
        subtree[ni] = 1;

        dfs(ni, children, visited, subtree);

        subtree[i] += subtree[ni];
    }
}

pub fn part1(input: &str) -> u32 {
    let mut blocks = parse(input);
    let (bot, _) = get_connections(&mut blocks);

    let n = blocks.len();

    let mut safe = vec![true; n + 1];
    for i in 0..n {
        if bot[i].len() == 1 {
            let pi = bot[i][0];
            safe[pi] = false;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        if safe[i] {
            ans += 1;
        }
    }
    ans
}

pub fn part2(input: &str) -> u32 {
    let mut blocks = parse(input);
    let (bot, top) = get_connections(&mut blocks);

    let n = blocks.len();

    // if V is the number of nodes in the tree, we have
    // L = ceil(log2(V)) = floor(log2(V-1))+1
    // each block is one node, plus one more for the ground, so V = n+1
    let l = (n.ilog2() + 1) as usize;

    let mut children = vec![Vec::new(); n + 1];
    {
        let mut visited = vec![false; n + 1];
        let mut depth = vec![0; n + 1];
        let mut up = vec![vec![0; l]; n + 1];

        visited[n] = true;
        depth[n] = 0;
        for i in 0..l {
            up[n][i] = n;
        }

        make_tree(
            n,
            &top,
            &bot,
            &mut visited,
            &mut children,
            &mut depth,
            &mut up,
        );
    }

    let mut subtree = vec![0; n + 1];
    {
        let mut visited = vec![false; n + 1];

        visited[n] = true;
        subtree[n] = 1;

        dfs(n, &children, &mut visited, &mut subtree);
    }

    let mut ans = 0;
    for i in 0..n {
        ans += subtree[i] - 1;
    }
    ans
}

crate::samples! {
    (part1_sample, part1, "sample.in", 5),
    (part1_puzzle, part1, "puzzle.in", 407),
    (part2_sample, part2, "sample.in", 7),
    (part2_puzzle, part2, "puzzle.in", 59266),
}
