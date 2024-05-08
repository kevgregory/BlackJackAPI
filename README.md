# BlackJackAPI

The BlackJackAPI provides a simple backend for simulating BlackJack games between two automated players. It also features a score tracking system.

## Requirements

To run this project, the following needs to be installed:
- Rust
- Cargo
- PostgreSQL
- Docker

This project is built with Rocket

## Setup

### Database Setup
1. Run
```
docker run -p 5432:5432 --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres
```
   to start the local Postgres databse in Docker
   
2. Enter the `api` folder and run
```
cargo install sqlx-cli --no-default-features --features rustls,postgres
 ```

3. To initialize the data base, run this command in the api folder
```
sqlx migrate run --database-url postgres://postgres:mysecretpassword@localhost:5432/postgres
```

4. Before running the application, ensure that all dependencies are correctly installed by running `cargo build`

5. To start the server, use the following command: 
```
Rust_LOG=info cargo run
```
This command starts the Rocket server and makes the BlackJackAPI available on `http://localhost:3000/`.

## Available Endpoints
* `Get /` : Displays a welcome message and instructions.
* `Get /start`: Simulates a new game of Blackjack, updates scores based on the results, and returns the game result. (Player1/Player2/Draw)
* `Get /scores`: Retrieves the current scores from the database.
* `Post /reset-scores`: Resets the scores of both players to zero.

## Example Usage
- **Start a Game:** Navigate to `http://localhost:3000/start` or use a tool like `curl`:
  ```
  curl http://localhost:3000/start
  ```
- **View Scores:** Navigate to `http://localhost:3000/scores` or use `curl`:
```
curl http://localhost:3000/scores
```
-**Reset Scores:** Use `curl` to post to the reset endpoint:
```
curl -X POST http://localhost:3000/reset-scores
```


## Screenshots:
<img width="691" alt="Screenshot 2024-05-07 at 11 53 53 PM" src="https://github.com/kevgregory/BlackJackAPI/assets/143756578/7bcbc7d8-aef3-4b92-a927-f365e75e433c">

<img width="452" alt="Screenshot 2024-05-07 at 11 54 05 PM" src="https://github.com/kevgregory/BlackJackAPI/assets/143756578/ce4e8955-b14a-4f00-9345-7a9addf478fa">

<img width="289" alt="Screenshot 2024-05-07 at 11 54 12 PM" src="https://github.com/kevgregory/BlackJackAPI/assets/143756578/ed69ac5c-368d-49ca-8c1f-8fcda3793eea">

<img width="323" alt="Screenshot 2024-05-07 at 11 54 19 PM" src="https://github.com/kevgregory/BlackJackAPI/assets/143756578/4112b575-b118-437f-ada1-6b772bb86067">

<img width="326" alt="Screenshot 2024-05-07 at 11 54 25 PM" src="https://github.com/kevgregory/BlackJackAPI/assets/143756578/eb70a86e-0ec1-4972-92fb-cbf7cc543b28">
