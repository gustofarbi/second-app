#[macro_use]
extern crate rocket;

use lazy_static::lazy_static;
use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("template/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                std::process::exit(1);
            }
        };
        tera
    };
}

#[get("/?<name>&<pass>")]
fn index(name: Option<&str>, pass: Option<&str>) -> String {
    let mut ctx = Context::new();
    ctx.insert("name", name.unwrap_or("no name"));
    ctx.insert("pass", pass.unwrap_or("no pass"));

    TEMPLATES.render("index.html", &ctx).unwrap()
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .launch()
        .await?;

    Ok(())
}
