use std::fs::File;
use std::io::ErrorKind;

fn main(){
    let f= File::open("hello.TXT")

    let f= match f {
        Ok(file) => file,
        Err(error)=> match error.kind() {
            
        }
    };
}