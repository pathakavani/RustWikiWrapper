# RustWikiWrapper
A rust crate that wraps the Wikipedia APIs like MediaWiki, Wikidata, REST, and Pageview APIs into a unified and easy-to-use interface. The crate will simplify the interactions with Wikipedia, allowing developers to query articles, fetch structured data, perform searches, and retrieve analytics effortlessly.

## How the Code Works

The code is structured to separate the concerns of making API calls and handling the data returned by these APIs. Here’s a brief overview of how different components interact:

### API Module (src/api):
The api module is divided into submodules for the Action API and the REST API.
Each submodule contains the code for interacting with a specific endpoint (e.g., get_page_summary.rs for retrieving page   summaries).
These modules define functions that construct URLs, make HTTP requests using reqwest, and handle responses.

### Models Module (src/models):
The models module defines the Rust structures that correspond to the JSON responses returned by the APIs.
The serde crate is used to deserialize JSON into these structures, making it easy to work with the data in a structured way.

### Code Interaction:
The API modules use functions like get_page_summary or search_pages to retrieve data from Wikipedia.
The returned JSON is parsed into structs defined in the models module.
This separation of API and data models makes the codebase clean, maintainable, and easy to extend.


## Dependencies in Cargo.toml

1. reqwest = { version = "0.11", features = ["json"] }
A popular HTTP client library in Rust that supports asynchronous requests. It’s used to make HTTP requests to the Wikipedia APIs.
The json feature is enabled to seamlessly handle JSON data.

2. serde = { version = "1.0", features = ["derive"] }
A framework for serializing and deserializing Rust data structures. The derive feature is used to automatically implement Deserialize and Serialize traits for our models.

3. tokio = { version = "1", features = ["full"] }
An asynchronous runtime that powers the async/await syntax in Rust. It is necessary for running async operations like HTTP requests.

## Running the Code

### Prerequisites

Make sure you have Rust installed.

If not, you can install it by following the instructions at the [Rust official website](https://www.rust-lang.org/tools/install).

### Build the Project

To build the project, navigate to the root directory of the project and run:

```cargo build```

### Run the Examples

To run an example from the examples directory, use the following command:

```cargo run --example example_name```


## Use the RustWikiWrapper Crate
Check out the example code [here](RustWikiWrapper/examples/example.rs) to run a particular API by using the crate.

## Project Directory Structure
We have created the directory structure using the 'tree' command as follows:

tree -I target > project.tree

(Here, we have ignored the contents of the /target directory)

Click the below link to go to the dir structure:
[Directory Structure](RustWikiWrapper/project.tree)