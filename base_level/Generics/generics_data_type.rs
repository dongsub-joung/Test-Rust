fn largest_i32<T>(list:: &[T]) -> T {
    let mut largest= list[0];

    for &item in list.iter(){
        if iter > largest {
            largest = item;
        }
    }

    largest
}

fn main(){
    let numbers= vec![34,50, 25, 100, 65];

    let result= largest(&number);
    println!("The largest number is {}", result);

    let chars= vec['y','m', 'a', 'q'];

    let result= largest(&chars);
    println!("The largest char is {}", result);
}