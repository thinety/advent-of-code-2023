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

fn get_bottom_blocks(blocks: &[Block]) -> Vec<Vec<usize>> {
    let n = blocks.len();

    let mut bottom_blocks = vec![Vec::new(); n];

    for i in 0..n {
        for j in 0..n {
            if blocks[j].z2 + 1 == blocks[i].z1
                && blocks[j].x1 <= blocks[i].x2
                && blocks[j].x2 >= blocks[i].x1
                && blocks[j].y1 <= blocks[i].y2
                && blocks[j].y2 >= blocks[i].y1
            {
                bottom_blocks[i].push(j);
            }
        }
    }

    bottom_blocks
}

fn get_top_blocks(blocks: &[Block]) -> (Vec<usize>, Vec<Vec<usize>>) {
    let n = blocks.len();

    let mut ground_blocks = Vec::new();
    let mut top_blocks = vec![Vec::new(); n];

    for i in 0..n {
        if blocks[i].z1 == 1 {
            ground_blocks.push(i);
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

    (ground_blocks, top_blocks)
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

pub fn part1(input: &str) -> u32 {
    let mut blocks = parse(input);
    let n = blocks.len();

    settle_blocks(&mut blocks);
    let bottom_blocks = get_bottom_blocks(&blocks);

    let mut fundamental_block = vec![false; n];
    for i in 0..n {
        if bottom_blocks[i].len() == 1 {
            fundamental_block[bottom_blocks[i][0]] = true;
        }
    }
    fundamental_block
        .iter()
        .map(|&fundamental| if fundamental { 0 } else { 1 })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let mut blocks = parse(input);
    let n = blocks.len();

    settle_blocks(&mut blocks);
    let (ground_blocks, top_blocks) = get_top_blocks(&blocks);

    let mut ans = 0;
    let mut visited = vec![false; n];
    for i in 0..n {
        visited[i] = true;
        for &gi in &ground_blocks {
            if visited[gi] {
                continue;
            }
            visited[gi] = true;
            dfs(gi, &top_blocks, &mut visited);
        }
        for i in 0..n {
            ans += if visited[i] { 0 } else { 1 };
            visited[i] = false;
        }
    }
    ans
}

crate::samples! {
    (part1_sample, part1, "sample.in", 5),
    (part1_puzzle, part1, "puzzle.in", 407),
    (part2_sample, part2, "sample.in", 7),
    (part2_puzzle, part2, "puzzle.in", 59266),
}
