use colored::*;
use regex::Regex;
use rust_wiki_wrapper::api::MediaWikiClient;
use std::error::Error;

async fn search_wikipedia(query: &str, limit: Option<u32>) -> Result<(), Box<dyn Error>> {
    let client = MediaWikiClient::new("https://en.wikipedia.org");

    let results = client.get_search_results(query, limit).await?;

    // print header
    println!("\n{}", "=".repeat(80).blue());
    println!(
        "{}",
        format!("Search results for '{}'", query).blue().bold()
    );
    println!("{}", "=".repeat(80).blue());

    if let Some(search_info) = &results.search_info {
        if let Some(total) = search_info.total_hits {
            println!("{}: {}\n", "Total matches found".yellow(), total);
        }
        if let Some(suggestion) = &search_info.search_suggestion {
            println!("{}: {}\n", "Did you mean".yellow(), suggestion);
        }
    }

    // create regex to clean HTML tags
    let html_tag_regex = Regex::new(r"<[^>]+>").unwrap();

    for (index, result) in results.query.search.iter().enumerate() {
        // print result number
        println!("{}", format!("Result #{}", index + 1).green().bold());
        println!("{}", "-".repeat(40).green());

        // print title
        println!("{}: {}", "Title".yellow().bold(), result.title);
        println!("{}: {}", "Page ID".yellow(), result.pageid);

        // clean and print snippet
        let clean_snippet = html_tag_regex.replace_all(&result.snippet, "");
        println!("\n{}", "Summary:".yellow());
        println!("{}", clean_snippet);

        // print additional information if available
        if let Some(word_count) = result.wordcount {
            println!("\n{}: {} words", "Length".yellow(), word_count);
        }
        if let Some(timestamp) = &result.timestamp {
            let datetime = chrono::DateTime::parse_from_rfc3339(timestamp)
                .map(|dt| dt.format("%B %d, %Y at %H:%M UTC").to_string())
                .unwrap_or_else(|_| timestamp.to_string());
            println!("{}: {}", "Last modified".yellow(), datetime);
        }

        println!("\n{}", "=".repeat(80).blue());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let query = "Rust programming language";
    let limit = Some(5);

    match search_wikipedia(query, limit).await {
        Ok(_) => println!("\n{}", "Search completed successfully".green()),
        Err(e) => println!("\n{}: {}", "Error during search".red(), e),
    }

    Ok(())
}
