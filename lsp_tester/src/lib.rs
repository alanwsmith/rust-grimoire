#![allow(unused)]
use anyhow::Result;
use serde_json::Value;
use std::path::PathBuf;
use std::process::*;
use std::{io::*, vec};

pub struct LspTester {
  output: Option<Value>,
}

impl LspTester {
  pub fn process_input(
    input: Vec<LspMessage>
  ) -> Option<Value> {
    let lt = LspTester { output: None };

    lt.output

    //Some(serde_json::to_value("{}")?
  }
}

pub enum LspMessage {
  Request,
  Notificaion,
}

pub fn run_test() {
  let path = PathBuf::from(
    "/Users/alan/workshop/rust-grimoire/rust_analyzer_lsp_example/target/debug/rust_analyzer_lsp_example",
  );
  // let path = PathBuf::from(
  //   "/Users/alan/.cargo/bin/rust-analyzer",
  // );
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

  //  dbg!(&r1);

  let mut line = String::new();
  // dbg!(&child_in);
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

  //child_in.flush();
  //child_out.read_to_string(&mut line).unwrap();
  // println!("OUTPUT: {}", line);
  //dbg!(&child_in);

  // let (r2, counter) =
  //   make_request("initialized", r#"{ }"#, counter)
  //     .unwrap();
  // // let mut line = String::new();
  // child_in.write(r2.as_bytes()).unwrap();
  // child_out.read_to_string(&mut line).unwrap();
  // println!("{}", line);

  let ecode = child_shell
    .kill()
    .expect("failed to wait on child");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn run_a_test() {
    let input = vec![LspMessage::Request];

    let lt = LspTester::process_input(input);
  }
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
