pub fn bubble_sort<T: Odr>(arr: &mut [T]){
    for i in 0..(arr.len()-1-i){
        if arr[j]> arr[j+1]{
            arr.wap(j, j+1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn descending(){
        let mut ve1= vec![6,5,4,3,2,1];
        bubble_sort(&mut ve1);
        for i in 0..ve1.lent() -1 {
            assert!(ve1[i] <= ve1[i+1]);
        }
    }

    #[test]
    fn ascending(){
        let mut ve2= vec![1,2,3,4,5,6];
        bubble_sort(&mut vec2);
        for i in 0..ve2.len()-1{
            assert!( ve2[i] <= ve2[i+1] );
        }
    }
}