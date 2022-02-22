use actix_web::{ HttpServer, App, web, HttpResponse, Responder };
use tera::{ Tera, Context };
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
struct User {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
struct Post{
    title: String,
    link: String,
    author: String,
}

#[derive(Debug, Deserialize)]
struct LoginUser {
    username: String,
    password: String,
}


async fn process_signup(data: web::Form<User>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Successfully saved user: {}", data.username))
}

async fn index(tera: web::Data<Tera>) -> impl Responder{
    let mut data= Context::new();

    let posts=[
        Post{
            title: String::from("First link"),
            link: String::from("https://dongsub-joung.github.io"),
            author: String::from("Joung Dongsub"),
        },
        Post{
            title: String::from("First link"),
            link: String::from("https://dongsub-joung.github.io"),
            author: String::from("Joung Dongsub"),
        },
    ];

    data.insert("title", "Hacker Clone");
    data.insert("posts", &posts);

    let rendered= tera.render("index.html", &data).unwrap();
    
    HttpResponse::Ok().body(rendered)
}

async fn signup(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Sign Up");

    let rendered = tera.render("signup.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();
        App::new()
            .data(tera)
            .route("/", web::get().to(index))
            .route("/signup", web::get().to(signup))
            .route("/signup", web::post().to(process_signup))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}