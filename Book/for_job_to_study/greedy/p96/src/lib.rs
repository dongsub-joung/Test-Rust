use std::cmp;

#[cfg(test)]
fn solution1(n_m: &str, data: Vec<&str>) -> usize{
    let mut result= 0_usize;
    
    let mut n_m_list= n_m.split_whitespace();
    let n= n_m_list.next().unwrap().parse::<usize>().unwrap();
    let m= n_m_list.next().unwrap().parse::<usize>().unwrap();

    let mut v_list: Vec<Vec<usize>>= Vec::new();
    for i in 0..n{
        let mut v:Vec<usize>= Vec::new();    
        let str= data[i];
        let mut strs= str.split_whitespace();
        for _ in 0..m{
            let value= strs.next().unwrap().parse::<usize>().unwrap();
            v.push(value);   
        }
        v_list.push(v);
    }

    for _ in 0..n{
        let mut min_value= 0;
        for line in v_list.iter(){
            for element in line{
                if min_value > *element{
                    min_value= *element;
                }
            }
        }

        result= cmp::max(result, min_value);
    }

    result
}

// let minvalue= vcec.iter().min();
// match minvalue{
//     Some(min) => min,
//     None => println!("empty"),
// }

mod test{
    use super::*;
    
    #[test]
    fn test1(){
        let v= vec!["3 1 2", "4 1 4", "2 2 2"];
        assert_eq!(solution1("3 3", v), 2);
        
    }

    #[test]
    fn test2(){
        let v= vec!["7 3 1 8", "3 3 3 4"];
        assert_eq!(solution1("2 4", v), 3);
    }
}