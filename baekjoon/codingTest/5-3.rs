fn main(){
    let mut arr= vec![5, 3, 7, 6, 2, 1, 4 ];
    quick_sort(arr.clone(),0,6);
    println!("{:?}", arr);
}

fn swap(arr: Vec<i32>, i: usize, j: usize) -> Vec<i32>{
    let mut arr2= arr.clone();

    let temp= arr2[i];
    arr2[i]= arr2[j];
    arr2[j]= temp;
    
    arr2
}


fn quick_sort(mut arr: Vec<i32>, start: usize, end: usize){
    if start >= end{
        panic!("end");
    }

    let pivot= arr[end].clone();
    let mut left= start.clone();

    for right in start..end{
        let right_val= arr[right].clone();
        if  right_val < pivot {
            arr= swap(arr, left, right);
            left+= 1;
        }
    }

    arr= swap(arr, left, end);

    quick_sort(arr.clone(), start, left-1);
    quick_sort(arr.clone(), left+1, end);
}