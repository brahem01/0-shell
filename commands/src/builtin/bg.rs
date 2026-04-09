use crate::builtin::{Command, Cmd, JOB_MANAGER, JobStatus};
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use std::io::Write;

pub struct Bg;

impl Command for Bg {
    
}
