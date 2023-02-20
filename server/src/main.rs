use std::{sync::Arc, io::Write};
use models::ClientList;
use tokio::{sync::Mutex, net::{TcpListener}};
use owo_colors::{OwoColorize, Style};

use crate::models::Client;
mod commands;
mod common;
mod models;

static LUNA_VER: &str = "v1.0.0-dev";

/* =============================================================================================================================================== */

async fn input(title:String) -> Option<String>{
    let mut stdout = std::io::stdout();
    let mut user_data = "".to_string();
    stdout.write_all(title.as_bytes()).unwrap();
    stdout.flush().unwrap();
    std::io::stdin().read_line(&mut user_data).unwrap();

    if user_data.len() > 0 {
        Some(user_data)
    }
    else {
        None
    }
}

/* =============================================================================================================================================== */

#[tokio::main]
async fn user_interaction(mut clients:ClientList) {
    loop {
        // Ask user for input
        match input("🌙 \x1B[4mluna\x1B[24m > ".to_string()).await {
            Some(cmd) => {
                let cmd = cmd.trim().split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
                let (cmd, args) = (&cmd[0], &cmd[1..]);
                match cmd.to_lowercase().as_str() {
                    "sessions" => commands::sessions(args, &mut clients).await,
                    "clear" => common::clear_console_and_print_banner(),
                    _ => {

                    }
                }
            }
            None => {
                //
            }
        }
    }
}

/* =============================================================================================================================================== */

#[tokio::main]
async fn handle_clients(clients: ClientList) {
    let addr = "127.0.0.1:8080";
    let socket = TcpListener::bind(addr).await.unwrap();
    loop {
        let (conn, sock_addr) = socket.accept().await.unwrap();
        let sock_addr = sock_addr.to_string();
        // println!("\n[+] Received connection from: {}", sock_addr);
        clients.lock().await.push(Client {socket:conn, addr:sock_addr});
    }
}

/* =============================================================================================================================================== */

#[tokio::main]
async fn main() {
    common::clear_console_and_print_banner();

    let clients_user: ClientList = Arc::new(Mutex::new(vec![]));
    let clients_handler = clients_user.clone();
    std::thread::spawn(|| user_interaction(clients_user));
    std::thread::spawn(|| handle_clients(clients_handler));
    loop { }
}
