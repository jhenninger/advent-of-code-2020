const P: usize = 20201227;

fn main() {
    let key_a = 335121;
    let key_b = 363891;

    let loop_size = find_loop_size(key_a);
    println!("Part 1: {}", transform(loop_size, key_b));
}

fn find_loop_size(n: usize) -> usize {
    let mut value = 1;

    for i in 1.. {
        value *= 7;
        value %= P;

        if value == n {
            return i;
        }
    }

    unreachable!()
}

fn transform(loop_size: usize, subject: usize) -> usize {
    let mut value = 1;

    for _ in 0..loop_size{
        value *= subject;
        value %= P;
    }

    return value;
}
