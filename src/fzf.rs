use std::io::Result;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct Fzf;

impl Fzf {
    pub fn run() {
        println!("fzf run")
    }

    pub fn run_hidden() {
        println!("fz run hidden");
    }
}
