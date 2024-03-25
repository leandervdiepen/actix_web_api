# Actix Web Example Project

This project is a simple web application built with Actix Web, a powerful, pragmatic, and extremely fast web framework for Rust.

## Features

- **Hello World**: A simple route that responds with "Hello world".
- **User Management**: Basic CRUD operations for user management.

## Requirements

- Rust 1.41 or higher
- Cargo
- MongoDB

## Getting Started

To get started with this project, clone the repository and navigate into the project directory.

To start the project run:
```bash
cargo run
```

## MongoDB Integration

This project uses MongoDB for persistent storage of user data, leveraging the MongoDB Rust driver for database operations within the Actix Web framework.

### Setting Up MongoDB

Before running the application, ensure you have MongoDB installed and running on your system. You can download MongoDB from [the official site](https://www.mongodb.com/try/download/community) and find installation instructions for your operating system.

### Configuring MongoDB Connection

The application connects to MongoDB using the `establish_connection` function defined in the `db` module. This function initializes a MongoDB client and connects to a database named `db`. You can modify the database name and connection details (e.g., host, port) as needed in the `src/db/connection.rs` file.

### Using MongoDB in the Application

The application's state (`AppState`) includes a MongoDB `Database` instance, which is passed to route handlers to perform CRUD operations on the `users` collection.

## Example Usage

- **Create User**: To add a new user, send a `POST` request to `/users` with a JSON body containing the user's details. For example:

```bash
curl -X POST http://localhost:8080/users -H "Content-Type: application/json" -d '{"id": 1, "name": "John Doe", "email": "john@example.com"}'
```

- **Get User**: To retrieve a user by their ID, send a `GET` request to `/users/{id}`. Replace `{id}` with the actual user ID. For example:

```bash
curl -X GET http://localhost:8080/users/1
```

- **Update User**: To update a user's details, send a `PUT` request to `/users/{id}` with a JSON body containing the updated user details. For example:

```bash
curl -X PUT http://localhost:8080/users/1 -H "Content-Type: application/json" -d '{"name": "Jane Doe", "email": "jane@example.com"}'
```

- **Delete User**: To delete a user by their ID, send a `DELETE` request to `/users/{id}`. Replace `{id}` with the actual user ID. For example:

```bash
curl -X DELETE http://localhost:8080/users/1
```
