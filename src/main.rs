use postgres::{Client, NoTls}; //NoTls [non secure connection]
use postgres::Error as PostgresError; // I have no idea
use std::net::{TcpListener, TcpStream}; // Self explanatory
use std::io::{Read, Write}; // input/output
use std::env; // environment variables

#[macro_use]
extern crate serde_derive;

// Model: User struct

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}


// Database_URL [For the conection]

const DB_URL: &str = !env("DATABASE_URL");

// Constants
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

// Main function [Set the database, start the server and call handleClient()]

fn main(){
    //Set database
    if let Err(e) = set_database() {
        println!("Error: {}", e);
        return;
    }


}

fn set_database() -> Result<(), PostgresError> {
    // Connect to database
    let mut client: Client = Client::connect(DB_URL, NoTls)?;

    //Create table
    client.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
            )",
         &[]
    )?;
}


