// ex
fn foo() -> ! {
    panic!("This call never returns.");
}

#![feature(never_type)]
fn main(){
    let x: != panic!("This call never returns.");
    println!("You will never see this line!");
}

fn main(){
    fn sum_odd_numbers(up_to: u32) -> u32{
        let mut acc= 0;
        for i in 0..up_to{
            let addition: u32= match i&2 == 1 {
                true => i,
                false => continue,
            };
            acc += addition;
        }
        acc
    }
}


// do-test
fn main()
    let chk: u8; 
    let fnc= |j| -> u8 { j-10 };
    for n in 0..{
        chk= match fnc(&n) == 0{
            true => {
                n
                break;
            },
            false => continue,
        };
    }
    println!("{}", chk;)
}