use std::{ops::{Deref, DerefMut}};
use owo_colors::{OwoColorize};
use tokio::sync::MutexGuard;

use crate::{ClientList, input, models::Client, common};

/* =============================================================================================================================================== */

pub async fn sessions(args: &[String], clients: &mut ClientList) {
    if args.len() == 0 {
        if clients.lock().await.deref().len() == 0 {
            print!("{}", "[-] No connection found!\n".bold().red())
        } else {
            println!("\n   Id   Information\n   ──   ────────────");
            for (index, client) in clients.lock().await.deref().iter().enumerate() {
                println!("   {}    {}", index + 1, client.addr);
            }
            println!("\n")
        }
    } else {
        match args[0].to_lowercase().as_str() {
            "-i" | "-interact" => {
                if let Some(session_number) = args.get(1) {
                    interact(session_number.parse().unwrap(), clients).await;
                }
            }
            "-h" | "-help" => {
                println!("Help text here")
            }
            _ => {
                //
            }
        }
    }
}

/* =============================================================================================================================================== */

pub async fn help() {

}

/* =============================================================================================================================================== */

async fn interact(session_number: u32, clients: &mut ClientList) {
    let mut client = clients.lock().await;

    println!("{} Starting interaction with {}...\n", "[*]".bold().magenta(), session_number.bold().magenta());
    loop { 
        match input("🌙 \x1B[4mShell\x1B[24m > ".to_string()).await {
            Some(cmd) => {
                if cmd.starts_with("/") {
                    let cmd = cmd[1..].to_string();
                    match shell_commands(&cmd, &mut client, session_number) {
                        Ok(_) => {continue}
                        Err(_) => {break}
                    }
                }

                let client = client.deref_mut().get_mut(session_number as usize - 1).unwrap();
                client.write(cmd.as_bytes()).await;
                let incoming = client.read().await;
                println!("{}", String::from_utf8(incoming).unwrap());
            }
            None => { continue; }
        }
    }
}

/* =============================================================================================================================================== */

fn shell_commands(cmd: &String, clients: &mut MutexGuard<Vec<Client>>, session_number: u32) -> Result<(), ()> {
    match cmd.to_lowercase().as_str().trim() {
        "r" | "return" => {
            common::clear_console_and_print_banner();
            Err(())
        }
        "e" | "exit" => {
            clients.deref_mut().remove(session_number as usize - 1);
            Err(())
        }
        _ => {Ok(())}
    }
}