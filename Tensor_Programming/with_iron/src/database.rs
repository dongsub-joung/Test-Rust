use models::Post;

#[derive(Debug, Clone)]
pub struct Database{
    posts: Vec<Post>,
}

impl Database {
    pub fn new() -> Database{
        Database { posts: vec![] }
    }

    // set
    pub fn add_post(&mut self, post: Post){
        self.posts.push(post);
    }

    // get
    pub fn posts(&self) -> &Vec<Post>{
        &self.posts
    }
}