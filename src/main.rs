use rocket::fs::FileServer;
use rocket::http::Status;
use rocket_dyn_templates::{context, Template};
use std::{env, path::PathBuf};

mod data;
mod utils;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().ok();
    let port = env::var("PORT").unwrap_or_default().parse::<u16>();
    let _rocket = rocket::build()
        .configure(rocket::Config::figment().merge(("port", port.unwrap_or(8601))))
        .attach(Template::fairing())
        .mount("/", routes![get_index, get_directory, get_manifest])
        .mount("/static", FileServer::from("static"))
        .launch()
        .await?;
    Ok(())
}

#[get("/?<path>")]
async fn get_index(path: Option<&str>) -> Result<Template, Status> {
    let path_buf = path.map(PathBuf::from);
    let directory = data::get_directory(path_buf).map_err(|_| Status::InternalServerError)?;
    Ok(Template::render(
        "index",
        context! {
            path,
            directory,
        },
    ))
}

#[get("/htmx/directory/<path..>")]
async fn get_directory(path: PathBuf) -> Result<Template, Status> {
    let directory = data::get_directory(Some(path)).map_err(|_| Status::InternalServerError)?;
    Ok(Template::render(
        "directory",
        context! {
            directory,
        },
    ))
}

#[get("/htmx/manifest/<path..>")]
async fn get_manifest(path: PathBuf) -> Result<Template, Status> {
    let manifest = data::get_manifest(path).map_err(|_| Status::InternalServerError)?;
    Ok(Template::render(
        "manifest",
        context! {
            manifest,
        },
    ))
}
