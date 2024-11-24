# Use the official Rust image from Docker Hub as the base image
FROM rust:latest

# Create and set the working directory inside the container
WORKDIR /usr/src/rustwikwrapper

# Copy Cargo.toml and Cargo.lock first to leverage Docker's cache mechanism
COPY Cargo.toml Cargo.lock ./

# Copy the entire project into the container
COPY . .

# Copy the shell script into the container
COPY run_examples.sh /usr/local/bin/run_examples.sh

# Make the shell script executable
RUN chmod +x /usr/local/bin/run_examples.sh

# Build the project in debug mode
RUN cargo build

# Generate documentation (no dependencies in the documentation!)
RUN cargo doc --no-deps

# Execute the shell script that runs all examples
# CMD ["/usr/local/bin/run_examples.sh"]

# Start the HTTP server in the background and run the shell (keeping the container running)
CMD /bin/bash