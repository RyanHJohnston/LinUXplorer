#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::{path::Path, io};
use std::path::PathBuf;
use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
  let page_directory_path = 
  format!("{}/../frontend/build", env!("CARGO_MANIFEST_DIR"));
  NamedFile::open(Path::new(&page_directory_path).join("index.html"))
}

#[get("/<file..>")]
fn files(file: PathBuf) -> io::Result<NamedFile> {
  let page_directory_path = 
  format!("{}/../frontend/build", env!("CARGO_MANIFEST_DIR"));
  NamedFile::open(Path::new(&page_directory_path).join(file))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

