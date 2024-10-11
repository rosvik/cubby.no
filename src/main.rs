use rocket::http::Status;
use rocket_dyn_templates::{context, Template};

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![get_index,])
        .launch()
        .await?;
    Ok(())
}

#[get("/")]
async fn get_index() -> Result<Template, Status> {
    let list_items = vec!["Item 1", "Item 2", "Item 3"];
    Ok(Template::render(
        "index",
        context! {
            list_items,
        },
    ))
}
