use rocket::http::Status;
use rocket_dyn_templates::{context, Template};

mod data;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![get_index])
        .launch()
        .await?;
    Ok(())
}

#[get("/")]
async fn get_index() -> Result<Template, Status> {
    let list_items = data::get_namespaces().map_err(|_| Status::InternalServerError)?;
    Ok(Template::render(
        "index",
        context! {
            list_items,
        },
    ))
}
