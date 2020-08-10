use std::mem;
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
enum Project {
    Mercury { mission: u8 },
    Gemini { mission: u8 },
    Apollo { mission: u8 },
    Shuttle { mission: u8 },
}

// Rust allows programmers to include DSTs
// as the last field of a struct this causes
// the struct itself to become a DST
struct ImportantThing {
    tag: u8,
    data: [u8],
}

fn main() {
    assert_eq!(1, mem::size_of::<u8>());
    assert_eq!(2, mem::size_of::<Project>());

    let ptr_sz = mem::size_of::<usize>();
    assert_eq!(ptr_sz, mem::size_of::<&Project>());

    let vec_sz = mem::size_of::<usize>() * 2 + ptr_sz;
    assert_eq!(vec_sz, mem::size_of::<Vec<Project>>());

    let values = vec![0, 1, 2, 3, 4, 5, 7, 8, 9, 10];
    let cur: usize = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    let cap: usize = cur % values.len();

    let slc: &[u8] = &values[0..cap];

    println!("{:?}", slc);
}
