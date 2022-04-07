// https://codemate.kr/project/%EC%BD%94%EB%94%A9-%ED%85%8C%EC%8A%A4%ED%8A%B8-Level-1/4-1.-%EB%B0%98%EB%B3%B5%EA%B3%BC-%EC%9E%AC%EA%B7%80

fn main(){
    println!("{}", factorial(5));
}

fn factorial(num: usize) -> usize{
    if num > 1{
        return num*factorial(num-1);
    } else {
        return 1;
    }
}