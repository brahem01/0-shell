use std::env;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use nix::unistd::{self, Pid};
use nix::sys::wait::{waitpid, WaitStatus, WaitPidFlag};
use std::os::unix::process::CommandExt;

pub use commands::{Cmd, Registry, JOB_MANAGER, JobStatus};

pub fn exec(commands: Vec<Cmd>) {
    let mut prev_stdout = None;
    let mut children: Vec<Child> = Vec::new();
    let mut cmd_iter = commands.into_iter().peekable();
    let mut is_background = false;
    let mut full_command = String::new();

    while let Some(cmd) = cmd_iter.next() {
        is_background = cmd.is_background;
        if !full_command.is_empty() {
            full_command.push_str(" | ");
        }
        full_command.push_str(&cmd.cmd);
        for arg in &cmd.args {
            full_command.push(' ');
            full_command.push_str(arg);
        }
        if is_background && cmd_iter.peek().is_none() {
            full_command.push_str(" &");
        }

        let registry = Registry::new();
        if registry.has(&cmd) {
            if cmd_iter.peek().is_some() || prev_stdout.is_some() {
                eprintln!("Built-in commands cannot be used in pipelines");
                continue;
            }

            registry.run(cmd);
            continue;
        }
        let executable = match find_executable(&cmd.cmd) {
            Some(path) => path,
            None => {
                eprintln!("command not found: {}", cmd.cmd);
                continue;
            }
        };

        let stdin = match prev_stdout.take() {
            Some(output) => Stdio::from(output),
            None => Stdio::inherit(),
        };

        let stdout = if cmd_iter.peek().is_some() {
            Stdio::piped()
        } else {
            Stdio::inherit()
        };

        let mut command = Command::new(&executable);
        command.args(&cmd.args)
               .stdin(stdin)
               .stdout(stdout);

        // Put the command in its own process group
        unsafe {
            command.pre_exec(|| {
                unistd::setpgid(Pid::from_raw(0), Pid::from_raw(0)).ok();
                Ok(())
            });
        }

        let mut child = match command.spawn() {
            Ok(child) => child,
            Err(err) => {
                eprintln!("failed to execute '{}': {}", cmd.cmd, err);
                continue;
            }
        };

        prev_stdout = child.stdout.take();
        children.push(child);
    }

    if children.is_empty() {
        return;
    }

    let pgid = Pid::from_raw(children[0].id() as i32);

    if is_background {
        let mut manager = JOB_MANAGER.lock().unwrap();
        let id = manager.add_job(pgid, full_command, JobStatus::Running);
        println!("[{}] {}", id, pgid);
    } else {
        // Put in foreground
        let _ = unistd::tcsetpgrp(std::io::stdin(), pgid);

        for child in children {
            let pid = Pid::from_raw(child.id() as i32);
            match waitpid(pid, Some(WaitPidFlag::WUNTRACED)) {
                Ok(WaitStatus::Stopped(_, _)) => {
                    let mut manager = JOB_MANAGER.lock().unwrap();
                    manager.add_job(pgid, full_command.clone(), JobStatus::Stopped);
                    let id = manager.find_job_id_by_pid(pgid).unwrap_or(0);
                    println!("\n[{}]+  Stopped                 {}", id, full_command);
                    break;
                }
                Ok(WaitStatus::Exited(_, _)) | Ok(WaitStatus::Signaled(_, _, _)) => {
                    // Normal termination
                }
                _ => {}
            }
        }

        // Back to shell
        let _ = unistd::tcsetpgrp(std::io::stdin(), unistd::getpgrp());
    }
}


pub fn check_background_jobs() {
    let mut manager = JOB_MANAGER.lock().unwrap();
    let job_ids: Vec<usize> = manager.jobs.keys().cloned().collect();

    for id in job_ids {
        let job = &manager.jobs[&id];
        let pgid = job.pgid;
        
        match waitpid(pgid, Some(WaitPidFlag::WNOHANG | WaitPidFlag::WUNTRACED | WaitPidFlag::WCONTINUED)) {
            Ok(WaitStatus::Exited(_, status)) => {
                let job_command = job.command.clone();
                manager.remove_job(id);
                if status == 0 {
                    println!("[{}]  Done                    {}", id, job_command);
                } else {
                    println!("[{}]  Done({})                {}", id, status, job_command);
                }
            }
            Ok(WaitStatus::Signaled(_, sig, _)) => {
                let job_command = job.command.clone();
                manager.remove_job(id);
                println!("[{}]  Terminated: {:?}         {}", id, sig, job_command);
            }
            Ok(WaitStatus::Stopped(_, _)) => {
                manager.update_status(id, JobStatus::Stopped);
            }
            Ok(WaitStatus::Continued(_)) => {
                manager.update_status(id, JobStatus::Running);
            }
            _ => {}
        }
    }
}

fn find_executable(cmd: &str) -> Option<PathBuf> {
    let dir = env::var("DIR").ok()?;
    let candidate = Path::new(&dir).join(cmd);
    if candidate.is_file() {
        Some(candidate)
    } else {
        None
    }
}
