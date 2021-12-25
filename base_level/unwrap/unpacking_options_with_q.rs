// https://doc.rust-lang.org/rust-by-example/error/option_unwrap/question_mark.html

struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job{
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber{
    area_code: Option<u8>,
    number   : u32,
}

impl Pserson{
    fn work_phone_area_code(&slef) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}

fn main(){
    let p= Person{
        job: Some(Job {
           phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number   : 456486,
           }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}