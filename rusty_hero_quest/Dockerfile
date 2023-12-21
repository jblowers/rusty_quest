# Use an official Rust image as a parent image
FROM rust:latest

# Set the working directory in the container
WORKDIR /usr/src/rusty_quest

# Copy the current directory contents into the container at /usr/src/myapp
COPY . .

# Compile the application
RUN cargo install --path .

# Make port available to the world outside this container
EXPOSE 3030

# Run the application when the container launches
#RUN cargo run
CMD ["rusty_hero_quest"]
