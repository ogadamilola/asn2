use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::HashMap;

type ChatRoom = Arc<Mutex<HashMap<String, Vec<String>>>>;

fn handle_client(mut stream: TcpStream, chat_rooms: ChatRoom) {
    let mut buffer = [0; 512];
    let mut current_room = String::new();

    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    break;
                }
                let message = String::from_utf8_lossy(&buffer[..size]).trim().to_string();
                println!("Received: {}", message);

                let parts: Vec<&str> = message.split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }

                match parts[0] {
                    "CREATE" => {
                        if parts.len() < 2 {
                            stream.write(b"ERROR: Room name required.\n").unwrap();
                            continue;
                        }
                        let room_name = parts[1].to_string();
                        let mut rooms = chat_rooms.lock().unwrap();
                        if rooms.contains_key(&room_name) {
                            stream.write(b"ERROR: Room already exists.\n").unwrap();
                        } else {
                            rooms.insert(room_name.clone(), Vec::new());
                            stream.write(b"Room created.\n").unwrap();
                        }
                    }
                    "LIST" => {
                        let rooms = chat_rooms.lock().unwrap();
                        let room_list = rooms.keys().cloned().collect::<Vec<String>>().join(", ");
                        stream.write(format!("Rooms: {}\n", room_list).as_bytes()).unwrap();
                    }
                    "JOIN" => {
                        if parts.len() < 2 {
                            stream.write(b"ERROR: Room name required.\n").unwrap();
                            continue;
                        }
                        let room_name = parts[1].to_string();
                        let mut rooms = chat_rooms.lock().unwrap();
                        if rooms.contains_key(&room_name) {
                            current_room = room_name.clone();
                            let messages = &rooms[&current_room];
                            for msg in messages {
                                stream.write(format!("{}\n", msg).as_bytes()).unwrap();
                            }
                            stream.write(b"Joined room.\n").unwrap();
                        } else {
                            stream.write(b"ERROR: Room does not exist.\n").unwrap();
                        }
                    }
                    "LEAVE" => {
                        if current_room.is_empty() {
                            stream.write(b"ERROR: You are not in a room.\n").unwrap();
                        } else {
                            current_room.clear();
                            stream.write(b"Left the room.\n").unwrap();
                        }
                    }
                    "SEND" => {
                        if current_room.is_empty() {
                            stream.write(b"ERROR: You are not in a room.\n").unwrap();
                        } else {
                            let message_text = parts[1..].join(" ");
                            let mut rooms = chat_rooms.lock().unwrap();
                            if let Some(room_messages) = rooms.get_mut(&current_room) {
                                room_messages.push(message_text.clone());
                                stream.write(b"Message sent.\n").unwrap();
                            }
                        }
                    }
                    _ => {
                        stream.write(b"ERROR: Unrecognized command.\n").unwrap();
                    }
                }
            }
            Err(_) => {
                println!("Connection with client lost.");
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("server:8080")?;
    let chat_rooms: ChatRoom = Arc::new(Mutex::new(HashMap::new()));

    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let chat_rooms = Arc::clone(&chat_rooms);
                
                // Spawn a new thread for each client connection
                thread::spawn(move || {
                    handle_client(stream, chat_rooms);
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

// just tryna make a change here