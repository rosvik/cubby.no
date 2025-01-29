use rocket::http::Status;
use rocket::{config, fs::FileServer};
use rocket_dyn_templates::{context, Template};
use std::net::{IpAddr, Ipv4Addr};
use std::{env, path::PathBuf};

mod data;
mod utils;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().ok();
    let port = env::var("PORT").unwrap_or_default().parse::<u16>();
    let config = config::Config {
        port: port.unwrap_or(8601),
        address: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
        ..Default::default()
    };
    let _rocket = rocket::build()
        .configure(config)
        .attach(Template::fairing())
        .mount("/", routes![get_index, get_directory, get_manifest])
        .mount("/static", FileServer::from("static"))
        .launch()
        .await?;
    Ok(())
}

#[get("/?<path>")]
async fn get_index(path: Option<&str>) -> Result<Template, Status> {
    let partial_path = path.map(PathBuf::from);

    let full_path =
        data::get_path_or_default(partial_path.clone()).map_err(|_| Status::InternalServerError)?;

    if full_path.is_dir() {
        let directory =
            data::get_directory(partial_path.clone()).map_err(|_| Status::InternalServerError)?;
        return Ok(Template::render(
            "index",
            context! {
                partial_path,
                directory,
            },
        ));
    }

    if let Some(partial_path) = partial_path {
        if full_path.is_file() {
            let file = data::get_manifest(partial_path).map_err(|_| Status::InternalServerError)?;
            return Ok(Template::render(
                "file",
                context! {
                    file,
                },
            ));
        }
    }

    Err(Status::NotFound)
}

#[get("/htmx/directory/<partial_path..>")]
async fn get_directory(partial_path: PathBuf) -> Result<Template, Status> {
    let directory =
        data::get_directory(Some(partial_path)).map_err(|_| Status::InternalServerError)?;
    Ok(Template::render(
        "directory",
        context! {
            directory,
        },
    ))
}

#[get("/htmx/manifest/<partial_path..>")]
async fn get_manifest(partial_path: PathBuf) -> Result<Template, Status> {
    let manifest = data::get_manifest(partial_path).map_err(|_| Status::InternalServerError)?;
    Ok(Template::render(
        "manifest",
        context! {
            manifest,
        },
    ))
}
