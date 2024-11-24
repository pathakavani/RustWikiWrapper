use dotenv::dotenv;
use rust_wiki_wrapper::api::action::login;
use rust_wiki_wrapper::api::MediaWikiClient;
use std::env;
use std::error::Error;

async fn login_and_edit_page(
    page_title: &str,
    page_content: &str,
    should_watch: bool,
) -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let username = env::var("WIKI_USERNAME").expect("WIKI_USERNAME must be set in .env file");
    let password = env::var("WIKI_PASSWORD").expect("WIKI_PASSWORD must be set in .env file");

    let client = MediaWikiClient::new("https://test.wikipedia.org");

    println!("Logging in as {}...", username);
    let login_result = login::login(&client, &username, &password).await?;

    match login_result.clientlogin.status.as_str() {
        "PASS" => println!(
            "Successfully logged in as {}",
            login_result.clientlogin.username.unwrap_or_default()
        ),
        _ => return Err("Login failed".into()),
    }

    println!("Creating new page '{}'...", page_title);
    let edit_result = client
        .create_edit_page(
            page_title,
            page_content,
            Some("Initial page creation via RustWikiWrapper"), // edit summary
        )
        .await?;

    match edit_result.edit.result.as_str() {
        "Success" => {
            println!("Page created successfully!");
            println!("Page ID: {:?}", edit_result.edit.page_id);
            println!("New revision ID: {:?}", edit_result.edit.new_rev_id);

            if should_watch {
                println!("Adding page to watchlist...");
                client.set_page_watched(page_title, true).await?;
                println!("Page added to watchlist");
            }
        }
        other => println!("Page creation failed with result: {}", other),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let page_title = "Testpage4";
    let page_content = r#"= Welcome to Testpage4 =
This is a demonstration page created using the RustWikiWrapper API.

== Introduction ==
This page was automatically created to test the Wikipedia API integration.

== Features ==
=== Current Implementation ===
* Page creation
* Content management
* Watchlist integration

== Example Lists ==
=== Ordered List ===
# First item
# Second item
# Third item

=== Unordered List ===
* Item A
* Item B
* Item C

== References ==
<references/>

[[Category:Test pages]]
[[Category:API created pages]]"#;

    let should_watch = true;

    match login_and_edit_page(page_title, page_content, should_watch).await {
        Ok(_) => println!("Page 'Testpage4' has been created successfully"),
        Err(e) => println!("Error creating page: {}", e),
    }

    Ok(())
}
