// ex
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewArticle {
    fn summary(&self) -> String{
        format!("{}, by {} ({})"
            , self.headline
            , self.author
            , self.location
        );
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


// ex1
pub trait Summarizable{
    fn summary(&self) -> string{
        String::from("REAd more...")
    }
}


// ex2
pub trait Summarizable {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
}

impl Summarizable for Tweet {
    fn author_summary(&self) -> String{
        format!("@{}", self.username)
    }
}


// Trait Bound
pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}


// Some trait bound
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 
// eq
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug

