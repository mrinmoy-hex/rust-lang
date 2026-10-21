fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];

    println!("Third element is {}", *num);

    v.push(4);
}
