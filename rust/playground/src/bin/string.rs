fn main(){
    let s = String::with_capacity(100);
    println!("{}",s.len());
    let b = &[0u8;100];
    let s = String::from_utf8(b.to_vec()).unwrap();
    println!("{}",s.len());
    let b = vec![0u8;100];
    let s = String::from_utf8(b).unwrap();
    println!("{}",s.len());
    let b = vec!(100);
    let s = String::from_utf8(b).unwrap();
    println!("{}",s.len());
    let b = &[0u8;100];
    let s = std::str::from_utf8(b).unwrap();
    println!("{}",s.len());
}
