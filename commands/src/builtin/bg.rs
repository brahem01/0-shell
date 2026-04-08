use crate::builtin::{Command, Cmd, JOB_MANAGER, JobStatus};
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use std::io::Write;

pub struct Bg;

impl Command for Bg {
    fn name(&self) -> &'static str {
        "bg"
    }

    fn run(&self, cmd: &mut Cmd) {
        let mut manager = JOB_MANAGER.lock().unwrap();
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
            if let Some(job) = manager.jobs.get_mut(&id) {
                if job.status == JobStatus::Stopped {
                    signal::kill(Pid::from_raw(-job.pgid.as_raw()), Signal::SIGCONT).ok();
                    job.status = JobStatus::Running;
                    writeln!(cmd.stdout, "[{}] {} &", id, job.command).unwrap();
                } else {
                    writeln!(cmd.stderr, "bg: job {} already in background", id).unwrap();
                }
            } else {
                writeln!(cmd.stderr, "bg: job not found").unwrap();
            }
        } else {
            writeln!(cmd.stderr, "bg: no current job").unwrap();
        }
    }
}
