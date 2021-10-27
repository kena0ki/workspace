use std::io::Read;

fn do_the_read<R: Read>(r: R) {
  // ...
}

fn main() {
  let mut data = [0, 1, 2];
  let mut slice: &[u8] = data.as_ref();
  do_the_read(slice);
}
