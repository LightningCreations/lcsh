use std::{collections::HashMap, process::Child};

pub enum Redirect<'a> {
    Tie(i32),
    Read(&'a str),
    Write(&'a str),
    Append(&'a str),
    Pattern(&'a str),
    String(&'a str),
}

pub enum Commands<'a> {
    Command {
        env: HashMap<&'a str, &'a str>,
        cmd: &'a str,
        args: Vec<&'a str>,
        redirects: Vec<(i32, Redirect<'a>)>,
    },
    Background(Box<Commands<'a>>),
    Pipe(Box<Commands<'a>>, Box<Commands<'a>>),
    Disjunction(Box<Commands<'a>>, Box<Commands<'a>>),
    Conjunction(Box<Commands<'a>>, Box<Commands<'a>>),
    Subshell(Box<Commands<'a>>),
}

pub enum CommandEvalResult {
    SpawnedChild(std::process::Child),
}

impl<'a> Commands<'a> {
    pub fn should_block(&self) -> bool {
        !matches!(self, Commands::Background(_))
    }
    pub fn execute(&self) -> std::io::Result<Option<CommandEvalResult>> {
        Ok(None)
    }
}
