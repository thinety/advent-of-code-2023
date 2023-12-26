struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

fn parse(input: &str) -> Vec<(Vec3, Vec3)> {
    input
        .lines()
        .map(|line| {
            let (position, velocity) = line.split_once(" @ ").unwrap();

            let mut position = position.split(", ");
            let (px, py, pz) = (
                position.next().unwrap(),
                position.next().unwrap(),
                position.next().unwrap(),
            );
            assert!(position.next().is_none());

            let mut velocity = velocity.split(", ");
            let (vx, vy, vz) = (
                velocity.next().unwrap(),
                velocity.next().unwrap(),
                velocity.next().unwrap(),
            );
            assert!(velocity.next().is_none());

            (
                Vec3 {
                    x: px.parse().unwrap(),
                    y: py.parse().unwrap(),
                    z: pz.parse().unwrap(),
                },
                Vec3 {
                    x: vx.parse().unwrap(),
                    y: vy.parse().unwrap(),
                    z: vz.parse().unwrap(),
                },
            )
        })
        .collect()
}

fn check_intersections(input: &[(Vec3, Vec3)], lower: i64, upper: i64) -> u32 {
    let mut ans = 0;

    let n = input.len();
    for i in 0..n {
        for j in i + 1..n {
            let (pi, vi) = &input[i];
            let (pj, vj) = &input[j];

            let mut num1 = (pj.y - pi.y) * vj.x - (pj.x - pi.x) * vj.y;
            let mut num2 = vi.x * (pj.y - pi.y) - vi.y * (pj.x - pi.x);
            let mut den = vj.x * vi.y - vi.x * vj.y;

            if den == 0 {
                continue;
            }
            if den < 0 {
                num1 = -num1;
                num2 = -num2;
                den = -den;
            }
            if num1 < 0 || num2 < 0 {
                continue;
            }

            let lower = (lower as i128) * (den as i128);
            let upper = (upper as i128) * (den as i128);
            let px = (pi.x as i128) * (den as i128) + (vi.x as i128) * (num1 as i128);
            let py = (pi.y as i128) * (den as i128) + (vi.y as i128) * (num1 as i128);

            if lower <= px && px <= upper && lower <= py && py <= upper {
                ans += 1;
            }
        }
    }

    ans
}

fn get_equations(input: &[(Vec3, Vec3)]) -> Vec<Vec<i64>> {
    let mut mat = Vec::new();

    for (i, j) in [(0, 1), (0, 2)] {
        let (pi, vi) = &input[i];
        let (pj, vj) = &input[j];
        mat.push(vec![
            0,
            vi.z - vj.z,
            vj.y - vi.y,
            0,
            pj.z - pi.z,
            pi.y - pj.y,
            pi.y * vi.z - pi.z * vi.y - pj.y * vj.z + pj.z * vj.y,
        ]);
        mat.push(vec![
            vj.z - vi.z,
            0,
            vi.x - vj.x,
            pi.z - pj.z,
            0,
            pj.x - pi.x,
            pi.z * vi.x - pi.x * vi.z - pj.z * vj.x + pj.x * vj.z,
        ]);
        mat.push(vec![
            vi.y - vj.y,
            vj.x - vi.x,
            0,
            pj.y - pi.y,
            pi.x - pj.x,
            0,
            pi.x * vi.y - pi.y * vi.x - pj.x * vj.y + pj.y * vj.x,
        ]);
    }

    mat
}

fn solve_equations(mat: &Vec<Vec<i64>>) -> Vec<i64> {
    use num_rational::Ratio;

    let n = mat.len();
    assert_eq!(mat[0].len(), n + 1);

    let mut mat: Vec<Vec<_>> = mat
        .iter()
        .map(|eq| eq.iter().map(|&x| Ratio::from(x as i128)).collect())
        .collect();

    for k in 0..n {
        let mut i = k;
        while mat[i][k] == Ratio::from(0) {
            i += 1;
        }
        mat.swap(k, i);

        for i in k + 1..n {
            let f = mat[i][k] / mat[k][k];
            mat[i][k] = Ratio::from(0);
            for j in k + 1..n + 1 {
                let tmp = f * mat[k][j];
                mat[i][j] -= tmp;
            }
        }
    }

    for k in (0..n).rev() {
        for i in 0..k {
            let f = mat[i][k] / mat[k][k];
            mat[i][k] = Ratio::from(0);
            let tmp = f * mat[k][n];
            mat[i][n] -= tmp;
        }
    }

    (0..n)
        .map(|i| {
            let ans = mat[i][n] / mat[i][i];
            assert!(ans.is_integer());
            i64::try_from(ans.to_integer()).unwrap()
        })
        .collect()
}

pub fn part1(input: &str) -> u32 {
    let input = parse(input);
    check_intersections(&input, 200000000000000, 400000000000000)
}

pub fn part2(input: &str) -> i64 {
    let input = parse(input);
    let equations = get_equations(&input);
    let ans = solve_equations(&equations);
    ans[0] + ans[1] + ans[2]
}

crate::samples! {
    (part1_puzzle, part1, "puzzle.in", 15558),
    (part2_sample, part2, "sample.in", 47),
    (part2_puzzle, part2, "puzzle.in", 765636044333842),
}
