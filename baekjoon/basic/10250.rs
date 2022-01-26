use std::{i128};

struct Data{
    h: i128,
    w: i128,
    n: i128
}
impl Data {
    fn new(h: i128, w: i128, n: i128) -> Data{
        Data{
            h,w,n
        }
    }use std::{i128};

    struct Data{
        h: i128,
        w: i128,
        n: i128
    }
    impl Data {
        fn new(h: i128, w: i128, n: i128) -> Data{
            Data{
                h,w,n
            }
        }
    
        fn calculation(self) -> String{
            let num= self.n / self.h + 1;
            let floor= self.n % self.h;
            let result= floor*100 + num;
            if self.n % self.h == 0{
                let num= self.n / self.h + 1;
                let floor= self.h;
                let result= floor *100 + num;
                
                return result.to_string();
            }
            return result.to_string();
        }
    }
    
    fn main() {
        // input value
        let t= 1i128;
        let mut v : Vec<String>= Vec::new();
        for _ in 0..t{
            // input value
            // let (h, w, n)= (6i128, 12i128, 10i128); 
            let (h, w, n)= (30, 50, 72); 
            let data= Data::new(h,w,n);
            v.push(Data::calculation(data)); 
        }
    
        for i in v.iter(){
            println!("{}", i);
        }
    }

    fn calculation(data: Data) -> String{
        let num= data.n / data.h + 1;
        let floor= data.n % data.h;
        let result= floor*100 + num;
        if data.n % data.h == 0{
            let num= data.n / data.h + 1;
            let floor= data.h;
            let result= floor *100 + num;
            
            return result.to_string();
        }
        return result.to_string();
    }
}

fn main() {
    // input value
    let t= 1i128;
    let mut v : Vec<String>= Vec::new();
    for _ in 0..t{
        // input value
        // let (h, w, n)= (6i128, 12i128, 10i128); 
        let (h, w, n)= (30, 50, 72); 
        let data= Data::new(h,w,n);
        v.push(Data::calculation(data)); 
    }

    for i in v.iter(){
        println!("{}", i);
    }
}