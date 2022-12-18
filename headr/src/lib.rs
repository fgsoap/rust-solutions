use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::{Arg, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}
