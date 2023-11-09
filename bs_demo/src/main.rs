use std::cmp::Ordering;
use std::str::from_utf8;

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let mut matrix = vec![vec![0; b_bytes.len() + 1]; a_bytes.len() + 1];
    for i in 0..=a_bytes.len() {
        for j in 0..=b_bytes.len() {
            if i == 0 {
                matrix[i][j] = j as usize;
            } else if j == 0 {
                matrix[i][j] = i as usize;
            } else {
                let cost = if a_bytes[i - 1] == b_bytes[j - 1] { 0 } else { 1 };
                matrix[i][j] = std::cmp::min(
                    std::cmp::min(
                        matrix[i - 1][j] + 1,
                        matrix[i][j - 1] + 1,
                    ),
                    matrix[i - 1][j - 1] + cost,
                );
            }
        }
    }
    matrix[a_bytes.len()][b_bytes.len()]
}

fn main() {

    let a = "我8612345678901";
    let b = "-8612345678901";
    let mut max_s ;
    let distance = levenshtein_distance(a, b);
    let similarity = 1.0 - (distance as f64) / (a.len().max(b.len()) as f64);
//println!("编辑距离: {}", distance);
    println!("相似度: {:0.4}", similarity);
    max_s= similarity;

    let len_a = a.len();
    let len_b = b.len();
    let abs = len_a.abs_diff(len_b) ;
    if len_a > len_b {
        let abs = len_a - len_b ;

        let t = & a[abs..];
        let distance = levenshtein_distance(t, b);
        let similarity = 1.0 - (distance as f64) / (len_b as f64);
//println!("编辑距离: {}", distance);
        println!("相似度: {:0.4}", similarity);
        max_s =max_s.max(similarity);

        let t = & a[..len_b];
        let distance = levenshtein_distance(t, b);
        let similarity = 1.0 - (distance as f64) / (len_b as f64);
//println!("编辑距离: {}", distance);
        println!("相似度: {:0.4}", similarity);
        max_s =max_s.max(similarity);
    }else if len_a < len_b{
        let abs =len_b - len_a ;
        let t = & b[abs..];
        let distance = levenshtein_distance(t, b);
        let similarity = 1.0 - (distance as f64) / (len_a as f64);
//println!("编辑距离: {}", distance);
        println!("相似度: {:0.4}", similarity);
        max_s =max_s.max(similarity);
        let t = & b[..len_a];
        let distance = levenshtein_distance(t, b);
        let similarity = 1.0 - (distance as f64) / (len_a as f64);
//println!("编辑距离: {}", distance);
        println!("相似度: {:0.4}", similarity);
        max_s =max_s.max(similarity);
    }

    println!("最大相似度: {:0.4}", max_s);

}