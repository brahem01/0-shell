use crate::builtin::{Command, Cmd, JOB_MANAGER};
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use std::io::Write;

pub struct Kill;

impl Command for Kill {
    fn name(&self) -> &'static str {
        "kill"
    }

    fn run(&self, cmd: &mut Cmd) {
        if cmd.args.is_empty() {
            writeln!(cmd.stderr, "kill: usage: kill <pid> | %<job_id>").unwrap();
            return;
        }

        let arg = &cmd.args[0];
        let target_pid = if arg.starts_with('%') {
            let id = arg[1..].parse::<usize>().ok();
            let manager = JOB_MANAGER.lock().unwrap();
            id.and_then(|i| manager.jobs.get(&i).map(|j| j.pgid))
        } else {
            arg.parse::<i32>().ok().map(Pid::from_raw)
        };

        if let Some(pid) = target_pid {
            let target = if arg.starts_with('%') {
                Pid::from_raw(-pid.as_raw())
            } else {
                pid
            };
            match signal::kill(target, Signal::SIGTERM) {
                Ok(_) => {
                    // We don't remove the job here, as we wait for it to actually terminate
                    // in the wait loop (or background check).
                }
                Err(e) => {
                    writeln!(cmd.stderr, "kill: {}: {}", pid, e).unwrap();
                }
            }
        } else {
            writeln!(cmd.stderr, "kill: {}: invalid job or pid", arg).unwrap();
        }
    }
}
