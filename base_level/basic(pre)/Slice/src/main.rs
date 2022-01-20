fn first_word(s: &String) -> usize
{
    let byte= s.as_bytes();

    for(i, &items) in byte.iter().enumerate()
    {
        if items == b' '
        {
            return i;
        }
    }
    
    s.len()
}

fn first_word_slaces(s: &String) -> &str
{
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() 
    {
        if item == b' ' 
        {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let str= String::from(" Hello");
    let result= first_word(&str);
    println!("{}", result);
}
