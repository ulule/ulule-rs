use futures::future::{lazy};
use actix_rt::System;

use ulule::client::Client;
use ulule::search;

fn main() {
    let client = Client::new();
    let p: search::Projects = System::new("test").block_on(lazy(|| {
        let p = search::Params::new()
            .limit(3)
            .with_extra_fields(
                vec![
                "owner".to_string(),
                "main_tag".to_string(),
                "main_image".to_string()]);
        search::projects(&client, Some(p))
    })).unwrap();
    println!("{:?}", p)
}
