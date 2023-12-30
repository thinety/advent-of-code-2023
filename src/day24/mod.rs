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

fn get_intersections(input: &[(Vec3, Vec3)], lower: i64, upper: i64) -> u32 {
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

// Let p0, v0 be the initial position and velocity of the rock, and pi, vi
// be the initial position and velocity of the i-th hailstone.
//
// The rock is eventually going to hit every hailstone, so we know that for every i,
// p0 + v0 * ti = pi + vi * ti, for some ti. Rearranging, we get:
//
// (p0 - pi) = - ti * (v0 - vi)
//
// With three such equations, we already have nine equations in nine unknowns
// (p0x, p0y, p0z, v0x, v0y, v0z, t1, t2, t3), so the system is, in theory, solvable.
//
// But we can be smarter about the equations. Note that (p0-pi) = -ti*(v0-vi) simply
// means that (p0-pi) and (v0-vi) are colinear, which is equivalent to
// (p0-pi) x (v0-vi) = p0 x v0 - pi x v0 - p0 x vi + pi x vi = 0, where x is the
// cross product. The problem is that now we have a non-linear term p0 x v0, but
// it can the cancelled by subtracting two such equations:
//
// (pi - pj) x v0 + p0 x (vi - vj) = pi x vi - pj x vj
//
// For each pair (i, j) of hailstones, we can get one such equation. With two pairs
// (three hailstones in total) we have six linear equations in six unknowns.
//
// This is the approach taken here, and the implementation uses integer gaussian
// elimination, with an integer ratio wrapper type.
fn part2_solution1(input: &[(Vec3, Vec3)]) -> i64 {
    use num_rational::Ratio;

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

    let n = mat.len();

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

    let ans: Vec<_> = (0..n)
        .map(|i| {
            let ans = mat[i][n] / mat[i][i];
            assert!(ans.is_integer());
            i64::try_from(ans.to_integer()).unwrap()
        })
        .collect();

    ans[0] + ans[1] + ans[2]
}

// But we can be even smarter about the previous equations. Of interest to us are
// only p0x, p0y and p0z, so we can do:
//
// (pi - pj) . ((pi - pj) x v0) + (pi - pj) . (p0 x (vi - vj)) = (pi - pj) . (pi x vi - pj x vj)
// v0 . (pi - pj) x (pi - pj) + p0 . (vi - vj) x (pi - pj) = (pi - pj) . (pi x vi - pj x vj)
// p0 . (vi - vj) x (pi - pj) = (pi - pj) . (pi x vi - pj x vj)
//
// Where . is the dot product. Now, for each pair (i, j) of hailstones, we have
// one linear equation in three unknowns. With three pairs (three hailstones in total)
// we get tree linear equations in three unknowns, which can be solved using just
// Cramer's rule.
//
// Taking into account that |pix| ≈ |piy| ≈ |piz| ≈ 1e15 and |vix| ≈ |viy| ≈ |viz| ≈ 1e2,
// the determinant in the numerator of Cramer's rule can use over 200 bits in memory,
// so as an implementation detail we need a big integer implementation.
fn part2_solution2(input: &[(Vec3, Vec3)]) -> i64 {
    use num_bigint::BigInt;

    fn diff(u: &[BigInt; 3], v: &[BigInt; 3]) -> [BigInt; 3] {
        [&u[0] - &v[0], &u[1] - &v[1], &u[2] - &v[2]]
    }
    fn dot(u: &[BigInt; 3], v: &[BigInt; 3]) -> BigInt {
        &u[0] * &v[0] + &u[1] * &v[1] + &u[2] * &v[2]
    }
    fn cross(u: &[BigInt; 3], v: &[BigInt; 3]) -> [BigInt; 3] {
        [
            &u[1] * &v[2] - &u[2] * &v[1],
            &u[2] * &v[0] - &u[0] * &v[2],
            &u[0] * &v[1] - &u[1] * &v[0],
        ]
    }
    fn coeffs(
        pi: &[BigInt; 3],
        vi: &[BigInt; 3],
        pj: &[BigInt; 3],
        vj: &[BigInt; 3],
    ) -> [BigInt; 4] {
        let [a1, a2, a3] = cross(&diff(vi, vj), &diff(pi, pj));
        let b = dot(&diff(pi, pj), &diff(&cross(pi, vi), &cross(pj, vj)));
        [a1, a2, a3, b]
    }

    fn det(mat: &[[&BigInt; 3]; 3]) -> BigInt {
        mat[0][0] * mat[1][1] * mat[2][2] - mat[0][0] * mat[1][2] * mat[2][1]
            + mat[0][1] * mat[1][2] * mat[2][0]
            - mat[0][1] * mat[1][0] * mat[2][2]
            + mat[0][2] * mat[1][0] * mat[2][1]
            - mat[0][2] * mat[1][1] * mat[2][0]
    }

    let (p0, v0) = &input[0];
    let (p1, v1) = &input[1];
    let (p2, v2) = &input[2];

    let p0 = [BigInt::from(p0.x), BigInt::from(p0.y), BigInt::from(p0.z)];
    let v0 = [BigInt::from(v0.x), BigInt::from(v0.y), BigInt::from(v0.z)];
    let p1 = [BigInt::from(p1.x), BigInt::from(p1.y), BigInt::from(p1.z)];
    let v1 = [BigInt::from(v1.x), BigInt::from(v1.y), BigInt::from(v1.z)];
    let p2 = [BigInt::from(p2.x), BigInt::from(p2.y), BigInt::from(p2.z)];
    let v2 = [BigInt::from(v2.x), BigInt::from(v2.y), BigInt::from(v2.z)];

    let [a11, a12, a13, b1] = coeffs(&p0, &v0, &p1, &v1);
    let [a21, a22, a23, b2] = coeffs(&p1, &v1, &p2, &v2);
    let [a31, a32, a33, b3] = coeffs(&p2, &v2, &p0, &v0);

    let a = det(&[[&b1, &a12, &a13], [&b2, &a22, &a23], [&b3, &a32, &a33]]);
    let b = det(&[[&a11, &b1, &a13], [&a21, &b2, &a23], [&a31, &b3, &a33]]);
    let c = det(&[[&a11, &a12, &b1], [&a21, &a22, &b2], [&a31, &a32, &b3]]);
    let d = det(&[[&a11, &a12, &a13], [&a21, &a22, &a23], [&a31, &a32, &a33]]);

    assert_eq!(&a % &d, BigInt::from(0));
    assert_eq!(&b % &d, BigInt::from(0));
    assert_eq!(&c % &d, BigInt::from(0));

    let ans: Vec<_> = [&a / &d, &b / &d, &c / &d]
        .iter()
        .map(|x| {
            let (sign, digits) = x.to_u64_digits();
            assert_eq!(digits.len(), 1);
            let sign = match sign {
                num_bigint::Sign::Minus => -1,
                num_bigint::Sign::NoSign => 0,
                num_bigint::Sign::Plus => 1,
            };
            sign * (digits[0] as i64)
        })
        .collect();

    ans[0] + ans[1] + ans[2]
}

pub fn part1(input: &str) -> u32 {
    let input = parse(input);

    get_intersections(&input, 200000000000000, 400000000000000)
}

pub fn part2(input: &str) -> i64 {
    let input = parse(input);

    let ans1 = part2_solution1(&input);
    let ans2 = part2_solution2(&input);
    assert_eq!(ans1, ans2);

    ans1
}

crate::samples! {
    (part1_puzzle, part1, "puzzle.in", 15558),
    (part2_sample, part2, "sample.in", 47),
    (part2_puzzle, part2, "puzzle.in", 765636044333842),
}
