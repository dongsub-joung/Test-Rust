struct Data{
    id: String,
    pw: i32,
    note: String,
}

impl data {
    fn setData(&self){
        let pw= String::from(self.pw);
        self.id + &pw
    }
}

fn main(){
    let data1= Data{
        id: String::now("wjdehdtjq"),
        pw: 123,
    }

    let note= data1.setData();
    println!(
        "{}", note
    );
}