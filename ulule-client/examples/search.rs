use ulule::search;
use ulule_client::{search_projects, Client};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let p = search::Params::new()
        .limit(2)
        .with_term("beer")
        .with_extra_fields(vec![
            "owner".to_string(),
            "main_tag".to_string(),
            "main_image".to_string(),
        ]);

    let first_page = search_projects(&client, Some(p)).await.unwrap();
    println!("first page: {:?}", first_page);

    if !first_page.meta.has_next() {
        return;
    }

    let second_page = search_projects(&client, first_page.meta.next).await.unwrap();
    println!("second page: {:?}", second_page)
}
