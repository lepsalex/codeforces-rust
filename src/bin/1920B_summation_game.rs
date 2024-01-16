use std::collections::BinaryHeap;

use input::Scanner;

fn main() {
    let mut input = Scanner::new();
    let num_test_cases: u16 = input.next();

    for _ in 0..num_test_cases {
        let n: u32 = input.next();
        let k: u32 = input.next();
        let x: u32 = input.next();

        let case: Vec<i32> = (0..n).map(|_| input.next()).collect();
        let solution = solve(case, k, x);
        println!("{}", solution)
    }
}

/**
 * Bob will always use all his moves to turn the largest numbers
 * negative, we need to simulate removing one of the large numbers
 * with each remaining move to see if it makes the sum larger or not
 * to decide when Alice stops removing nums from the case vec
 */
fn solve(mut case: Vec<i32>, k: u32, x: u32) -> i32 {
    // sort the vec
    case.sort();

    // init the max_sums with Alice taking no move
    let mut max_sum = BinaryHeap::new();
    max_sum.push(bobs_move(&case, x));

    // compute the new max_sum while the number is going up
    for _ in 0..k {
        case.pop();
        max_sum.push(bobs_move(&case, x))
    }

    // return the sum
    max_sum.pop().unwrap()
}

fn bobs_move(case: &Vec<i32>, x: u32) -> i32 {
    case.iter().enumerate().map(|(i, n)| if i >= case.len().saturating_sub(x as usize) { *n * -1 } else { *n }).sum()
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn case_1() {
        let case = vec![1];
        assert_eq!(solve(case, 1, 1), 0);
    }

    #[test]
    fn case_2() {
        let case = vec![3, 1, 2, 4];
        assert_eq!(solve(case, 1, 1), 2);
    }

    #[test]
    fn case_3() {
        let case = vec![1, 4, 3, 2, 5, 6];
        assert_eq!(solve(case, 6, 3), 0);
    }

    #[test]
    fn case_4() {
        let case = vec![3, 7, 3, 3, 32, 15];
        assert_eq!(solve(case, 6, 1), 3);
    }

    #[test]
    fn case_5() {
        let case = vec![5, 5, 3, 3, 3, 2, 9, 9];
        assert_eq!(solve(case, 5, 3), -5);
    }

    #[test]
    fn case_6() {
        let case = vec![1, 8, 2, 9, 3, 3, 4, 5, 3, 200];
        assert_eq!(solve(case, 6, 4), -9);
    }

    #[test]
    fn case_7() {
        let case = vec![4, 3];
        assert_eq!(solve(case, 2, 1), 0);
    }

    #[test]
    fn case_8() {
        let case = vec![1, 3];
        assert_eq!(solve(case, 1, 2), -1);
    }
}