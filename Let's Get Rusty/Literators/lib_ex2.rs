#[cfg(test)]

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item= u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            slef.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
mod tests{
    use super::*;

    #[test]
    fn using_other_iterator_trait_methods(){
        let sum: u32= Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a,b)| a*b )
            .filter(|x| x%3 == 0)
            .sum();

        assert_eq!(18, sum);
    }
}