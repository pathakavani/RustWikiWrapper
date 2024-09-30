use rust_wiki_wrapper::api::MediaWikiClient;

#[tokio::main]
async fn main() {
    let client = MediaWikiClient::new("https://en.wikipedia.org/api/rest_v1");
    let summary = client.get_page_summary("Rust_(programming_language)").await.unwrap();
    println!("Page Summary: {:?}", summary);
}