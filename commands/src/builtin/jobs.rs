use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use nix::unistd::Pid;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JobStatus {
    Running,
    Stopped,
    Terminated,
}

impl fmt::Display for JobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JobStatus::Running => write!(f, "Running"),
            JobStatus::Stopped => write!(f, "Stopped"),
            JobStatus::Terminated => write!(f, "Terminated"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Job {
    pub id: usize,
    pub pgid: Pid,
    pub command: String,
    pub status: JobStatus,
}

pub struct JobManager {
    pub jobs: HashMap<usize, Job>,
    pub next_id: usize,
    pub current: Option<usize>,
    pub previous: Option<usize>,
}

impl JobManager {
    pub fn new() -> Self {
        Self {
            jobs: HashMap::new(),
            next_id: 1,
            current: None,
            previous: None,
        }
    }

    pub fn add_job(&mut self, pgid: Pid, command: String, status: JobStatus) -> usize {
        let id = self.next_id;
        self.jobs.insert(id, Job {
            id,
            pgid,
            command,
            status,
        });
        self.next_id += 1;
        
        self.previous = self.current;
        self.current = Some(id);
        
        id
    }

    pub fn remove_job(&mut self, id: usize) {
        self.jobs.remove(&id);
        if self.current == Some(id) {
            self.current = self.previous;
            self.previous = None; // Simplification
        } else if self.previous == Some(id) {
            self.previous = None;
        }
    }

    pub fn get_job_by_pid(&self, pid: Pid) -> Option<&Job> {
        self.jobs.values().find(|j| j.pgid == pid)
    }

    pub fn find_job_id_by_pid(&self, pid: Pid) -> Option<usize> {
        self.jobs.iter().find(|(_, j)| j.pgid == pid).map(|(id, _)| *id)
    }

    pub fn update_status(&mut self, id: usize, status: JobStatus) {
        if let Some(job) = self.jobs.get_mut(&id) {
            job.status = status;
        }
    }
}

pub static JOB_MANAGER: Lazy<Arc<Mutex<JobManager>>> = Lazy::new(|| {
    Arc::new(Mutex::new(JobManager::new()))
});
