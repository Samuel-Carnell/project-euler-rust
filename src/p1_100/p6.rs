use super::common::{square_sum, triangle};

pub fn problem() {
    let answer = triangle(100).pow(2) - square_sum(100);
    println!("answer: {answer}");
}
