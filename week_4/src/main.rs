fn main() {
    let numbers: [i32;5] = [1,2,3,4,5];

    let slice: &[i32] = &numbers[1..4];

    println!("Original array: {:?}", numbers);
    println!("Slice: {:?}",slice);

    let vec:Vec<i32> = vec![10,20,30,40,50];
    let vec_slice:&[i32] = &vec[2..];

    println!("Vector slice: {:?}", vec_slice);
}
