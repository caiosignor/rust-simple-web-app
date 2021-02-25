//docs
//https://blog.logrocket.com/rust-web-apps-using-rocket-framework/
#![feature(decl_macro)] //necessario quando for utilizar as rotas.

//o que eh exatamente as macros.
#[macro_use]
extern crate rocket; //importa as macros do rocket, como alternativa use rocket::*;
use rocket::request::Form;
use rocket::response::content::Json; //importa as macros para usar json.
use rocket::Request;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[get("/hello")]
fn hello() -> Json<&'static str> {
    Json(
        "{
    'status': 'success',
    'message': 'Hello API!'
  }",
    )
}

#[derive(FromForm, Debug)] //the trait `FromDataSimple` i  s not implemented for `Book` para evitar esse erro
struct Book {
    title: String,
    author: String,
    isbn: String,
}

#[post("/book", data = "<book_form>")]
//cria um wrap de form para book
fn new_book(book_form: Form<Book>) -> String {
    let book: Book = book_form.into_inner(); //get the body from request
    let mut dummy_db: Vec<Book> = Vec::new();
    dummy_db.push(book);
    format!("Book added successfully: {:?}", dummy_db)
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

#[derive(Serialize)]
struct Context {
    first_name: String,
    last_name: String,
}

#[get("/")]
fn index() -> Template {
    let context = Context {
        first_name: String::from("Jane"),
        last_name: String::from("Doe"),
    };
    Template::render("home", context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .register(catchers![not_found])
        .attach(Template::fairing())
        .mount("/api", routes![hello])
        .launch();
}
