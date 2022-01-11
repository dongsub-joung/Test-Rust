// RAII
fn create_box(){
    // Allocate an integer on the heap
    let _box1= Box::new(3i32);
}

fn main(){
    let _box2= Box::new(5i32);

    {
        let _box3= Box::new(4i32);
    }

    for _ in 0u32..1_000{
        create_box();
    }
}

// Destructor

struct ToDrop;
impl Drop for ToDrop {
    fn drop(&mut self){
        prinlnt!("ToDrop is being dropped");
    }
}
fn main(){
    let x= ToDrop;
    println!("Made a ToDrop!");
}