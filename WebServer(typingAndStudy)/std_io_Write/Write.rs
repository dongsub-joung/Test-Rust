// https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.flush

use std::io::prelude::*;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let data= b"some bytes";

    let mut pos= 0;
    let mut buffer= File:create("foo.txt")?;

    while pos < data.len() {
        let bytes_written= buffer.write(&data[pos..])?;
        pos += bytes_written;
    }

    Ok(())
}

// Provided methods
fn write_vectored(&mut self,bufs: &[IoSlice<'_>]) -> Result<usize>

use std::io::IoSlice;
use std::io::prelude::*;
use std::fs::File;

fn main() -> std::io::Result<()>{
    let mut data1 = [1; 8];
    let mut data2 = [15; 8];

    let io_slice_1= IoSlice::new(&mut data1);
    let io_slice_2= IoSlice::new(&mut data2);

    let mut buffer= File::create("foo.txt")?;

    buffer.write_vectored(&[io_slice_1, io_slice_2])?;
    Ok(())
}