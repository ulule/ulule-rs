use futures::future::{lazy};
use actix_rt::System;

use ulule::client::Client;
use ulule::search;

fn main() {
    let client = Client::new();
    let p: search::Projects = System::new("test").block_on(lazy(|| {
        search::projects(&client, Some(search::Params::new()
                                       .with_extra_fields(vec!["owner".to_string()])
                                       .limit(30)
                                       ))
    })).unwrap();
    println!("{:?}", p)
}
