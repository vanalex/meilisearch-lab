# Meilisearch Lab

A simple Rust application demonstrating how to use [Meilisearch](https://www.meilisearch.com/) for full-text search capabilities.

## Overview

This project showcases Meilisearch operations (under construction)

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2021 edition or newer)
- [Docker](https://docs.docker.com/get-docker/)

## Installation

1. Clone this repository:
   ```
   git clone <repository-url>
   cd meilisearch-lab
   ```

2. Start the Meilisearch server using Docker:
   ```
   # On Linux/macOS
   ./install.sh
   ```

   This will:
   - Pull the Meilisearch Docker image (v1.15)
   - Run Meilisearch in development mode on port 7700
   - Mount the local `meili_data` directory to persist data

3. Build and run the Rust application:
   ```
   cargo build
   cargo run
   ```