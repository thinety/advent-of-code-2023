use super::*;

pub(super) fn parse(input: &str) -> Vec<Node> {
    let input = input.lines().map(|s| s.to_string()).collect::<Vec<_>>();
    let n = input.len();
    let m = input[0].len();

    let mut u = 0;
    let mut nodes = Vec::new();

    let mut mapping = vec![vec![0; m]; n];
    let mut reverse_mapping = Vec::new();

    for i in 0..n {
        let mut j = 0;
        while j < m {
            mapping[i][j] = u;

            if input[i].as_bytes()[j].is_ascii_digit() {
                let mut jj = j + 1;
                while jj < m {
                    if !input[i].as_bytes()[jj].is_ascii_digit() {
                        break;
                    }
                    mapping[i][jj] = u;
                    jj += 1;
                }

                reverse_mapping.push((i, j, jj));
                nodes.push(Node {
                    r#type: NodeType::Number(str::parse(&input[i][j..jj]).unwrap()),
                    neighbors: Vec::new(),
                });

                u += 1;
                j = jj;
            } else {
                reverse_mapping.push((i, j, j + 1));
                nodes.push(Node {
                    r#type: NodeType::Char(input[i].as_bytes()[j]),
                    neighbors: Vec::new(),
                });

                u += 1;
                j += 1;
            }
        }
    }

    let mut visited = vec![false; u];

    for (u, node) in nodes.iter_mut().enumerate() {
        let (i, j1, j2) = reverse_mapping[u];
        if i > 0 {
            if j1 > 0 {
                let v = mapping[i - 1][j1 - 1];
                if !visited[v] {
                    visited[v] = true;
                    node.neighbors.push(v);
                }
            }
            for j in j1..j2 {
                let v = mapping[i - 1][j];
                if !visited[v] {
                    visited[v] = true;
                    node.neighbors.push(v);
                }
            }
            if j2 < m {
                let v = mapping[i - 1][j2];
                if !visited[v] {
                    visited[v] = true;
                    node.neighbors.push(v);
                }
            }
        }
        if j1 > 0 {
            let v = mapping[i][j1 - 1];
            if !visited[v] {
                visited[v] = true;
                node.neighbors.push(v);
            }
        }
        if j2 < m {
            let v = mapping[i][j2];
            if !visited[v] {
                visited[v] = true;
                node.neighbors.push(v);
            }
        }
        if i + 1 < n {
            if j1 > 0 {
                let v = mapping[i + 1][j1 - 1];
                if !visited[v] {
                    visited[v] = true;
                    node.neighbors.push(v);
                }
            }
            for j in j1..j2 {
                let v = mapping[i + 1][j];
                if !visited[v] {
                    visited[v] = true;
                    node.neighbors.push(v);
                }
            }
            if j2 < m {
                let v = mapping[i + 1][j2];
                if !visited[v] {
                    visited[v] = true;
                    node.neighbors.push(v);
                }
            }
        }

        if i > 0 {
            if j1 > 0 {
                let v = mapping[i - 1][j1 - 1];
                visited[v] = false;
            }
            for j in j1..j2 {
                let v = mapping[i - 1][j];
                visited[v] = false;
            }
            if j2 < m {
                let v = mapping[i - 1][j2];
                visited[v] = false;
            }
        }
        if j1 > 0 {
            let v = mapping[i][j1 - 1];
            visited[v] = false;
        }
        if j2 < m {
            let v = mapping[i][j2];
            visited[v] = false;
        }
        if i + 1 < n {
            if j1 > 0 {
                let v = mapping[i + 1][j1 - 1];
                visited[v] = false;
            }
            for j in j1..j2 {
                let v = mapping[i + 1][j];
                visited[v] = false;
            }
            if j2 < m {
                let v = mapping[i + 1][j2];
                visited[v] = false;
            }
        }
    }

    nodes
}
