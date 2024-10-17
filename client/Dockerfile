# Use the official Rust image as a base
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the current directory contents into the container
COPY . .

# Build the client
RUN cargo build --release

# Run the client
CMD ["./target/release/client"]
