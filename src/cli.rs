use std::collections::HashSet;
use rustyline::{Config, Context, Editor};
use rustyline::error::ReadlineError;
use rustyline::hint::{Hint, Hinter};
use rustyline::{Completer, Helper, Validator, Highlighter};
use rustyline::sqlite_history::SQLiteHistory;

pub type Result<T> = rustyline::Result<T>;

#[derive(Completer, Helper, Validator, Highlighter)]
struct JSnapHinter {
    hints: HashSet<CommandHint>,
}

#[derive(Hash, Debug, PartialEq, Eq)]
struct CommandHint {
    display: String,
    complete_up_to: usize,
}

impl Hint for CommandHint {
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

impl CommandHint {
    fn new(text: &str, complete_up_to: &str) -> CommandHint {
        assert!(text.starts_with(complete_up_to));
        CommandHint {
            display: text.into(),
            complete_up_to: complete_up_to.len(),
        }
    }

    fn suffix(&self, strip_chars: usize) -> CommandHint {
        CommandHint {
            display: self.display[strip_chars..].to_owned(),
            complete_up_to: self.complete_up_to.saturating_sub(strip_chars),
        }
    }
}

impl Hinter for JSnapHinter {
    type Hint = CommandHint;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<CommandHint> {
        if line.is_empty() || pos < line.len() {
            return None;
        }

        self.hints
            .iter()
            .filter_map(|hint| {
                // expect hint after word complete, like redis cli, add condition:
                // line.ends_with(" ")
                if hint.display.starts_with(line) {
                    Some(hint.suffix(pos))
                } else {
                    None
                }
            })
            .next()
    }
}

pub struct JSnapCli {
    rl: Editor<JSnapHinter, SQLiteHistory>,
}

impl JSnapCli {
    pub fn new() -> Result<JSnapCli> {
        // 提示
        let mut hints = HashSet::new();
        hints.insert(CommandHint::new("help", "help"));
        hints.insert(CommandHint::new("exit", "exit"));
        let hinter = JSnapHinter {
            hints
        };
        // 历史
        let config = Config::builder()
            .auto_add_history(true)
            .build();
        let history = SQLiteHistory::with_config(config)?;

        let mut rl: Editor<JSnapHinter, SQLiteHistory> = Editor::with_history(config, history)?;
        rl.set_helper(Some(hinter));
        Ok(JSnapCli { rl })
    }

    pub fn readline(&mut self, prompt: &str) -> Result<String> {
        let prompt = format!("{}>", prompt);
        self.rl.readline(prompt.as_str())
    }

}