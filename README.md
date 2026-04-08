# 🐚 Custom Shell in Rust (Job Control)

A minimalist Unix-like shell written in **Rust**, supporting a set of builtins, common commands, and **Job Control**.  
⚠️ Unlike traditional shells, this shell **does not run any system binaries** — it only executes the ones **built and provided inside this project**.

---

## 🚀 Features

### Builtin Commands
- `cd` – Change directory
- `pwd` – Print working directory
- `exit` – Exit the shell
- `clear` – Clear the terminal screen

### Job Control Builtins
- `jobs` – List active jobs.
  - `-l` → Long format (includes PGID).
  - `-p` → List only PGIDs.
  - `-r` → Only running jobs.
  - `-s` → Only stopped jobs.
- `bg [%job_id]` – Resume a stopped job in the background.
- `fg [%job_id]` – Bring a background/stopped job to the foreground.
- `kill <pid> | %job_id` – Send SIGTERM to a process or job.

### Job Control Features
- **Background execution**: Append `&` to a command to run it in the background.
- **Stop jobs**: Press `Ctrl + Z` to stop (suspend) the current foreground job.
- **Job Status Notification**: The shell notifies you when background jobs complete or change status.

### External Commands
- `echo`, `ls`, `cat`, `cp`, `rm`, `mv`, `mkdir`.

### Shell Capabilities
- **Pipelines**: `cmd1 | cmd2`
- **Command chaining**: `cmd1 && cmd2`
- **Multiple commands**: `cmd1 ; cmd2`

---

## 📂 Project Structure

```
├── bin/                # Installed command binaries (after build)
├── commands/           # Implementation of external commands and builtins
│   ├── src/bin/        # Each subcommand (cat, ls, etc.)
│   ├── builtin/        # Builtin commands (cd, jobs, fg, bg, etc.)
│   └── src/lib.rs      # Registry for commands
├── executer/           # Executes parsed commands and manages job control
├── tokenizer/          # Tokenizer & parser for command-line input (supports |, ;, &)
├── shell/              # Main shell entry point with signal handling
├── Makefile            # Build, install, test, and run automation
├── tests/              # Integration tests
└── README.md           # You are here!
```

---

## ⚙️ Build & Run

### 1. Build & Install Everything
```sh
make
```
This cleans, builds, and copies binaries to the `bin/` directory.

### 2. Run the Shell
```sh
make run
```

### 3. Run Tests
```sh
make test
```

---

## 🖥️ Usage Examples

### Running in Background
```sh
$ sleep 5 &
[1] 12345
$ jobs
[1]+  Running                 sleep 5 &
```

### Stopping and Resuming Jobs
```sh
$ sleep 100
^Z
[1]+  Stopped                 sleep 100
$ bg %1
[1] sleep 100 &
$ jobs
[1]+  Running                 sleep 100 &
$ fg %1
sleep 100
```

### Pipelines with Background
```sh
$ ls -l | cat &
[2] 12346
```

---

## ✅ Implementation Details

- **Process Groups**: Each job (single command or pipeline) is put into its own process group (`setpgid`).
- **Terminal Control**: The shell uses `tcsetpgrp` to switch control of the terminal between itself and foreground jobs.
- **Signal Handling**: The shell ignores `SIGINT` and `SIGTSTP` while a foreground job is running, allowing the job to receive them.
- **Asynchronous Monitoring**: The shell checks for status changes in background jobs using `waitpid` with `WNOHANG` before each prompt.

---

## 📜 License

This project is licensed under the MIT License.  
Feel free to use, modify, and share it!
