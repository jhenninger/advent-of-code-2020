fn main() {
    let input = include_str!("../input");
    let sum: usize = input
        .lines()
        .map(|line| calc(&mut line.chars(), false))
        .sum();

    dbg!(sum);
}

#[test]
fn test() {
    let tests = [
        ("2 * 3 * 1 + 3", 24),
        ("(2 * 3 * 1) + 3", 9),
        ("1 + 2 * 3 + 4 * 5 + 6", 231),
        ("1 + (2 * 3) + (4 * (5 + 6))", 51),
        ("2 * 3 + (4 * 5)", 46),
        ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
        ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
        ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
    ];

    for &(expr, result) in tests.iter() {
        assert_eq!(calc2(expr), result);
        assert_eq!(calc(&mut expr.chars(), false), result);
    }
}

#[test]
fn test_input() {
    for line in include_str!("../input").lines() {
        if calc2(line) != calc(&mut line.chars(), false) {
            dbg!(line);
            assert!(false);
        }
    }
}

fn calc(iter: &mut impl Iterator<Item = char>, rhs: bool) -> usize {
    let mut result = 0;

    while let Some(c) = iter.next() {
        match c {
            ' ' | '+' => (),
            '*' => {
                result *= calc(iter, true);
                if rhs {
                    return result;
                }
            }
            '(' => {
                result += calc(iter, rhs);
                if rhs {
                    //return result;
                }
            }
            ')' => return result,
            c if c.is_ascii_digit() => result += c.to_digit(10).unwrap() as usize,
            invalid => panic!("invalid char {}", invalid),
        }
    }

    result
}

/*

((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
((6 * 9) * (15 * 14) + 6) + 2 + 4 * 2
(54 * 210 + 6) + 2 + 4 * 2
(54 * 216) + 2 + 4 * 2
(54 * 216) + 2 + 4 * 2
11664 + 2 + 4  * 2
11670 * 2

23340

*/

fn calc2(s: &str) -> usize {
    let mut stack: Vec<char> = Vec::new();
    let mut out: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            ' ' => (),
            '+' | '*' => {
                while let Some(&top) = stack.last() {
                    if top == '(' {
                        break;
                    }

                    if prec(top) >= prec(c) {
                        out.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }

                stack.push(c);
            }
            '(' => stack.push(c),
            ')' => {
                while let Some(&top) = stack.last() {
                    if top != '(' {
                        out.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }

                if let Some('(') = stack.last() {
                    stack.pop();
                }
            }
            x if x.is_ascii_digit() => {
                out.push(x);
            }
            _ => panic!(),
        }
    }

    while let Some(x) = stack.pop() {
        out.push(x);
    }

    let mut stack = Vec::new();

    for x in out {
        match x {
            '+' => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            }
            '*' => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b);
            }
            _ if x.is_ascii_digit() => {
                let n = x.to_digit(10).unwrap() as usize;
                stack.push(n);
            }
            _ => panic!(),
        }
    }
    stack.pop().unwrap()
}

fn prec(c: char) -> usize {
    match c {
        '*' => 1,
        '+' => 2,
        _ => panic!("wtf: {}", c),
    }
}
