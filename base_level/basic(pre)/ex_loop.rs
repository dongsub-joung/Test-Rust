// 랜덤값 (1~10) 중에서 발생해서 만약 홀수이면 다시 발생하시오.
 use rand::Rng;

fn main()
{
    
    let random_num= rand::thread_rng().gen_range(1, 11);
        
}
