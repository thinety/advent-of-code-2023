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

fn settle_blocks(blocks: &mut [Block]) {
    blocks.sort_by_key(|block| block.z1);

    for i in 0..blocks.len() {
        let mut new_z1 = 1;
        for j in 0..i {
            if blocks[i].x1 <= blocks[j].x2
                && blocks[i].x2 >= blocks[j].x1
                && blocks[i].y1 <= blocks[j].y2
                && blocks[i].y2 >= blocks[j].y1
            {
                new_z1 = new_z1.max(blocks[j].z2 + 1);
            }
        }

        let diff = blocks[i].z1 - new_z1;
        blocks[i].z1 -= diff;
        blocks[i].z2 -= diff;
    }
}

// last node is ground
fn get_top_blocks(blocks: &[Block]) -> Vec<Vec<usize>> {
    let n = blocks.len();

    let mut top_blocks = vec![Vec::new(); n + 1];

    for i in 0..n {
        if blocks[i].z1 == 1 {
            top_blocks[n].push(i);
        }
        for j in 0..n {
            if blocks[i].z2 + 1 == blocks[j].z1
                && blocks[i].x1 <= blocks[j].x2
                && blocks[i].x2 >= blocks[j].x1
                && blocks[i].y1 <= blocks[j].y2
                && blocks[i].y2 >= blocks[j].y1
            {
                top_blocks[i].push(j);
            }
        }
    }

    top_blocks
}

fn dfs(i: usize, neighbors: &[Vec<usize>], visited: &mut [bool]) {
    for &ni in &neighbors[i] {
        if visited[ni] {
            continue;
        }
        visited[ni] = true;
        dfs(ni, neighbors, visited);
    }
}

fn solve(input: &str) -> (u32, u32) {
    let mut blocks = parse(input);
    let n = blocks.len();

    settle_blocks(&mut blocks);
    let top_blocks = get_top_blocks(&blocks);

    let (mut ans1, mut ans2) = (0, 0);
    for i in 0..n {
        let mut visited = vec![false; n+1];
        visited[i] = true;
        dfs(n, &top_blocks, &mut visited);

        let mut is_articulation = false;
        for i in 0..n {
            if !visited[i] {
                is_articulation = true;
                ans2 += 1;
            }
        }
        if !is_articulation {
            ans1 += 1;
        }
    }
    (ans1, ans2)
}

pub fn part1(input: &str) -> u32 {
    let (ans, _) = solve(input);
    ans
}

pub fn part2(input: &str) -> u32 {
    let (_, ans) = solve(input);
    ans
}

crate::samples! {
    (part1_sample, part1, "sample.in", 5),
    (part1_puzzle, part1, "puzzle.in", 407),
    (part2_sample, part2, "sample.in", 7),
    (part2_puzzle, part2, "puzzle.in", 59266),
}
