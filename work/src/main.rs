#[derive(Debug)]
struct Inches(i32);

#[derive(PartialEq, PartialOrd)]
struct Cemtimeters(f64);

impl Inches {
    fn to_centimeters(&self) -> Cemtimeters{
        let &Inches(inches) = self;

        Cemtimeters(inches as f64 *2.54)  
    }
}

fn main() {
    
    let foot= Inches(12);

    let meter= Cemtimeters(100.0);

    let tcmp=
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };
}
