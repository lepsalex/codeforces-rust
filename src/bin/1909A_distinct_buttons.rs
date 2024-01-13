use input::Scanner;

fn main() {
    let mut input = Scanner::new();
    let num_test_cases: u16 = input.next();

    for _ in 0..num_test_cases {
        let num_coords: u8 = input.next();
        let coords: Vec<(i8, i8)> = (0..num_coords).map(|_| (input.next(), input.next())).collect();
        let solution = solve(coords);
        println!("{}", solution)
    }
}

fn solve(coords: Vec<(i8, i8)>) -> String {
    /**
    Using only 3-buttons on the controller we can reach any point
    as long as all points are within the same 180-degree arc

    Ex. if you only use button U, R, D, you can never reach the coords with x < 0
    */
    return if coords.iter().all(|(x, y)| *x >= 0) ||
        coords.iter().all(|(x, y)| *x <= 0) ||
        coords.iter().all(|(x, y)| *y >= 0) ||
        coords.iter().all(|(x, y)| *y <= 0) {
        "YES".to_string()
    } else {
        "NO".to_string()
    };
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn case_1() {
        let coords = vec![
            (1, -1),
            (0, 0),
            (1, -1),
        ];
        assert_eq!(solve(coords), "YES".to_string());
    }

    #[test]
    fn case_2() {
        let coords = vec![
            (-3, -2),
            (-3, -1),
            (-3, 0),
            (-3, 1),
        ];
        assert_eq!(solve(coords), "YES".to_string());
    }

    #[test]
    fn case_3() {
        let coords = vec![
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ];
        assert_eq!(solve(coords), "NO".to_string());
    }

    #[test]
    fn case_4() {
        let coords = vec![
            (-4, 14),
            (-9, -13),
            (-14, 5),
            (14, 15),
            (-8, -4),
            (19, 9),
        ];
        assert_eq!(solve(coords), "NO".to_string());
    }

    #[test]
    fn case_5() {
        let coords = vec![
            (82, 64),
            (39, 91),
            (3, 46),
            (87, 83),
            (74, 21),
            (7, 25),
        ];
        assert_eq!(solve(coords), "YES".to_string());
    }

    #[test]
    fn case_6() {
        let coords = vec![
            (100, -100),
        ];
        assert_eq!(solve(coords), "YES".to_string());
    }
}