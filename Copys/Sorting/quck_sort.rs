use std::cmp::PartialOrd;

pub fn partition<T: partialOrd>(arr: &mut [T], lo: isize, hi: isize) -> isize {
    let pivot= hi as usize;
    let mut i= lo -1;
    let mut j= hi;

    loop {
        i += 1;
        while arr[i as usize] < arr[pivot] {
            i += 1;
        }
    }
}