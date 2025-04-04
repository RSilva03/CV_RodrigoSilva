#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, NamedFile};
use rocket::State;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::path::{Path, PathBuf};

struct Counter(AtomicUsize);

#[get("/")]
async fn index(counter: &State<Counter>) -> Option<NamedFile> {
    counter.0.fetch_add(1, Ordering::Relaxed); // Increment counter
    NamedFile::open("/static/index.html").await.ok()
}

#[get("/views")]
fn views(counter: &State<Counter>) -> String {
    format!("CV Views: {}", counter.0.load(Ordering::Relaxed))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Counter(AtomicUsize::new(0)))
        .mount("/", routes![index, views])
        .mount("/static", FileServer::from("static")) // Serves CSS & images
}
