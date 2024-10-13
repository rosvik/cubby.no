use rocket::fs::FileServer;
use rocket::http::Status;
use rocket_dyn_templates::{context, Template};
use std::path::PathBuf;

mod data;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().ok();
    let _rocket = rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![get_index, get_tags])
        .mount("/static", FileServer::from("static"))
        .launch()
        .await?;
    Ok(())
}

#[get("/")]
async fn get_index() -> Result<Template, Status> {
    let directory = data::get_namespaces(None).map_err(|_| Status::InternalServerError)?;
    Ok(Template::render(
        "index",
        context! {
            directory,
        },
    ))
}

#[get("/tags/<path..>")]
async fn get_tags(path: PathBuf) -> Result<Template, Status> {
    let directory = data::get_namespaces(Some(path)).map_err(|_| Status::InternalServerError)?;
    Ok(Template::render(
        "list",
        context! {
            directory,
        },
    ))
}