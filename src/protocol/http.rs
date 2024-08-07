use core::str;
use std::fmt::Debug;

pub fn parse_http_request_message<'a>(data: &'a [u8]) -> Option<HTTPMessage<'a>> {
    // https://httpwg.org/specs/rfc9112.html#message.format
    // Message Format
    //   HTTP-message = start-line CRLF
    // *( field-line CRLF )
    // CRLF
    // [ message-body ]

    let mut cursor = Cursor::new(data);

    let line = cursor.find_next_crlf()?;
    let start_line = StartLine::new(line);

    println!("{start_line:#?}");

    let mut field_lines = Vec::new();

    while let Some(line) = cursor.find_next_crlf() {
        if line.is_empty() {
            break;
        }

        let Some(delimiter) = line.iter().position(|b| *b == b':') else {
            continue;
        };

        field_lines.push((
            bytes_as_str(&line[..delimiter]).trim(),
            bytes_as_str(&line[delimiter + 1..]).trim(),
        ))
    }

    let body = cursor.remain();

    Some(HTTPMessage {
        start_line,
        header: Header { field_lines },
        body,
    })
}

#[derive(Debug)]
pub struct StartLine<'a> {
    method: &'a str,
    request_target: &'a str,
    http_version: &'a str,
}

fn bytes_as_str<'a>(data: &'a [u8]) -> &'a str {
    str::from_utf8(data).unwrap_or_default()
}

impl<'a> StartLine<'a> {
    fn new(line: &'a [u8]) -> Self {
        let mut chunks = line.split(|b| b == &b' ');
        let mut consume = || chunks.next().map(bytes_as_str).unwrap_or_default();

        Self {
            method: consume(),
            request_target: consume(),
            http_version: consume(),
        }
    }
}

#[derive(Debug)]
pub struct Header<'a> {
    field_lines: Vec<(&'a str, &'a str)>,
}

impl<'a> Header<'a> {
    pub fn get(&self, name: &str) -> Option<&(&str, &str)> {
        for line in &self.field_lines {
            if line.0 == name {
                return Some(line);
            }
        }

        return None;
    }
}

pub struct HTTPMessage<'a> {
    start_line: StartLine<'a>,
    header: Header<'a>,
    body: &'a [u8],
}

impl<'a> Debug for HTTPMessage<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body = str::from_utf8(&self.body).unwrap_or("<Not UTF8>");

        f.debug_struct("HTTPMessage")
            .field("start_line", &self.start_line)
            .field("header", &self.header)
            .field("body", &body)
            .finish()
    }
}

struct Cursor<'a> {
    inner: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(inner: &'a [u8]) -> Self {
        Self { inner, pos: 0 }
    }

    fn find_next_crlf(&mut self) -> Option<&'a [u8]> {
        let data = &self.inner[self.pos..];

        let mut i = 0;

        while i < data.len() {
            if &data[i..i + 2] == b"\r\n" {
                self.pos += i + 2;
                return Some(&data[..i]);
            }

            i += 1;
        }

        None
    }

    fn remain(self) -> &'a [u8] {
        &self.inner[self.pos..]
    }
}
