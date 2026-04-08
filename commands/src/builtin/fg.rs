use crate::builtin::{Command, Cmd, JOB_MANAGER, JobStatus};
use nix::sys::signal::{self, Signal};
use nix::sys::wait::{waitpid, WaitStatus, WaitPidFlag};
use nix::unistd::{self, Pid};
use std::io::Write;

pub struct Fg;

impl Command for Fg {
    fn name(&self) -> &'static str {
        "fg"
    }

    fn run(&self, cmd: &mut Cmd) {
        let manager = JOB_MANAGER.lock().unwrap();
        let job_id = if let Some(arg) = cmd.args.get(0) {
            if arg.starts_with('%') {
                arg[1..].parse::<usize>().ok()
            } else {
                arg.parse::<usize>().ok()
            }
        } else {
            manager.jobs.keys().max().cloned()
        };

        if let Some(id) = job_id {
            if let Some(job) = manager.jobs.get(&id) {
                let pgid = job.pgid;
                let command = job.command.clone();
                
                writeln!(cmd.stdout, "{}", command).unwrap();
                
                // Continue if stopped
                if job.status == JobStatus::Stopped {
                    signal::kill(Pid::from_raw(-pgid.as_raw()), Signal::SIGCONT).ok();
                }
                
                // Put in foreground
                // Note: tcsetpgrp requires the terminal FD.
                let _ = unistd::tcsetpgrp(std::io::stdin(), pgid);
                
                // Release lock while waiting
                drop(manager);
                
                // Wait for job
                match waitpid(pgid, Some(WaitPidFlag::WUNTRACED)) {
                    Ok(WaitStatus::Stopped(_, _)) => {
                        let mut manager = JOB_MANAGER.lock().unwrap();
                        manager.update_status(id, JobStatus::Stopped);
                        writeln!(cmd.stdout, "\n[{}]+  Stopped                 {}", id, command).unwrap();
                    }
                    Ok(WaitStatus::Exited(_, _)) | Ok(WaitStatus::Signaled(_, _, _)) => {
                        let mut manager = JOB_MANAGER.lock().unwrap();
                        manager.remove_job(id);
                    }
                    _ => {}
                }
                
                // Back to shell
                let _ = unistd::tcsetpgrp(std::io::stdin(), unistd::getpgrp());
            } else {
                writeln!(cmd.stderr, "fg: job not found").unwrap();
            }
        } else {
            writeln!(cmd.stderr, "fg: no current job").unwrap();
        }
    }
}
