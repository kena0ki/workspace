fn main() {
    let s = "|\u{FFFD}|";
    for c in s.chars() {
        println!("{}", c);
    }
}
