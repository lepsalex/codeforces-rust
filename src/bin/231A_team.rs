use input::Scanner;

fn main() {
    let mut input = Scanner::new();

    let solution = (0..input.next()).filter(|_| solve(&[input.next::<u32>(), input.next::<u32>(), input.next::<u32>()])).count();
    println!("{}", solution)
}

fn solve(votes: &[u32]) -> bool {
    votes.iter().sum::<u32>() >= 2
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn case_1() {
        assert_eq!(solve(&[1, 1, 0]), true);
    }

    #[test]
    fn case_2() {
        assert_eq!(solve(&[1, 1, 1]), true);
    }

    #[test]
    fn case_3() {
        assert_eq!(solve(&[1, 0, 0]), false);
    }

    #[test]
    fn case_4() {
        assert_eq!(solve(&[0, 0, 0]), false);
    }
}