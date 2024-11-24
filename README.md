# RustWikiWrapper
A rust crate that wraps the Wikipedia APIs like MediaWiki; REST, and Pageview APIs into a unified and easy-to-use interface. The crate will simplify the interactions with Wikipedia, allowing developers to query articles, fetch structured data, and perform searches effortlessly.

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


## Running the Code in a Docker Container

### Prerequisites

Make sure you have Rust and Docker installed.

If not, you can install it by following the instructions at the [Rust official website](https://www.rust-lang.org/tools/install) and [Docker](https://docs.docker.com/engine/install/)


### Build the Docker Image

To build the Docker image for this project, run the following command in the project root directory (where the `Dockerfile` is located). We will be using the Dockerfile which have the Rust container setup, setting up the directory in the container, copying the local rust code in the container, building, and executing the examples.

##### NOTE: Every time we make a change, we will have to build the container, so that all the changes are copied into the container directory.

```bash
docker build -t rust-wiki-wrapper .
```

### Run the Docker Container

To run the project and execute all example scripts sequentially with a delay, use the following command to run the above built container:
This should only be run when you have to run all the examples.

In the dockerfile, please uncomment the following line:

```bash
# CMD ["/usr/local/bin/run_examples.sh"]
```
and comment the below line:
```bash
CMD /bin/bash
```

and then run the container as below:
```bash
docker run --rm rust-wiki-wrapper
```

This will run the run_examples.sh script, which will execute each example one by one, with a 3-second delay between each, and print the descriptions and results of each example to the terminal.

## Run the docker container interactively and mount the cargo doc!
We can run the docker container in interactive mode so that we are able to run our rust program using the commands we want.

Once the code changes are completed, build the image using the above mentioned command, and run the following:

```bash
docker run -it --rm -v $(pwd)/target/doc:/usr/src/rustwikwrapper/target/doc rust-wiki-wrapper
```
#### Here, we are mounting the local host volume in the container, so that we can create cargo doc from within the container, and access it in the target/doc/rust_wiki_wrapper/index.html easily.

This will open a command line bash interface, after which you can test out all the functionality within the container, exactly as we did locally.

To access the cargo docs, access this link: http://localhost:8080

```bash
cargo build    # build the project

cargo run --example login_example # test all the examples from the examples directory

cargo doc --open # this will create docs in target/doc/rust_wiki_wrapper/index.html directory, and easily accessible.

cargo +nightly fuzz run fuzz_login # Command for fuzzing

cargo test --test {name_of_test_in_the_test_directory}
# or any other commands to test.
```

### Customizing the Example Execution
We have created a run_example.sh file to to execute all the examples which utilize the RustWikiWrapper crate. We can modify this script to change the order of execution, add delays within examples etc.

```bash
#!/bin/bash

# Function to run an example with a description and a delay
run_example() {
    echo "Running example: $1"
    echo "Description: $2"
    echo "----------------------------------------"
    cargo run --example $1
    echo "----------------------------------------"
    echo "Example $1 completed."
    echo "----------------------------------------"
    sleep 3  # Delay between examples (3 seconds)
}

# Run each example sequentially with a description
run_example "login_example" "This example demonstrates how to log in a user to the Wikipedia API."
run_example "edit_page_eg" "This example shows how to edit a page on Wikipedia."
run_example "email_a_user_eg" "This example demonstrates how to email a user via the Wikipedia API."
run_example "get_current_userinfo_eg" "This example retrieves the current user's information."
run_example "get_page_content_eg" "This example fetches the content of a Wikipedia page."
run_example "get_page_summary_eg" "This example retrieves the summary of a Wikipedia page."
run_example "logout_example" "This example logs out a user from the Wikipedia API."
run_example "search_eg" "This example demonstrates how to search for a Wikipedia article."

echo "All examples have been executed."
```

## Project Directory Structure
We have created the directory structure using the 'tree' command as follows:

tree -I target > project.tree

(Here, we have ignored the contents of the /target directory)

Click the below link to go to the dir structure:
[Directory Structure](RustWikiWrapper/project.tree)