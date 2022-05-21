use array2d::Array2D;
use std::cmp;
use std::string::String;
use std::time::Instant;

fn swa(seq1: String, seq2: String) {
    const GAP_PENALTY: i32 = 2;

    let seq1_arr: Vec<char> = seq1.chars().collect();
    let seq2_arr: Vec<char> = seq2.chars().collect();

    let mut score_matrix = Array2D::filled_with(0, seq1_arr.len() + 1, seq2_arr.len() + 1);

    let s_ab = |a: char, b: char| -> i32 {
        if a == b {
            return 1;
        } else {
            return -1;
        };
    };

    for i in 1..(seq1_arr.len() + 1) {
        for j in 1..(seq2_arr.len() + 1) {
            score_matrix[(i, j)] = cmp::max(
                score_matrix[(i - 1, j - 1)] + s_ab(seq1_arr[i - 1], seq2_arr[j - 1]),
                cmp::max(
                    score_matrix[(i - 1, j)] - GAP_PENALTY,
                    cmp::max(score_matrix[(i, j - 1)] - GAP_PENALTY, 0),
                ),
            );
        }
    }

    let mut top_str = String::from("");
    let mut mid_str = String::from("");
    let mut btm_str = String::from("");

    let mut x = seq1_arr.len();
    let mut y = seq2_arr.len();

    while x > 0 && y > 0 {
        let score = cmp::max(
            score_matrix[(x - 1, y - 1)],
            cmp::max(score_matrix[(x - 1, y)], score_matrix[(x, y - 1)]),
        );
        if score == score_matrix[(x - 1, y - 1)] {
            top_str.push(seq1_arr[x - 1]);
            mid_str.push('|');
            btm_str.push(seq2_arr[y - 1]);
            x -= 1;
            y -= 1;
        } else if score == score_matrix[(x - 1, y)] {
            top_str.push(seq1_arr[x - 1]);
            mid_str.push(' ');
            btm_str.push('-');
            x -= 1;
        } else if score == score_matrix[(x, y - 1)] {
            top_str.push('-');
            mid_str.push(' ');
            btm_str.push(seq2_arr[y - 1]);
            y -= 1;
        }
    }
}

fn main() {
    println!("Running SWA");
    let seq1 = "AAATTTA".repeat(1000);
    let seq2 = "AAATT".repeat(1000);
    let now = Instant::now();
    swa(seq1, seq2);
    let elapsed = now.elapsed().as_secs_f32();
    println!("SWA finished in {:.5?}s", elapsed)
}
