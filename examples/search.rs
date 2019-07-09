use futures::future::{lazy};
use actix_rt::System;

fn main() {
    let client = ulule::client::Client::new();
    let p: ulule::search::Projects = System::new("test").block_on(lazy(|| {
        ulule::search::projects(&client, Some(ulule::search::Params::new().limit(30)))
    })).unwrap();
    println!("{:?}", p)
}
