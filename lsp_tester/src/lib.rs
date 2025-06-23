#![allow(unused)]
use anyhow::Result;
use serde_json::Value;
use std::path::PathBuf;
use std::process::Child;
use std::process::ChildStdin;
use std::process::ChildStdout;
use std::process::Command;
use std::process::Stdio;
use std::{io::*, vec};

pub struct LspTester {
  _child_in: ChildStdin,
  _child_out: BufReader<ChildStdout>,
  _child_shell: Child,
  _input: Vec<LspMessage>,
  counter: usize,
  output: Option<Value>,
}

impl LspTester {
  pub fn test_input(
    path: &PathBuf,
    input: Vec<LspMessage>,
  ) -> Option<Value> {
    let mut child_shell = Command::new(path)
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn()
      .expect("failed to open connection");

    let mut lt = LspTester {
      _child_in: child_shell.stdin.take().unwrap(),
      _child_out: BufReader::new(
        child_shell.stdout.take().unwrap(),
      ),
      _child_shell: child_shell,
      _input: input,
      counter: 0,
      output: None,
    };

    lt.process_input();

    lt._child_shell
      .kill()
      .expect("could not kill child process");
    lt.output
  }

  pub fn process_input(&mut self) {
    for input in self._input.iter() {
      match input {
        LspMessage::Notification { method, params } => {
          let json_string = format!(
            r#"{{"jsonrpc": "2.0", "method": "{}", "params": {}}}"#,
            method, params
          );
          let content_length = json_string.len();
          let payload = format!(
            "Content-Length: {}\r\n\r\n{}",
            content_length, json_string
          );
          let mut line = String::new();
          self
            ._child_in
            .write_all(payload.as_bytes())
            .unwrap();
        }
        LspMessage::Request { method, params } => {
          self.counter = self.counter + 1;
          let json_string = format!(
            r#"{{"jsonrpc": "2.0", "method": "{}", "id": {}, "params": {}}}"#,
            method, self.counter, params
          );
          let content_length = json_string.len();
          let payload = format!(
            "Content-Length: {}\r\n\r\n{}",
            content_length, json_string
          );
          let mut line = String::new();
          self
            ._child_in
            .write_all(payload.as_bytes())
            .unwrap();
          let mut header = [0; 16];
          self._child_out.read_exact(&mut header);
          let mut vec_for_number_to_get: Vec<u8> =
            vec![];
          self
            ._child_out
            .read_until(13, &mut vec_for_number_to_get);
          let number_string =
            str::from_utf8(&vec_for_number_to_get)
              .unwrap()
              .trim();
          // remove the whitespace
          let mut chomper = [0; 3];
          self._child_out.read_exact(&mut chomper);
          let num =
            number_string.parse::<usize>().unwrap();
          let mut delivery: Vec<u8> =
            Vec::with_capacity(num);
          delivery.resize(num, 0);
          let _ = self._child_out.read(&mut delivery);
          let json_string =
            str::from_utf8(&delivery).unwrap();
          self.output = Some(
            serde_json::to_value(json_string).unwrap(),
          );
        }
      }
    }
  }
}

#[derive(Debug)]
pub enum LspMessage {
  Request { method: String, params: String },
  Notification { method: String, params: String },
}

pub fn run_test() {
  let path = PathBuf::from(
    "/Users/alan/workshop/rust-grimoire/rust_analyzer_lsp_example/target/debug/rust_analyzer_lsp_example",
  );
  let mut child_shell = Command::new(path)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to open connection");
  let mut child_in =
    BufWriter::new(child_shell.stdin.take().unwrap());
  let mut child_out = BufReader::new(
    child_shell.stdout.as_mut().unwrap(),
  );
  let mut counter = 0;
  let (r1, counter) = make_request(
    "initialize",
    r#"{ "capabilities": {}}"#,
    counter,
  )
  .unwrap();
  let mut line = String::new();
  child_in.write_all(r1.as_bytes()).unwrap();
  child_in.flush();

  let mut header = [0; 16];
  child_out.read_exact(&mut header);
  let mut vec_for_number_to_get: Vec<u8> = vec![];
  // let _ = vec_for_number_to_get.pop();
  child_out.read_until(13, &mut vec_for_number_to_get);
  let number_string =
    str::from_utf8(&vec_for_number_to_get)
      .unwrap()
      .trim();
  // remove the whitespace
  let mut chomper = [0; 3];
  child_out.read_exact(&mut chomper);
  let num = number_string.parse::<usize>().unwrap();
  //let mut delivery = [0; 10];
  //child_out.read_exact(&mut delivery);
  let mut delivery: Vec<u8> = Vec::with_capacity(num);
  delivery.resize(num, 0);
  let _ = child_out.read(&mut delivery);
  let json_string = str::from_utf8(&delivery).unwrap();
  println!("{}", &json_string);

  // let number = u8::from_ne_bytes(
  //   vec_for_number_to_get.try_into().unwrap(),
  // );
  // dbg!(&number);
  //dbg!(vec_for_number_to_get[0] as u32);
  // for byte in child_out.bytes() {
  //   println!("{}", byte.unwrap());
  // }
  let ecode = child_shell
    .kill()
    .expect("failed to wait on child");
}

fn make_request(
  method: &str,
  params: &str,
  counter: usize,
) -> Result<(String, usize)> {
  match method {
    "initialized" => {
      let json_string = format!(
        r#"{{"jsonrpc": "2.0", "method": "{}", "params": {}}}"#,
        method, params
      );
      let content_length = json_string.len();
      let payload = format!(
        "Content-Length: {}\r\n\r\n{}",
        content_length, json_string
      );
      Ok((payload, counter))
    }
    _ => {
      let new_counter = counter;
      let json_string = format!(
        r#"{{"jsonrpc": "2.0", "method": "{}", "id": {}, "params": {}}}"#,
        method, new_counter, params
      );
      let content_length = json_string.len();
      let payload = format!(
        "Content-Length: {}\r\n\r\n{}",
        content_length, json_string
      );
      Ok((payload, new_counter))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn run_a_test() {
    let path = PathBuf::from(
      "/Users/alan/workshop/rust-grimoire/rust_analyzer_lsp_example/target/debug/rust_analyzer_lsp_example",
    );
    let input = vec![
      LspMessage::Request {
        method: "initialize".to_string(),
        params: r#"{ "capabilities": {}}"#.to_string(),
      },
      LspMessage::Notification {
        method: "initialized".to_string(),
        params: r#"{}"#.to_string(),
      },
      LspMessage::Notification {
        method: "textDocument/didOpen".to_string(),
        params: r#"{ "textDocument": { "uri": "file:///some-path.txt", "languageId": "en", "version": 1, "text": "this is the text" } }"#.to_string(),
      },

      LspMessage::Request {
        method: "textDocument/formatting".to_string(),
        params: r#"{ "textDocument": { "uri": "file:///some-path.txt" } , "options": { "tabSize": 4, "insertSpaces": false, "trimTrailingWhitespace": false, "insertFinalNewline": false, "trimFinalNewlines": false }}"#.to_string(),
      },

    ];
    let output = LspTester::test_input(&path, input);
    dbg!(output);
  }
}
