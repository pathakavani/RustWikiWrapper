# RustWikiWrapper
A rust crate that wraps the Wikipedia APIs like MediaWiki, Wikidata, REST, and Pageview APIs into a unified and easy-to-use interface. The crate will simplify the interactions with Wikipedia, allowing developers to query articles, fetch structured data, perform searches, and retrieve analytics effortlessly.

# How the Code Works

The code is structured to separate the concerns of making API calls and handling the data returned by these APIs. Here’s a brief overview of how different components interact:

Interaction Between APIs and Models

API Module (src/api):
	The api module is divided into submodules for the Action API and the REST API.
	Each submodule contains the code for interacting with a specific endpoint (e.g., get_page_summary.rs for retrieving page   summaries).
	These modules define functions that construct URLs, make HTTP requests using reqwest, and handle responses.
Models Module (src/models):
	The models module defines the Rust structures that correspond to the JSON responses returned by the APIs.
	The serde crate is used to deserialize JSON into these structures, making it easy to work with the data in a structured way.
Code Interaction:
	The API modules use functions like get_page_summary or search_pages to retrieve data from Wikipedia.
	The returned JSON is parsed into structs defined in the models module.
	This separation of API and data models makes the codebase clean, maintainable, and easy to extend.


# Dependencies in Cargo.toml

[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }

# Dependency Breakdown

1.	reqwest:
•	A popular HTTP client library in Rust that supports asynchronous requests. It’s used to make HTTP requests to the Wikipedia APIs.
•	The json feature is enabled to seamlessly handle JSON data.
2.	serde:
•	A framework for serializing and deserializing Rust data structures. The derive feature is used to automatically implement Deserialize and Serialize traits for our models.
3.	tokio:
•	An asynchronous runtime that powers the async/await syntax in Rust. It is necessary for running async operations like HTTP requests.

# Running the Code

1. Prerequisites

Make sure you have Rust installed. If not, you can install it by following the instructions at the Rust official website.

2. Build the Project

To build the project, navigate to the root directory of the project and run:
cargo build

3. Run the Examples

To run an example from the examples directory, use the following command:
cargo run --example example_name

4. Running Tests

To run all tests defined in the project, use:
cargo test

# Using the Library in Your Own Project

If you want to use this library in your own project, add the following to your Cargo.toml:

[dependencies]
rust_wiki_wrapper = { path = "../RustWikiWrapper" }  # Adjust the path to point to your local project

Then, in your Rust code, you can use the library as follows:

use rust_wiki_wrapper::api::rest::get_page_summary;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let summary = get_page_summary(&client, "Jupiter").await.unwrap();
    println!("Page Summary: {:?}", summary);
}
This code snippet shows how to use the library to get the summary of a Wikipedia page using the REST API.