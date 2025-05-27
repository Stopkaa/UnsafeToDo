use crate::commands::Command;
use crate::todo::Todo;
use serde_cbor::Value;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
pub struct ShowCommand;

impl Command for ShowCommand {
    fn execute(&self, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        show_todo2()
    }
}

pub fn show_todo() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("todos.txt").expect("Nothing to read");
    let reader = BufReader::new(file);
    let mut json_string = String::new();
    for todo in reader.lines() {
        let todo = todo?;
        let todo = todo.replace("\n", "");
        let cbor: Value = serde_cbor::from_slice(todo.as_bytes())?;
        let todo_as_json = serde_json::to_string_pretty(&cbor)?;
        json_string.push_str(&todo_as_json);
    }
    println!("CBOR als JSON:");
    println!("{}", json_string);
    Ok(())
}

fn show_todo2() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = std::fs::read("todos.txt")?;
    let mut offset = 0;
    let mut count = 0;

    while offset < bytes.len() {
        // Versuche CBOR-Objekt zu deserialisieren
        match serde_cbor::from_slice::<Todo>(&bytes[offset..]) {
            Ok(obj) => {
                count += 1;
                println!("Objekt {}: {:?}", count, obj);

                // Berechne die Bytes die gelesen wurden
                let obj_bytes = serde_cbor::to_vec(&obj)?;
                offset += obj_bytes.len();
            }
            Err(e) => {
                println!("Fehler bei Offset {}: {}", offset, e);
                break;
            }
        }
    }
    Ok(())
}
