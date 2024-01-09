use input::Scanner;

fn main() {
    let mut input = Scanner::new();
    let k: u32 = input.next();
    println!("{}", solve(k));
}

fn solve(k: u32) -> String {
    if k > 2 && k % 2 == 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn case_1() {
        assert_eq!(solve(8), "YES");
    }

    #[test]
    fn case_5() {
        assert_eq!(solve(2), "NO");
    }
}