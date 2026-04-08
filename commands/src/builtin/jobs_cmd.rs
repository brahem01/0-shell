use crate::builtin::{Command, Cmd, JOB_MANAGER, JobStatus};
use std::io::Write;

pub struct Jobs;

impl Command for Jobs {
    fn name(&self) -> &'static str {
        "jobs"
    }

    fn run(&self, cmd: &mut Cmd) {
        let manager = JOB_MANAGER.lock().unwrap();
        let mut job_ids: Vec<usize> = manager.jobs.keys().cloned().collect();
        job_ids.sort();

        let show_pids = cmd.args.iter().any(|arg| arg == "-p");
        let show_long = cmd.args.iter().any(|arg| arg == "-l");
        let only_running = cmd.args.iter().any(|arg| arg == "-r");
        let only_stopped = cmd.args.iter().any(|arg| arg == "-s");

        for id in job_ids {
            let job = &manager.jobs[&id];
            
            if only_running && job.status != JobStatus::Running {
                continue;
            }
            if only_stopped && job.status != JobStatus::Stopped {
                continue;
            }

            let marker = if manager.current == Some(id) {
                "+"
            } else if manager.previous == Some(id) {
                "-"
            } else {
                " "
            };

            if show_pids {
                writeln!(cmd.stdout, "{}", job.pgid).unwrap();
            } else if show_long {
                writeln!(cmd.stdout, "[{}]{} {} {:<23} {} &", job.id, marker, job.pgid, format!("{}", job.status), job.command).unwrap();
            } else {
                writeln!(cmd.stdout, "[{}]{}  {:<23} {} &", job.id, marker, format!("{}", job.status), job.command).unwrap();
            }
        }
    }
}
