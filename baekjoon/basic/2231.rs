fn main(){
    let mut num =0_usize; 
    let mut sum= 0_usize;
    let mut part= 0_usize;

    // num= inputing()
    num= 216;
 
    for i in 1..num{
        sum= i;
        part= i;
        
        loop{
            sum += part % 10;
            part /= 10;
            
            // while (true)
            if part == 1 {
                break;
            }
        }

        if num == sum {
            println!("{}", i);
        } else {
            println!("0");
        }
    }
}