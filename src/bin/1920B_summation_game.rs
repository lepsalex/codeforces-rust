use std::ops::Sub;
use input::Scanner;

fn main() {
    let mut input = Scanner::new();
    let num_test_cases: u16 = input.next();

    for _ in 0..num_test_cases {
        let (n, k, x) = (input.next::<u32>(), input.next::<u32>(), input.next::<u32>());
        let case: Vec<i32> = (0..n).map(|_| input.next()).collect();
        let solution = solve(case, n, k, x);
        println!("{}", solution)
    }
}

/**
 * Bob will always use all his moves to turn the largest numbers
 * negative, we need to simulate removing one of the large numbers
 * with each remaining move to see if it makes the sum larger or not
 * to decide when Alice stops removing nums from the case vec
 */
fn solve(mut case: Vec<i32>, n: u32, k: u32, x: u32) -> i32 {
    // sort the vec
    case.sort();

    // the point at which we start negating
    let a = n.saturating_sub(k + x) as usize;

    // the point at which we can start removing
    let b = n.saturating_sub(k) as usize;

    // start curr sum at 0
    let mut cur_sum = 0;

    // the max sum is min 0 if we can remove all elements
    let mut max_sum = if n - k == 0 { 0 } else { i32::MIN };

    for (i, num) in case.iter().enumerate() {
        // for all point up to where we need to negate,
        // add them to the current sum
        if i < a {
            cur_sum += num;
        }

        // any numbers that Bob can negate and Alice
        // cannot remove will be negated
        if i >= a && i < b {
            cur_sum -= num
        }

        // numbers that can be removed by Alice ...
        if i >= b {

            // save the current sum
            // (meaning Alice removed everything up to this point)
            max_sum = max_sum.max(cur_sum);

            // if possible, add back the previous number
            if i.checked_sub(x as usize).is_some() {
                cur_sum += case[i - x as usize] * 2;
            }

            // negate the current number (ie. Alice did not remove it
            cur_sum -= num;
        }
    }

    // finally compare the max and cur one more time before returning
    return max_sum.max(cur_sum);
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn case_1() {
        let case = vec![1];
        assert_eq!(solve(case, 1, 1, 1), 0);
    }

    #[test]
    fn case_2() {
        let case = vec![3, 1, 2, 4];
        assert_eq!(solve(case, 4, 1, 1), 2);
    }

    #[test]
    fn case_3() {
        let case = vec![1, 4, 3, 2, 5, 6];
        assert_eq!(solve(case, 6, 6, 3), 0);
    }

    #[test]
    fn case_4() {
        let case = vec![3, 7, 3, 3, 32, 15];
        assert_eq!(solve(case, 6, 6, 1), 3);
    }

    #[test]
    fn case_5() {
        let case = vec![5, 5, 3, 3, 3, 2, 9, 9];
        assert_eq!(solve(case, 8, 5, 3), -5);
    }

    #[test]
    fn case_6() {
        let case = vec![1, 8, 2, 9, 3, 3, 4, 5, 3, 200];
        assert_eq!(solve(case, 10, 6, 4), -9);
    }

    #[test]
    fn case_7() {
        let case = vec![4, 3];
        assert_eq!(solve(case, 2, 2, 1), 0);
    }

    #[test]
    fn case_8() {
        let case = vec![1, 3];
        assert_eq!(solve(case, 2, 1, 2), -1);
    }
}