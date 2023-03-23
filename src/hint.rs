use crate::salesforce::Connection;

use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::{Hint, Hinter};
use rustyline::{Context, Helper, Result, Validator};
use serde::Deserialize;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fs;
use termion::{color, style};

#[derive(Deserialize, Debug)]
struct JsonData {
    value: String,
}

#[derive(Helper, Validator)]
pub struct QueryHinter<'a> {
    pub connection: &'a Connection,
    pub hints: RefCell<HashSet<QueryHint>>,
}

impl<'a> QueryHinter<'a> {
    pub fn new(connection: &'a Connection) -> Self {
        let objects = connection.get_cached_objects();
        let hints = HashSet::from_iter(objects.into_iter().map(|s| QueryHint::new(&s)));
        QueryHinter {
            connection,
            hints: hints.into(),
        }
    }

    fn update_hints(&self, line: &str) {
        let dot_boundary = line.rfind('.').unwrap_or(0);
        let object_name = line.trim();
        let objects = self.connection.get_cached_objects();
        let is_matching_object =
            object_name.is_empty() || objects.contains(&object_name.to_string());

        let mut hints = self.hints.borrow_mut();
        if is_matching_object {
            *hints = HashSet::from_iter(objects.into_iter().map(|s| QueryHint::new(&s)));
        } else if dot_boundary > 0 {
            *hints = method_hints().unwrap();
        }
    }
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

impl Hinter for QueryHinter<'_> {
    type Hint = QueryHint;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<QueryHint> {
        if line.is_empty() || pos < line.len() {
            return None;
        }

        self.update_hints(line);

        let last_word_boundary = line
            .rfind(|c: char| c.is_whitespace() || c == '.' || c == '(' || c == ',')
            .map(|idx| idx + 1)
            .unwrap_or(0);
        let line_suffix = &line[last_word_boundary..];

        let hints = self.hints.borrow();

        hints
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

impl Highlighter for QueryHinter<'_> {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        let styled_hint = format!(
            "{}{}{}{}",
            style::Faint,
            color::Fg(color::LightWhite),
            hint,
            style::Reset,
        );
        Cow::Owned(styled_hint)
    }
}

impl<'a> Completer for QueryHinter<'a> {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        _pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> Result<(usize, Vec<Pair>)> {
        self.update_hints(line);

        let last_word_boundary = line
            .rfind(|c: char| c.is_whitespace() || c == '.' || c == '(' || c == ',')
            .map(|idx| idx + 1)
            .unwrap_or(0);
        let line_suffix = &line[last_word_boundary..];

        let hints = self.hints.borrow();
        let candidates: Vec<Pair> = hints
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

pub fn method_hints() -> std::result::Result<HashSet<QueryHint>, Box<dyn std::error::Error>> {
    let mut set = HashSet::new();
    set.insert(QueryHint::new("select("));
    set.insert(QueryHint::new("where("));
    set.insert(QueryHint::new("limit("));
    set.insert(QueryHint::new("orderby("));
    set.insert(QueryHint::new("open("));

    Ok(set)
}
