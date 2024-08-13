#[macro_use] extern crate rocket;

use rocket::State;
use std::sync::atomic::{AtomicUsize, Ordering};
use rocket::tokio::time::{sleep, Duration};

struct HitCounter {
    count: AtomicUsize,
}

#[get("/")]
async fn index(hit_counter: &State<HitCounter>) -> String {
    let hits = hit_counter.count.fetch_add(1, Ordering::Relaxed);
    // Simulate some async work
    sleep(Duration::from_millis(10)).await;
    format!("Hello, world! Welcome to Rocket. Hits: {}", hits)
}

#[get("/delay/<ms>")]
async fn delay(ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    format!("Delayed response after {} ms", ms)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(HitCounter { count: AtomicUsize::new(0) })
        .mount("/", routes![index, delay])
        .configure(rocket::Config::figment()
            .merge(("workers", num_cpus::get() * 2))
            .merge(("max_blocking", 512))
            .merge(("keep_alive", 5))
            .merge(("address", "0.0.0.0"))
        )
}
