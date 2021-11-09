use permutohedron::Heap;

fn main() {
    let mut nums = (0..10).collect::<Vec<_>>();
    let h = Heap::new(&mut nums);
    let cnt = h.collect::<Vec<_>>();
    println!("{}", cnt.len());
}