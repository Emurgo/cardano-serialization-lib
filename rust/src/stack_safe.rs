use std::fmt;
use std::mem;

/// Drain a recursive owned tree onto the heap so `Drop` of an empty node cannot
/// overflow the native stack.
pub fn iterative_drop<T>(root: &mut T, mut take_children: impl FnMut(&mut T, &mut Vec<T>)) {
    let mut stack = Vec::new();
    take_children(root, &mut stack);
    while let Some(mut node) = stack.pop() {
        take_children(&mut node, &mut stack);
    }
}

/// CBOR definite/indefinite remaining-element counter.
#[derive(Clone, Copy, Debug)]
pub enum Remaining {
    Definite(usize),
    Indefinite,
}

impl Remaining {
    pub fn from_cbor_len(len: cbor_event::Len) -> Self {
        match len {
            cbor_event::Len::Len(n) => Remaining::Definite(n as usize),
            cbor_event::Len::Indefinite => Remaining::Indefinite,
        }
    }

    #[allow(dead_code)]
    pub fn is_done(&self) -> bool {
        matches!(self, Remaining::Definite(0))
    }

    #[allow(dead_code)]
    pub fn is_indefinite(&self) -> bool {
        matches!(self, Remaining::Indefinite)
    }

    pub fn decrement(&mut self) {
        if let Remaining::Definite(n) = self {
            *n = n.saturating_sub(1);
        }
    }
}

/// Heap JSON DOM with iterative Drop, used on the output side only: `to_json` of
/// chain data can be arbitrarily deep, so it is built and printed without recursion.
/// JSON input goes through serde_json, whose recursion limit bounds the depth instead.
pub enum JsonVal {
    Number(String),
    String(String),
    Array(Vec<JsonVal>),
    Object(Vec<(String, JsonVal)>),
}

impl fmt::Debug for JsonVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonVal::Number(n) => write!(f, "Number({n})"),
            JsonVal::String(s) => write!(f, "String({s:?})"),
            JsonVal::Array(items) => write!(f, "Array(len={})", items.len()),
            JsonVal::Object(items) => write!(f, "Object(len={})", items.len()),
        }
    }
}

impl Drop for JsonVal {
    fn drop(&mut self) {
        iterative_drop(self, |node, stack| match node {
            JsonVal::Array(items) => stack.extend(mem::take(items)),
            JsonVal::Object(items) => {
                stack.extend(mem::take(items).into_iter().map(|(_, v)| v));
            }
            _ => {}
        });
    }
}

impl JsonVal {
    /// Build an object with the key semantics of `serde_json::Map` (no `preserve_order`),
    /// which is what every JSON entry point had through 16.0.0: entries sorted by key and,
    /// for a repeated key, the last value wins.
    pub fn object_sorted(mut entries: Vec<(String, JsonVal)>) -> JsonVal {
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        entries.dedup_by(|later, earlier| {
            if later.0 == earlier.0 {
                std::mem::swap(&mut later.1, &mut earlier.1);
                true
            } else {
                false
            }
        });
        JsonVal::Object(entries)
    }
}

/// Convert `JsonVal` into `serde_json::Value` without recursing on the native stack.
pub fn json_val_to_serde(value: JsonVal) -> serde_json::Value {
    enum Frame {
        Array {
            rest: std::vec::IntoIter<JsonVal>,
            acc: Vec<serde_json::Value>,
        },
        Object {
            rest: std::vec::IntoIter<(String, JsonVal)>,
            acc: Vec<(String, serde_json::Value)>,
            pending_key: Option<String>,
        },
    }

    fn number_to_serde(n: String) -> serde_json::Value {
        if let Ok(u) = n.parse::<u64>() {
            serde_json::Value::Number(u.into())
        } else if let Ok(i) = n.parse::<i64>() {
            serde_json::Value::Number(i.into())
        } else {
            #[cfg(feature = "arbitrary-precision-json")]
            {
                serde_json::Value::Number(serde_json::Number::from_string_unchecked(n))
            }
            #[cfg(not(feature = "arbitrary-precision-json"))]
            {
                serde_json::Value::String(n)
            }
        }
    }

    enum Taken {
        Number(String),
        String(String),
        Array(Vec<JsonVal>),
        Object(Vec<(String, JsonVal)>),
    }

    fn take_json(value: &mut JsonVal) -> Taken {
        match value {
            JsonVal::Number(n) => Taken::Number(mem::take(n)),
            JsonVal::String(s) => Taken::String(mem::take(s)),
            JsonVal::Array(items) => Taken::Array(mem::take(items)),
            JsonVal::Object(items) => Taken::Object(mem::take(items)),
        }
    }

    let mut stack: Vec<Frame> = Vec::new();
    let mut current = value;
    loop {
        let mut completed = match take_json(&mut current) {
            Taken::Number(n) => number_to_serde(n),
            Taken::String(s) => serde_json::Value::String(s),
            Taken::Array(items) => {
                let mut rest = items.into_iter();
                if let Some(first) = rest.next() {
                    stack.push(Frame::Array {
                        rest,
                        acc: Vec::new(),
                    });
                    current = first;
                    continue;
                } else {
                    serde_json::Value::Array(Vec::new())
                }
            }
            Taken::Object(items) => {
                let mut rest = items.into_iter();
                if let Some((k, v)) = rest.next() {
                    stack.push(Frame::Object {
                        rest,
                        acc: Vec::new(),
                        pending_key: Some(k),
                    });
                    current = v;
                    continue;
                } else {
                    serde_json::Value::Object(serde_json::Map::new())
                }
            }
        };

        loop {
            match stack.last_mut() {
                None => return completed,
                Some(Frame::Array { rest, acc }) => {
                    acc.push(std::mem::replace(&mut completed, serde_json::Value::Null));
                    if let Some(next) = rest.next() {
                        current = next;
                        break;
                    } else if let Frame::Array { acc, .. } = stack.pop().unwrap() {
                        completed = serde_json::Value::Array(acc);
                    }
                }
                Some(Frame::Object {
                    rest,
                    acc,
                    pending_key,
                }) => {
                    let key = pending_key.take().unwrap();
                    acc.push((key, std::mem::replace(&mut completed, serde_json::Value::Null)));
                    if let Some((k, v)) = rest.next() {
                        *pending_key = Some(k);
                        current = v;
                        break;
                    } else if let Frame::Object { acc, .. } = stack.pop().unwrap() {
                        completed = serde_json::Value::Object(acc.into_iter().collect());
                    }
                }
            }
        }
    }
}

pub fn stringify_json(value: &JsonVal, pretty: bool) -> String {
    let mut out = String::new();
    let mut stack: Vec<StrFrame<'_>> = Vec::new();
    let mut current = value;
    let mut indent = 0usize;
    loop {
        match current {
            JsonVal::Number(n) => out.push_str(n),
            JsonVal::String(s) => write_json_string(&mut out, s),
            JsonVal::Array(items) => {
                out.push('[');
                if items.is_empty() {
                    out.push(']');
                } else {
                    indent += 1;
                    if pretty {
                        out.push('\n');
                        push_indent(&mut out, indent);
                    }
                    let mut rest = items.iter();
                    current = rest.next().unwrap();
                    stack.push(StrFrame::Array {
                        rest,
                        indent,
                        pretty,
                    });
                    continue;
                }
            }
            JsonVal::Object(items) => {
                out.push('{');
                if items.is_empty() {
                    out.push('}');
                } else {
                    indent += 1;
                    if pretty {
                        out.push('\n');
                        push_indent(&mut out, indent);
                    }
                    let mut rest = items.iter();
                    let (k, v) = rest.next().unwrap();
                    write_json_string(&mut out, k);
                    if pretty {
                        out.push_str(": ");
                    } else {
                        out.push(':');
                    }
                    current = v;
                    stack.push(StrFrame::Object {
                        rest,
                        indent,
                        pretty,
                    });
                    continue;
                }
            }
        }

        loop {
            match stack.last_mut() {
                None => return out,
                Some(StrFrame::Array {
                    rest,
                    indent: frame_indent,
                    pretty,
                }) => {
                    if let Some(next) = rest.next() {
                        out.push(',');
                        if *pretty {
                            out.push('\n');
                            push_indent(&mut out, *frame_indent);
                        }
                        current = next;
                        indent = *frame_indent;
                        break;
                    } else {
                        let pretty = *pretty;
                        let frame_indent = *frame_indent;
                        stack.pop();
                        indent = frame_indent.saturating_sub(1);
                        if pretty {
                            out.push('\n');
                            push_indent(&mut out, indent);
                        }
                        out.push(']');
                    }
                }
                Some(StrFrame::Object {
                    rest,
                    indent: frame_indent,
                    pretty,
                }) => {
                    if let Some((k, v)) = rest.next() {
                        out.push(',');
                        if *pretty {
                            out.push('\n');
                            push_indent(&mut out, *frame_indent);
                        }
                        write_json_string(&mut out, k);
                        if *pretty {
                            out.push_str(": ");
                        } else {
                            out.push(':');
                        }
                        current = v;
                        indent = *frame_indent;
                        break;
                    } else {
                        let pretty = *pretty;
                        let frame_indent = *frame_indent;
                        stack.pop();
                        indent = frame_indent.saturating_sub(1);
                        if pretty {
                            out.push('\n');
                            push_indent(&mut out, indent);
                        }
                        out.push('}');
                    }
                }
            }
        }
    }
}

enum StrFrame<'a> {
    Array {
        rest: std::slice::Iter<'a, JsonVal>,
        indent: usize,
        pretty: bool,
    },
    Object {
        rest: std::slice::Iter<'a, (String, JsonVal)>,
        indent: usize,
        pretty: bool,
    },
}

fn push_indent(out: &mut String, indent: usize) {
    for _ in 0..indent {
        out.push_str("  ");
    }
}

fn write_json_string(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0C}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out.push('"');
}
