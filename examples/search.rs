use futures::future::{lazy};
use actix_rt::System;

use ulule::client::Client;
use ulule::search;

fn main() {
    let client = Client::new();
    let p: search::Projects = System::new("test").block_on(lazy(|| {
        search::projects(&client, Some(search::Params::new().limit(30)))
    })).unwrap();
    println!("{:?}", p)
}
