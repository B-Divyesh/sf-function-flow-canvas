use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

pub struct LspClient {
    child: Child,
    input: ChildStdin,
    messages: Receiver<Result<Value, String>>,
    next_id: u64,
    timeout: Duration,
}

impl LspClient {
    pub fn start(command: &str, args: &[String], timeout: Duration) -> Result<Self, String> {
        let stderr = if std::env::var_os("FFC_DEBUG").is_some() {
            Stdio::inherit()
        } else {
            Stdio::null()
        };
        let mut child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(stderr)
            .spawn()
            .map_err(|error| format!("could not start `{command}`: {error}"))?;
        let input = child
            .stdin
            .take()
            .ok_or("language server stdin unavailable")?;
        let output = child
            .stdout
            .take()
            .ok_or("language server stdout unavailable")?;
        let (sender, messages) = mpsc::channel();

        thread::spawn(move || {
            let mut reader = BufReader::new(output);
            loop {
                let parsed = read_message(&mut reader);
                let done = parsed.is_err();
                if sender.send(parsed).is_err() || done {
                    break;
                }
            }
        });

        Ok(Self {
            child,
            input,
            messages,
            next_id: 1,
            timeout,
        })
    }

    pub fn request(&mut self, method: &str, params: Value) -> Result<Value, String> {
        let id = self.next_id;
        self.next_id += 1;
        self.send(&json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}))?;

        loop {
            let message = self
                .messages
                .recv_timeout(self.timeout)
                .map_err(|_| format!("language server timed out while waiting for `{method}`"))??;
            if message.get("id").and_then(Value::as_u64) == Some(id) {
                if let Some(error) = message.get("error") {
                    return Err(format!("language server rejected `{method}`: {error}"));
                }
                return Ok(message.get("result").cloned().unwrap_or(Value::Null));
            }
            if message.get("method").is_some() && message.get("id").is_some() {
                let response_id = message["id"].clone();
                self.send(&json!({"jsonrpc":"2.0","id":response_id,"result":null}))?;
            }
        }
    }

    pub fn notify(&mut self, method: &str, params: Value) -> Result<(), String> {
        self.send(&json!({"jsonrpc":"2.0","method":method,"params":params}))
    }

    fn send(&mut self, value: &Value) -> Result<(), String> {
        let body = serde_json::to_vec(value).map_err(|error| error.to_string())?;
        write!(self.input, "Content-Length: {}\r\n\r\n", body.len())
            .and_then(|_| self.input.write_all(&body))
            .and_then(|_| self.input.flush())
            .map_err(|error| format!("could not write to language server: {error}"))
    }
}

impl Drop for LspClient {
    fn drop(&mut self) {
        let _ = self.notify("exit", Value::Null);
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn read_message<R: BufRead + Read>(reader: &mut R) -> Result<Value, String> {
    let mut content_length = None;
    loop {
        let mut header = String::new();
        let count = reader
            .read_line(&mut header)
            .map_err(|error| error.to_string())?;
        if count == 0 {
            return Err("language server closed its output".into());
        }
        if header == "\r\n" || header == "\n" {
            break;
        }
        if let Some((name, value)) = header.split_once(':') {
            if name.eq_ignore_ascii_case("content-length") {
                content_length = Some(
                    value
                        .trim()
                        .parse::<usize>()
                        .map_err(|_| "invalid Content-Length")?,
                );
            }
        }
    }
    let length = content_length.ok_or("language server response omitted Content-Length")?;
    let mut body = vec![0; length];
    reader
        .read_exact(&mut body)
        .map_err(|error| error.to_string())?;
    serde_json::from_slice(&body).map_err(|error| format!("invalid language server JSON: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_framed_message() {
        let body = r#"{"jsonrpc":"2.0","id":1,"result":null}"#;
        let framed = format!(
            "Content-Length: {}\r\nX-Trace: yes\r\n\r\n{}",
            body.len(),
            body
        );
        let parsed = read_message(&mut BufReader::new(framed.as_bytes())).unwrap();
        assert_eq!(parsed["id"], 1);
    }
}
