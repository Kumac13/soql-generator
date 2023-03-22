use std::borrow::Cow;
use std::collections::HashSet;
use std::fs;

use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::{Hint, Hinter};
use rustyline::history::DefaultHistory;
use rustyline::{Context, Editor, Helper, Result, Validator};
use serde::Deserialize;
use termion::{color, style};

#[derive(Deserialize, Debug)]
struct JsonData {
    value: String,
}

#[derive(Helper, Validator)]
pub struct QueryHinter {
    pub hints: HashSet<QueryHint>,
}

#[derive(Hash, Debug, PartialEq, Eq)]
pub struct QueryHint {
    display: String,
    complete_up_to: usize,
}

impl Hint for QueryHint {
    fn display(&self) -> &str {
        &self.display
    }

    fn completion(&self) -> Option<&str> {
        if self.complete_up_to > 0 {
            Some(&self.display[..self.complete_up_to])
        } else {
            None
        }
    }
}

impl QueryHint {
    fn new(value: &str) -> QueryHint {
        QueryHint {
            display: value.into(),
            complete_up_to: value.len(),
        }
    }

    fn suffix(&self, strip_chars: usize) -> QueryHint {
        let start_idx = strip_chars.min(self.display.len());
        QueryHint {
            display: self.display[start_idx..].to_owned(),
            complete_up_to: self.complete_up_to.saturating_sub(strip_chars),
        }
    }
}

impl Hinter for QueryHinter {
    type Hint = QueryHint;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<QueryHint> {
        if line.is_empty() || pos < line.len() {
            return None;
        }

        let last_word_boundary = line
            .rfind(|c: char| c.is_whitespace() || c == '.' || c == '(' || c == ',')
            .map(|idx| idx + 1)
            .unwrap_or(0);
        let line_suffix = &line[last_word_boundary..];

        self.hints
            .iter()
            .filter_map(|hint| {
                if hint.display.starts_with(line_suffix) {
                    Some(hint.suffix(line_suffix.len()))
                } else {
                    None
                }
            })
            .next()
    }
}

impl Highlighter for QueryHinter {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        let styled_hint = format!(
            "{}{}{}{}",
            style::Faint,
            color::Fg(color::LightWhite),
            hint,
            style::Reset
        );
        Cow::Owned(styled_hint)
    }
}

impl Completer for QueryHinter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        _pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> Result<(usize, Vec<Pair>)> {
        let last_word_boundary = line
            .rfind(|c: char| c.is_whitespace() || c == '.' || c == '(' || c == ',')
            .map(|idx| idx + 1)
            .unwrap_or(0);
        let line_suffix = &line[last_word_boundary..];

        let candidates: Vec<Pair> = self
            .hints
            .iter()
            .filter(|hint| hint.display.starts_with(line_suffix))
            .map(|hint| Pair {
                display: hint.display.clone(),
                replacement: hint.display[..hint.complete_up_to].to_string(),
            })
            .collect();

        Ok((last_word_boundary, candidates))
    }
}

pub fn query_hints() -> std::result::Result<HashSet<QueryHint>, Box<dyn std::error::Error>> {
    let json_data: Vec<JsonData> = serde_json::from_str(&fs::read_to_string("data.json")?)?;
    let mut set = HashSet::new();

    for data in json_data {
        set.insert(QueryHint::new(&data.value));
    }

    Ok(set)
}
