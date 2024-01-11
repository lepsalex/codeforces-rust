use input::Scanner;

fn main() {
    let mut input = Scanner::new();
    let solution = solve(input.next(), input.next(), input.next());
    println!("{}", solution);
}

fn solve(n: u64, m: u64, a: u64) -> u64 {
    ((n + a - 1) / a) * ((m + a - 1) / a)
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn case_1() {
        assert_eq!(solve(6, 6, 4), 4);
    }

    #[test]
    fn case_2() {
        assert_eq!(solve(6, 4, 4), 2);
    }
}