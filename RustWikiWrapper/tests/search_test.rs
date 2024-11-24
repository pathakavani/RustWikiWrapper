use rust_wiki_wrapper::api::MediaWikiClient;

fn setup_client() -> MediaWikiClient {
    MediaWikiClient::new("https://en.wikipedia.org")
}

#[tokio::test]
async fn test_basic_search() {
    let client = setup_client();
    let result = client
        .get_search_results("Rust programming language", None)
        .await;

    match result {
        Ok(search_results) => {
            println!("Search found {} results", search_results.query.search.len());

            // Basic validations
            assert!(
                !search_results.query.search.is_empty(),
                "Should return search results"
            );

            // Check first result
            let first_result = &search_results.query.search[0];
            println!("First result: {}", first_result.title);
            println!("Snippet: {}", first_result.snippet);

            assert!(!first_result.title.is_empty(), "Title should not be empty");
            assert!(
                !first_result.snippet.is_empty(),
                "Snippet should not be empty"
            );

            if let Some(wordcount) = first_result.wordcount {
                assert!(wordcount > 0, "Word count should be positive");
            }

            if let Some(timestamp) = &first_result.timestamp {
                assert!(timestamp.contains('T'), "Timestamp should be in ISO format");
            }
        }
        Err(e) => panic!("Search failed: {}", e),
    }
}

#[tokio::test]
async fn test_search_with_limit() {
    let client = setup_client();
    let limit = 5;
    let result = client
        .get_search_results("Python programming", Some(limit))
        .await;

    match result {
        Ok(search_results) => {
            let results_count = search_results.query.search.len();
            println!("Requested {} results, got {}", limit, results_count);
            assert_eq!(
                results_count, limit as usize,
                "Should return exactly the number of results specified"
            );

            // Verify all results have required fields
            for result in search_results.query.search {
                assert!(!result.title.is_empty(), "Title should not be empty");
                assert!(!result.snippet.is_empty(), "Snippet should not be empty");
                if let Some(wordcount) = result.wordcount {
                    assert!(wordcount > 0, "Word count should be positive");
                }
            }
        }
        Err(e) => panic!("Search with limit failed: {}", e),
    }
}

#[tokio::test]
async fn test_search_special_characters() {
    let client = setup_client();
    let result = client
        .get_search_results("C++ programming language", None)
        .await;

    match result {
        Ok(search_results) => {
            assert!(
                !search_results.query.search.is_empty(),
                "Should handle special characters in search term"
            );

            println!("Search results for C++:");
            for result in &search_results.query.search {
                println!("Title: {}", result.title);
                println!("Snippet: {}", result.snippet);
                println!("---");
            }

            // Check if any of the top results are related to C++
            let found_relevant_result = search_results.query.search.iter().any(|result| {
                let title_relevant = result.title.to_lowercase().contains("c++")
                    || result.title.contains("programming")
                    || result.title.contains("language");
                let snippet_relevant = result.snippet.to_lowercase().contains("c++")
                    || result.snippet.contains("programming")
                    || result.snippet.contains("language");

                title_relevant || snippet_relevant
            });

            assert!(
                found_relevant_result,
                "Should find at least one result related to programming or C++"
            );
        }
        Err(e) => panic!("Search with special characters failed: {}", e),
    }
}

#[tokio::test]
async fn test_alternative_special_characters() {
    let client = setup_client();
    let test_terms = [
        "C# programming",
        "Java++ development",
        "F# language",
        ".NET framework",
    ];

    for term in test_terms {
        let result = client.get_search_results(term, None).await;
        match result {
            Ok(search_results) => {
                println!("\nResults for '{}':", term);
                assert!(
                    !search_results.query.search.is_empty(),
                    "Should handle special characters in: {}",
                    term
                );

                // Print first result for debugging
                if let Some(first) = search_results.query.search.first() {
                    println!("First result title: {}", first.title);
                    println!("First result snippet: {}", first.snippet);
                }

                // Check if any result is relevant
                let found_relevant = search_results.query.search.iter().any(|result| {
                    let search_term = term.to_lowercase();
                    let title = result.title.to_lowercase();
                    let snippet = result.snippet.to_lowercase();

                    title.contains(&search_term)
                        || snippet.contains(&search_term)
                        || snippet.contains("programming")
                        || snippet.contains("language")
                        || snippet.contains("development")
                });

                assert!(
                    found_relevant,
                    "Should find at least one relevant result for: {}",
                    term
                );
            }
            Err(e) => panic!("Search failed for '{}': {}", term, e),
        }
    }
}

#[tokio::test]
async fn test_empty_search() {
    let client = setup_client();
    let result = client.get_search_results("", None).await;

    match result {
        Ok(search_results) => {
            println!(
                "Results for empty search: {}",
                search_results.query.search.len()
            );
        }
        Err(e) => {
            println!("Empty search error: {}", e);
            // Both empty results or error are acceptable for empty search
        }
    }
}

#[tokio::test]
async fn test_very_long_search() {
    let client = setup_client();
    let long_search = "a".repeat(1000); // Very long search term
    let result = client.get_search_results(&long_search, None).await;

    match result {
        Ok(search_results) => {
            println!(
                "Results for long search: {}",
                search_results.query.search.len()
            );
            // API might return empty results for very long searches
            assert!(
                !search_results.query.search.is_empty(),
                "Should handle long search terms gracefully"
            );
        }
        Err(e) => {
            println!("Long search error: {}", e);
            // Error is also acceptable for extremely long search terms
        }
    }
}

#[tokio::test]
async fn test_multilingual_search() {
    let client = setup_client();
    let search_terms = ["café", "书籍", "München"];

    for term in search_terms {
        let result = client.get_search_results(term, None).await;
        match result {
            Ok(search_results) => {
                println!(
                    "Results for '{}': {}",
                    term,
                    search_results.query.search.len()
                );
                assert!(
                    !search_results.query.search.is_empty(),
                    "Should handle non-English characters"
                );

                let first_result = &search_results.query.search[0];
                println!("First result title: {}", first_result.title);
            }
            Err(e) => panic!("Multilingual search failed for '{}': {}", term, e),
        }
    }
}

#[tokio::test]
async fn test_search_result_fields() {
    let client = setup_client();
    let result = client.get_search_results("Albert Einstein", Some(1)).await;

    match result {
        Ok(search_results) => {
            let first_result = &search_results.query.search[0];

            // Verify all required fields are present and valid
            assert!(!first_result.title.is_empty(), "Title should be present");
            assert!(
                !first_result.snippet.is_empty(),
                "Snippet should be present"
            );

            if let Some(wordcount) = first_result.wordcount {
                assert!(wordcount > 0, "Word count should be positive");
            }

            // Check timestamp format
            if let Some(timestamp) = &first_result.timestamp {
                assert!(timestamp.contains('T'), "Timestamp should be in ISO format");
                assert!(timestamp.len() > 10, "Timestamp should be complete");
            }

            println!("Result fields:");
            println!("Title: {}", first_result.title);
            println!("Snippet: {}", first_result.snippet);
            println!("Word count: {:?}", first_result.wordcount);
            println!("Timestamp: {:?}", first_result.timestamp);
        }
        Err(e) => panic!("Field validation failed: {}", e),
    }
}

#[tokio::test]
async fn test_rate_limit_handling() {
    let client = setup_client();

    for i in 1..=5 {
        let result = client.get_search_results("test", Some(1)).await;

        match result {
            Ok(_) => println!("Request {} succeeded", i),
            Err(e) => {
                println!("Request {} failed: {}", i, e);
                if e.to_string().contains("rate") {
                    println!("Rate limit detected as expected");
                    return;
                }
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
