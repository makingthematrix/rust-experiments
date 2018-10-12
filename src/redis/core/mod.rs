use std::collections::VecDeque;
use std::iter::Iterator;
use std::net::TcpStream;

use super::streams;

enum Behaviour {
    Continue,
    Result(String),
    Stop,
}

fn perform(command: &str, queue: &mut VecDeque<String>) -> Result<Behaviour, String> {
    if command.starts_with("PUBLISH") {
        if command.trim() == "PUBLISH" {
            Err("Error: Empty PUBLISH".into())
        } else {
            (&command[8..])
                .split(',')
                .for_each(|word| queue.push_back(word.trim().into()));
            Ok(Behaviour::Continue)
        }
    } else if command.starts_with("RETRIEVE") {
        if let Some(word) = queue.pop_front() {
            Ok(Behaviour::Result(word))
        } else {
            Err("Error: Empty RETRIEVE".into())
        }
    } else if command.starts_with("STOP") {
        Ok(Behaviour::Stop)
    } else {
        Err(format!("Error: Unrecognized command {}", command))
    }
}

fn error(stream: &mut TcpStream, err_str: &str) -> Result<bool, String> {
    let s = format!("Error: {}", err_str);
    streams::write(stream, &s).unwrap();
    Err(s)
}

fn flatten(res: Result<Result<Behaviour, String>, String>) -> Result<Behaviour, String> {
    match res {
        Ok(Ok(behaviour)) => Ok(behaviour),
        Ok(err) => err,
        Err(err) => Err(err),
    }
}

pub fn handle(stream: &mut TcpStream, queue: &mut VecDeque<String>) -> Result<bool, String> {
    let mut res = (Ok(false), false);
    while !res.1 {
        match flatten(streams::read(stream).map(|command| perform(&command, queue))) {
            Ok(Behaviour::Result(word)) => {
                streams::write(stream, &word)
                    .map_err(|err| {
                        res = (error(stream, &err.to_string()), true);
                    })
                    .unwrap();
            }
            Ok(Behaviour::Stop) => {
                res = (Ok(true), true);
            }
            Err(err) => res = (error(stream, &err.to_string()), false),
            _ => {}
        };
    }
    res.0
}
