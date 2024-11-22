# Submodule User Example Server

This project is a **Rust**-based server that handles **gRPC** requests. It uses **Tokio**, **Tonic**, and **Prost** for asynchronous operations and gRPC communication. The server is designed to demonstrate how to integrate and work with **Git submodules** in a Rust project, with a focus on using gRPC protocols.

### Submodules
This repository employs a submodule, `submodule-provider-example`, which contains the necessary **gRPC** definition files. These files are essential for the server to interact with other services using gRPC.

- **Submodule:** [submodule-provider-example](https://github.com/eyyyyyyy3/submodule-provider-example)

The submodule is already added in this repository, and all you need to do is **initialize** it locally.

### How to Work with Git Submodules

To work with Git submodules, follow these steps:

1. **Cloning the Repository with Submodules:**

   If you clone the repository and want to include the submodule, use:

   ```bash
   git clone --recurse-submodules https://github.com/eyyyyyyy3/submodule-user-example-server.git
   ```

   If you have already cloned the repository without the `--recurse-submodules` flag, you can initialize the submodule with the following commands:

   ```bash
   git submodule init
   git submodule update
   ```

2. **Updating Submodules:**

   To update the submodule to the latest commit from the upstream repository, use:

   ```bash
   git submodule update --remote
   ```

   This will fetch the latest changes from the submodule's repository.

### Starting the Server

To start the server, follow these steps:

1. **Navigate to the project directory:**

   ```bash
   cd submodule-user-example-server
   ```

2. **Run the server:**

   ```bash
   cargo run
   ```

This will start the server, and it will begin handling incoming gRPC requests.
