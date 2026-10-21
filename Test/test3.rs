fn get_first(vr: &Vec<i32>) -> i32 {
    vr[0]
}


fn main() {
    let mut v = vec![0, 1, 2];
    let n = get_first(&v);

    println!("{} {}", n, v[1]);
}
