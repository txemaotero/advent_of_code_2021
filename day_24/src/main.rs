static ALPHA: [i32; 14] = [1, 1, 1, 26, 1, 1, 1, 26, 1, 26, 26, 26, 26, 26];
static BETA: [i32; 14] = [10, 12, 15, -9, 15, 10, 14, -5, 14, -7, -12, -10, -1, -11];
static GAMMA: [i32; 14] = [15, 8, 2, 6, 13, 4, 1, 9, 5, 13, 9, 6, 2, 2];

fn part1() -> [u8; 14] {
    let mut input = [0; 14];
    find_max_sequence(0, &mut input, 13);
    input
}

fn find_max_sequence(zn_1: i32, input: &mut [u8; 14], depth: usize) {
    let mut w: u8 = 0;
    // if input[0] != 0 {
    //     return;
    // }
    while w < 9 {
        w += 1;
        if depth >= 13 {
            println!("Depth {}, w={}", depth, w);
        }
        let aux = zn_1 - w as i32 - GAMMA[depth];
        // for each alpha 2 arms corresponding to x = 0 and x = 1
        if ALPHA[depth] == 1 {
            alpha_1_x_1(aux, w, input, depth);
            alpha_1_x_0(zn_1, w, input, depth);
        } else {
            let diff = w as i32 - BETA[depth];
            alpha_26_x_1(aux, diff, w, input, depth);
            alpha_26_x_0(zn_1, diff, w, input, depth);
        }
    }
}

fn alpha_1_x_1(aux: i32, w: u8, input: &mut [u8; 14], depth: usize) {
    if aux < 0 || aux % 26 != 0 {
        return;
    }
    let zn = aux / 26;
    if zn % 26 == w as i32 - BETA[depth] {
        return;
    }
    if depth == 0 {
        if zn == 0 {
            input[depth] = w;
        }
        return;
    }
    input[depth] = w;
    find_max_sequence(zn, input, depth - 1);
}

fn alpha_1_x_0(zn_1: i32, w: u8, input: &mut [u8; 14], depth: usize) {
    if zn_1 % 26 != w as i32 - BETA[depth] {
        return;
    }
    let zn = zn_1;
    if depth == 0 {
        if zn == 0 {
            input[depth] = w;
        }
        return;
    }
    input[depth] = w;
    find_max_sequence(zn, input, depth - 1);
}

fn alpha_26_x_1(aux: i32, diff: i32, w: u8, input: &mut [u8; 14], depth: usize) {
    if aux < 0 || aux % 26 != 0 {
        return;
    }
    for b in 0..26 {
        if b == diff {
            return;
        }
        let zn = aux / 26 + b;
        if depth == 0 {
            if zn == 0 {
                input[depth] = w;
            }
            return;
        }
        input[depth] = w;
        find_max_sequence(zn, input, depth - 1);
    }
}
fn alpha_26_x_0(zn_1: i32, diff: i32, w: u8, input: &mut [u8; 14], depth: usize) {
    if diff < 0 || diff > 25 {
        return;
    }
    let zn = zn_1 * 26 + diff;
    if depth == 0 {
        if zn == 0 {
            input[depth] = w;
        }
        return;
    }
    input[depth] = w;
    find_max_sequence(zn, input, depth - 1);
}

fn check_answer(number: &[u8; 14]) -> bool {
    let mut zn = 0;
    for (ind, num) in number.iter().enumerate() {
        let x = {
            if zn % 26 + BETA[ind] != *num as i32 {
                1
            } else {
                0
            }
        };
        zn = (zn / 26) * (25 * x + 1) + x * (ALPHA[ind] + *num as i32);
    }
    if zn == 0 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let result = part1();
    println!(
        "Part 1: {}",
        result
            .into_iter()
            .map(|x| x.to_string())
            .collect::<String>()
    );
    dbg!(check_answer(&result));
    // Too high: 81111111111111
    // Too low:  52916991911111
    // Too low:  32916991911111
}
