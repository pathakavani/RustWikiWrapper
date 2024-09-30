# RustWikiWrapper
A rust crate that wraps the Wikipedia APIs like MediaWiki, Wikidata, REST, and Pageview APIs into a unified and easy-to-use interface. The crate will simplify the interactions with Wikipedia, allowing developers to query articles, fetch structured data, perform searches, and retrieve analytics effortlessly.
# Directory Structure
.
├── Cargo.lock                              # Auto-generated file tracking exact versions of dependencies
├── Cargo.toml                              # Project dependencies and metadata
├── examples                                # Directory for example scripts demonstrating library usage
│   └── example.rs                          # Example code showing how to use the library
├── project.tree                            # File capturing the directory structure output from the `tree` command
└── src                                     # Main source code directory
    ├── api                                 # API modules for handling different API endpoints
    │   ├── action                          # Modules related to MediaWiki Action API
    │   │   ├── create_account.rs            # Create account module
    │   │   ├── create_edit_page.rs          # Create and edit page module
    │   │   ├── get_current_user.rs          # Get current logged-in user information
    │   │   ├── get_page_content.rs          # Get content of a specific page
    │   │   ├── login.rs                     # Login module for authentication
    │   │   ├── logout.rs                    # Logout module for ending sessions
    │   │   ├── mod.rs                       # Module file for Action API
    │   │   ├── send_email.rs                # Send email through Wikipedia API
    │   │   └── upload_file.rs               # Upload file to Wikipedia
    │   ├── mod.rs                          # Main API module file
    │   └── rest                            # Modules related to MediaWiki REST API
    │       ├── get_metadata.rs              # Get metadata of a page
    │       ├── get_page_summary.rs          # Get summary of a page
    │       ├── mod.rs                       # Module file for REST API
    │       ├── related_pages.rs             # Get related pages
    │       └── search.rs                    # Search for pages using the REST API
    ├── lib.rs                              # Entry point for the library
    └── models                              # Data models for JSON deserialization of API responses
        ├── action                          # Models for Action API responses
        │   ├── create_account.rs            # Model for account creation response
        │   ├── email_status.rs              # Model for email status response
        │   ├── login_response.rs            # Model for login response
        │   ├── logout_response.rs           # Model for logout response
        │   ├── mod.rs                       # Module file for Action models
        │   ├── page_content.rs              # Model for page content response
        │   ├── upload_response.rs           # Model for upload response
        │   └── user_info.rs                 # Model for user information response
        ├── mod.rs                          # Main models module file
        └── rest                            # Models for REST API responses
            ├── metadata.rs                  # Model for metadata response
            ├── mod.rs                       # Module file for REST models
            ├── page_summary.rs              # Model for page summary response
            ├── related_pages.rs             # Model for related pages response
            └── search_results.rs            # Model for search results response

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