#[cfg(test)]

fn solution(first: &str, second: &str) -> usize{
    let mut result= 0_usize;

    let mut f_list= first.split_whitespace();
    let n= f_list.next().unwrap().parse::<usize>().unwrap();
    let mut m= f_list.next().unwrap().parse::<usize>().unwrap();
    let k= f_list.next().unwrap().parse::<usize>().unwrap();

    let mut data: Vec<usize>= Vec::new();
    let s_list= second.split_whitespace();
    for i in s_list{
        let val= i.parse().unwrap();
        data.push(val);
    }

    data.sort();

    let max= data[n-1];
    let second= data[n-2];

    loop {
        for i in 0..k{
            if m == 0 { break; }
            result += max;
            m -= 1;
        }

        if m == 0{ break; }
        result += second;
        m -= 1;
    }

    result
}

fn solution2(first: &str, second: &str) -> usize{
    let mut result= 0_usize;

    let mut f_list= first.split_whitespace();
    let n= f_list.next().unwrap().parse::<usize>().unwrap();
    let m= f_list.next().unwrap().parse::<usize>().unwrap();
    let k= f_list.next().unwrap().parse::<usize>().unwrap();

    let mut data: Vec<usize>= Vec::new();
    let s_list= second.split_whitespace();
    for i in s_list{
        let val= i.parse().unwrap();
        data.push(val);
    }

    let max= data[n-1];
    let second= data[n-2];
    let cnt_index= (m as f64) / (k as f64 + 1.0) * k as f64;

    let mut cnt= cnt_index.round() as usize;
    cnt += m%(k+1);
    
    result += (cnt) * max;
    result += (m-cnt) * second;

    result
}

mod test{
    use super::*;
    
    #[test]
    fn test1(){
        assert_eq!(solution("5 8 3", "2 4 5 4 6"), 46);
    }

    #[test]
    fn test2(){
        assert_eq!(solution2("5 8 3", "2 4 5 4 6"), 46);
    }
}