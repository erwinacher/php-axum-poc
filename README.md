# The Three P’s (…and a Rusty Plot Twist)  
### A Minimal LAPP Stack + Rust Hybrid Architecture

This repository is a tiny proof-of-concept demonstrating a fast, modern, low-overhead architecture using:

- **Linux**
- **Axum (Rust) (not Apache)**
- **PHP 8 (FPM)**
- **PostgreSQL**

…and then tying it all together behind **Nginx**.

Yes — it’s basically a LAPP stack with a Rust-powered engine underneath.  
And yes — it’s much faster and lighter than most JS-heavy stacks today.


## Why This Exists

Because nostalgia met modern engineering.

PHP was the first backend many of us touched.  
Rust is the systems language we wish we always had.

Put them together and you get:

- PHP, the super fast SSR, templates, routing  
- Rust, the CPU-heavy logic, parallel work, binary-safe pipelines  
- PHP and Rust, simple HTTP call (todo for later: binary protocol or unix socket)

This hybrid architecture achieves:

- very low memory usage (~40 MB total in this POC)
- fast request handling
- clear separation of concerns
- extremely low infra cost
- very fun developer ergonomics

## Architecture Overview

- Nginx serves PHP pages.
- PHP renders the view, then calls Rust for any “heavy lifting”.
- Rust responds with a fast JSON response (later: binary protocol).
- PostgreSQL is included but optional in this POC.

## Run the Demo

Make sure Docker + Docker Compose are installed.

```bash
docker-compose up --build
```
You should see:

```
Hello from PHP!
Rust server says: Hello, World! (from Rust/Axum server)
```

<img width="2049" height="1809" alt="Screenshot from 2025-11-15 17-04-41" src="https://github.com/user-attachments/assets/de78762c-ba68-4611-a4f7-22bc14f4a200" />
