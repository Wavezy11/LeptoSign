# LeptoSign
A full-stack newsletter subscription app built in Rust using Leptos for the frontend, Axum + SQLx for the backend, PostgreSQL for storage, and CORS for frontend-backend communication.


* **Frontend:** [Leptos](https://leptos.dev) (`/leptos-project`)
* **Backend:** [Axum](https://github.com/tokio-rs/axum) + [SQLx](https://github.com/launchbadge/sqlx) (`/leptos-backend`)
* **Database:** PostgreSQL
* **Deployment:** Docker setup in `/docker`

---

## 🚀 Project Structure

```
.
├── leptos-project     # Frontend (Leptos + Trunk)
├── leptos-backend     # Backend (Axum + SQLx + PostgreSQL)
└── docker             # Docker & docker-compose setup
```

---

## ⚙️ Prerequisites

* [Rust](https://www.rust-lang.org/tools/install) (latest stable)
* [Trunk](https://trunkrs.dev/#install) for serving the frontend
* [PostgreSQL](https://www.postgresql.org/download/) (only if not using Docker)
* [Docker](https://docs.docker.com/get-docker/) & [Docker Compose](https://docs.docker.com/compose/)

> ⚡️ All Rust dependencies are managed via **Cargo**, no extra setup needed.

---

## 🗄 Database Setup

### Locally

1. Start PostgreSQL
2. Create a database (e.g. `newsletter`):

   ```bash
   createdb newsletter
   ```
3. Set the environment variable (adjust credentials if needed):

   ```bash
   export DATABASE_URL=postgres://user:password@localhost/newsletter
   ```
4. Run migrations:

   ```bash
   cargo install sqlx-cli --no-default-features --features native-tls,postgres
   sqlx migrate run
   ```

### With Docker

* When you run `docker-compose up` from the `/docker` folder, PostgreSQL will automatically start.
* Make sure your backend is configured to connect with the Docker DB (check `DATABASE_URL` in `.env`).
* If you need migrations inside Docker, you can run:

  ```bash
  docker exec -it leptos-backend sqlx migrate run
  ```

---

## ▶️ Running Locally

### Backend

```bash
cd leptos-backend
cargo run
```

### Frontend

```bash
cd leptos-project
trunk serve --open
```

This will open the app automatically in your browser.

---

## 🐳 Running with Docker

1. Go to the `docker` folder:

   ```bash
   cd docker
   ```

2. Start everything (frontend, backend, and database):

   ```bash
   docker-compose up --build
   ```

---

## 📌 Notes

* All dependencies are handled via `cargo build`
* Database migrations are required before the backend can run queries
* Trunk is used for serving the Leptos frontend

---
