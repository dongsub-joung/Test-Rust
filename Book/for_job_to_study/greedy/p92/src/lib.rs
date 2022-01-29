#[cfg(test)]

mod normal;
mod speed;

mod test{
    use super::*;
    
    #[test]
    fn test1(){
        assert_eq!(normal::solution1("5 8 3", "2 4 5 4 6"), 46);
    }

    #[test]
    fn test2(){
        assert_eq!(speed::solution2("5 8 3", "2 4 5 4 6"), 46);
    }
}