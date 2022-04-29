// run >> would overflow

const MAX_STACK_SIZE:usize= 3;
const TOP: i32= -1;

fn isFUll() -> bool{
    if (TOP>= (MAX_STACK_SIZE-1) as i32){
        return true;
    } else {
        return false;
    }
}

fn push(value: i32, mut stack: [i32; 3]){
    if(isFUll()){
        println!("stack is full");
    }else {
        let mut top= TOP as usize;
        top += 1;
        stack[top]= value;
    }
}

fn main(){
    let stack= [0; MAX_STACK_SIZE];
    push(3, stack);
    push(5, stack);
    push(12, stack);
    push(12, stack);
    push(12, stack);
}