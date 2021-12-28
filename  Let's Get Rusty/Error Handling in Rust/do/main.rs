// handle error
use std::fs::File;
use std::io::ErrorKind;

fn main(){
    let FILE= "hello.TXT";
    let f= File::open(&FILE)

    let f= match f {
        Ok(file) => file,
        Err(error)=> match error.kind() {
            ErrorKind::NotFound => match File::create(&FILE) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem opening the file: {:?}", e) 
            },
            ohter_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };

    let f= File::open.unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound{
            File::create("hello.TXT").unwrap_or_else(|error|){
                panic!("Problem opening the file: {:?}", error)
            }
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });
}


// smart pointer
use std::ops::Deref;

struct DerefExample<T>{
    value: T,
}

impl<T> Deref for DerefExample<T> {
    type Target= T;
    fn deref(&self) -> &self::Target{
        &self.value
    }
}

let x= DerefExample { value: 'a' };
assert_eq!('a', *x);