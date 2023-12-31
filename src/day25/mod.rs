use std::collections::HashMap;

fn parse(input: &str) -> Vec<Vec<usize>> {
    let mut indices = HashMap::new();
    let mut neighbors = Vec::new();

    for line in input.lines() {
        let (src, dsts) = line.split_once(": ").unwrap();
        let src_index = *indices.entry(src).or_insert_with(|| {
            neighbors.push(Vec::new());
            neighbors.len() - 1
        });
        for dst in dsts.split(' ') {
            let dst_index = *indices.entry(dst).or_insert_with(|| {
                neighbors.push(Vec::new());
                neighbors.len() - 1
            });
            neighbors[src_index].push(dst_index);
            neighbors[dst_index].push(src_index);
        }
    }

    neighbors
}

// straightforward implementation of Stoer-Wagner
// (https://dl.acm.org/doi/pdf/10.1145/263867.263872)
//
// runs in O(V^3), which is optimal for dense graphs (E = V^2), but not for
// sparse ones. the implementation for the sparse case needs a priority queue
// (no need for fibonacci heap, a binary heap should be enough), but I'll leave
// that as an exercise to the reader.
fn minimum_cut_phase(
    g: &mut Vec<Vec<u32>>,
    cuts: &mut Vec<Vec<usize>>,
    merged: &mut Vec<bool>,
    nv: &mut usize,
    a: usize,
) -> (usize, u32) {
    let n = g.len();

    let mut A = vec![false; n];
    let mut w = vec![0; n];

    // s = n just to satisfy compiler, the for loop is guaranteed to run at least once
    let mut s = n;
    let mut t = a;

    for _ in 1..*nv {
        A[t] = true;
        for i in 0..n {
            w[i] += g[t][i];
        }

        s = t;
        t = (0..n)
            .filter(|&i| !merged[i] && !A[i])
            .max_by_key(|&i| w[i])
            .unwrap();
    }

    for i in 0..n {
        g[s][i] += g[t][i];
        g[i][s] += g[i][t];
    }

    // technically we don't need to clone here, since s and t are guaranteeded
    // to be different, but I've tried some unsafe code and it consistenly ran
    // around 5% slower than cloning. compilers are magic
    let old_cut = cuts[t].clone();
    cuts[s].extend(old_cut);

    merged[t] = true;

    *nv -= 1;

    (t, w[t])
}

fn minimum_cut(
    g: &mut Vec<Vec<u32>>,
    cuts: &mut Vec<Vec<usize>>,
    merged: &mut Vec<bool>,
    nv: &mut usize,
    a: usize,
) -> (usize, u32) {
    let (mut t, mut c) = minimum_cut_phase(g, cuts, merged, nv, a);

    while *nv > 1 {
        let (new_t, new_c) = minimum_cut_phase(g, cuts, merged, nv, a);
        if new_c < c {
            (t, c) = (new_t, new_c);
        }
    }

    (t, c)
}

fn solve(neighbors: &[Vec<usize>]) -> usize {
    let n = neighbors.len();

    let mut adjacency_matrix = vec![vec![0; n]; n];
    for i in 0..n {
        for &ni in &neighbors[i] {
            adjacency_matrix[i][ni] += 1;
        }
    }

    let mut cuts = vec![vec![0]; n];
    for i in 0..n {
        cuts[i][0] = i;
    }

    let mut merged = vec![false; n];

    let mut nv = n;

    let (t, _) = minimum_cut(&mut adjacency_matrix, &mut cuts, &mut merged, &mut nv, 0);

    let m = cuts[t].len();

    m * (n - m)
}

pub fn part1(input: &str) -> usize {
    let neighbors = parse(input);
    solve(&neighbors)
}

crate::samples! {
    (part1_sample, part1, "sample.in", 54),
    (part1_puzzle, part1, "puzzle.in", 545528),
}
