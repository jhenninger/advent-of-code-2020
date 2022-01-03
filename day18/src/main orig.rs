fn main() {
    let input = include_str!("../input");
    let ex = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".trim();
    //let ex = "(2 * 3 * 1) + 3".trim();

    let sum: usize = input
        .lines()
        .map(|line| {
            calc2(line)
        })
        .sum();

    dbg!(sum);
}

fn calc(iter: &mut impl Iterator<Item = char>, level: usize) -> usize {
    let mut res = 0;

    while let Some(c) = iter.next() {
        match c {
            ' ' => (),
            '+' => {
                (if (level > 0) {
                    return res;
                })
            }
            '*' => {
                res *= calc(iter, level + 1);
            }
            '(' => {
                let n = calc(iter, level);
                if (level > 0) {
                    return res;
                }
                res += n;
            }
            c if c.is_ascii_digit() => {
                let n = c.to_digit(10).unwrap() as usize;
                res += n;
            }
            ')' => {
                return res;
            }
            _ => panic!(),
        }
    }
    res
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
                stack.push(a+b);
            }
            '*' => {

                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a*b);
            }
            _ if x.is_ascii_digit() => {
                let n = x.to_digit(10).unwrap() as usize;
                stack.push(n);
            }
            _ => panic!()
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
