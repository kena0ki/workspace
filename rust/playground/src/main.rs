fn main() {
    let mut v = vec![1];
    println!("{}, {}", v.len(), v.capacity());
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    println!("{}, {}", v.len(), v.capacity());
    v.pop();
    println!("{}, {}", v.len(), v.capacity());
    v = vec![];
    println!("{}, {}", v.len(), v.capacity());
    // let mut s = String::new();
    // unsafe {
    //     let vec = s.as_mut_vec();
    //     let old_len = vec.len();
    //     let capacity = vec.capacity();
    //     vec.set_len(capacity);
    //     println!("{}", capacity);
    // }
}
