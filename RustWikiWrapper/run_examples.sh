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