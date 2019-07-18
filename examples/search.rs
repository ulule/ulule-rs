use futures::future::{lazy};
use actix_rt::System;

use ulule::client::Client;
use ulule::search;

fn main() {
    let mut test = System::new("test");
    let client = Client::new();
    let p = search::Params::new()
        .limit(2)
        .with_term("beer")
        .with_extra_fields(
            vec![
            "owner".to_string(),
            "main_tag".to_string(),
            "main_image".to_string()]);

    println!("first page -------");
    let first_page: search::Projects = test.block_on(lazy(|| {
            search::projects(&client, Some(p))
        })).unwrap();
    println!("{:?}", first_page);

    if !first_page.meta.has_next() {
        return
    }

    println!("second page -------");
    let second_page: search::Projects = test.block_on(lazy(|| {
            search::projects(&client, first_page.meta.next)
        })).unwrap();
    println!("{:?}", second_page)
}
