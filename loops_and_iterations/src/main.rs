fn main() {
    let mut count = 0;
    loop{
        println!("Count: {}", count);
        count+=1;
        if count == 3{
            break;
        }
    }
}
