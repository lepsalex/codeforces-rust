use std::collections::HashSet;
use input::Scanner;

fn main() {
    let mut input = Scanner::new();
    let num_test_cases: u16 = input.next();

    for _ in 0..num_test_cases {
        let nums_len: u8 = input.next();
        let nums: Vec<u64> = (0..nums_len).map(|_| input.next()).collect();
        let solution = solve(nums);
        println!("{}", solution)
    }
}

fn solve(nums: Vec<u64>) -> u64 {
    let mut distinct_mod_results: HashSet<u64> = HashSet::new();

    let mut cur_mod: u64 = 2;
    let mut cursor: usize = 0;

    while cursor < nums.len() {
        let x = nums[cursor] % cur_mod;

        if (distinct_mod_results.insert(x) && distinct_mod_results.len() > 2) || (cursor == nums.len() - 1 && distinct_mod_results.len() != 2) {
            distinct_mod_results.clear();
            cur_mod *= 2;
            cursor = 0;
            continue;
        }

        cursor += 1;
    }

    cur_mod
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn case_1() {
        let nums = vec![8, 15, 22, 30];
        assert_eq!(solve(nums), 2);
    }

    #[test]
    fn case_2() {
        let nums = vec![60, 90, 98, 120, 308];
        assert_eq!(solve(nums), 4);
    }

    #[test]
    fn case_3() {
        let nums = vec![328, 769, 541, 986, 215, 734];
        assert_eq!(solve(nums), 2);
    }

    #[test]
    fn case_4() {
        let nums = vec![1000, 2000, 7000, 11000, 16000];
        assert_eq!(solve(nums), 16);
    }

    #[test]
    fn case_5() {
        let nums = vec![2, 1];
        assert_eq!(solve(nums), 2);
    }
}