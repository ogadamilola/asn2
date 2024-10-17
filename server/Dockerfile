# Use the official Rust image as a base
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the current directory contents into the container
COPY . .

# Build the server
RUN cargo build --release

# Run the server
CMD ["./target/release/server"]
