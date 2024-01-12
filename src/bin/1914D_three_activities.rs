use input::Scanner;

fn main() {
    let mut input = Scanner::new();
    let num_test_cases: usize = input.next();

    for _ in 0..num_test_cases {
        let input_size = input.next();
        let case: Vec<Vec<u32>> = (0..3).map(|_| (0..input_size).map(|_| input.next()).collect()).collect();
        let max_friends = solve(case);
        println!("{}", max_friends)
    }
}

fn solve(case: Vec<Vec<u32>>) -> u32 {
    let mut max_sum = 0;

    let top_3: Vec<Vec<(usize, u32)>> = case.into_iter().map(|x| {
        let mut top: Vec<(usize, u32)> = x.into_iter().enumerate().collect();
        top.sort_by(|(idx_a, num_a), (idx_b, num_b)| num_b.cmp(num_a));
        top.into_iter().take(3).collect()
    }).collect();

    for (i, x) in &top_3[0] {
        for (j, y) in &top_3[1] {
            for (k, z) in &top_3[2] {
                if i != j && j != k && i != k {
                    let sum = x + y + z;
                    if sum > max_sum {
                        max_sum = sum;
                    }
                }
            }
        }
    }

    max_sum
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn case_1() {
        let case = vec![
            vec![1, 10, 1],
            vec![10, 1, 1],
            vec![1, 1, 10],
        ];

        assert_eq!(solve(case), 30);
    }

    #[test]
    fn case_2() {
        let case = vec![
            vec![30, 20, 10, 1],
            vec![30, 5, 15, 20],
            vec![30, 25, 10, 10],
        ];

        assert_eq!(solve(case), 75);
    }

    #[test]
    fn case_3() {
        let case = vec![
            vec![5, 19, 12, 3, 18, 18, 6, 17, 10, 13],
            vec![15, 17, 19, 11, 16, 3, 11, 17, 17, 17],
            vec![1, 17, 18, 10, 15, 8, 17, 3, 13, 12],
        ];

        assert_eq!(solve(case), 55);
    }
}