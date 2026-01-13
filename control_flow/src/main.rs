fn main() {
    let score = 85;
    if score >= 90 {
        println!("Excellent!");
    }
    else if score >= 70 {
        println!("Good job!");
    }
    else {
        println!("Needs improvement!");
    }

    let grade = if score > 80 {"A"} else {"B"};
    println!("Grade: {}", grade);
}
