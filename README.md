# ime-api

The API is built using the [Actix Web](https://actix.rs/) framework in Rust.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [API Endpoints](#api-endpoints)
- [Configuration](#configuration)
- [Testing](#testing)
- [Contributing](#getting-involved)

## Introduction

The IME-API is designed to provide a wide range of quiz questions for various purposes, such as educational apps, trivia games, and more. It offers a simple and efficient way to access a vast collection of questions.

## Features

- Retrieve all questions or paginate through them.
- Filter questions by category and subcategory.
- Get random questions for quizzes and games.
- Filter questions by type, such as multiple-choice or true/false.

## Getting Started

Follow these instructions to get the IME-API up and running on your local machine.

### Prerequisites

- Install Rust by going to the [Rust](https://www.rust-lang.org/tools/install) page and following the instructions.
|  This installs a tool called "rustup", which lets you manage multiple versions of Rust. By default, it installs the latest stable Rust release, which you can use for general Rust development. Rustup installs `rustc`, the Rust compiler, as well as `cargo`, Rust's package manager, `rust-std`, Rust's standard libraries, and some helpful docs — `rust-docs`.
- Cargo package manager installed.
- JSON data(assets) file with quiz questions.

### Installation

1. Clone the repository:

   ```console
   git clone https://github.com/Islam-Made-Easy/ime-api.git
   cd ime-api
   ```

## Usage

To use the IME-API, make HTTP requests to the provided endpoints. You can use tools like curl or Postman, or integrate it into your application.

## API Endpoints

    GET /api/questions: Retrieve all questions or paginate through them.
    GET /api/questions/{category}/{subcategory}: Filter questions by category and subcategory.
    GET /api/questions/type: Filter questions by type.
    GET /api/questions/random: Get random questions.

For detailed information on each endpoint and their request/response formats, refer to the [API Documentation](https://github.com/Islam-Made-Easy/ime-api/wiki/API-Documentation).

## Testing

You can run tests for the API using the following command:

```console
  cargo test
```

## Getting involved

We welcome and encourage you to report issues and contribute to changes.

- [Contribution guide for developers](https://github.com/Islam-Made-Easy/ime-api/wiki)

[![Open in Gitpod!](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/Islam-Made-Easy/ime-api)
