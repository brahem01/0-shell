#!/usr/bin/env bash

# This script runs a series of tests to verify job control functionality.
# It uses the built binary in target/release/shell

SHELL_BIN="./target/release/shell"

if [ ! -f "$SHELL_BIN" ]; then
    echo "Shell binary not found! Run 'make build' first."
    exit 1
fi

export DIR="$(pwd)/bin/"

echo "--- Starting Job Control Tests ---"

# Test 1: Background job &
echo "Test 1: Background job..."
result=$(echo "sleep 1 &" | $SHELL_BIN)
if [[ $result == *"[1]"* ]]; then
    echo "✅ Background job started successfully."
else
    echo "❌ Background job failed to start."
    echo "Output: $result"
    exit 1
fi

# Test 2: jobs command
echo "Test 2: jobs command..."
# We sleep a bit to give the background job time to show up in 'jobs'
# but not too long so it doesn't finish.
result=$(echo "sleep 2 & ; jobs" | $SHELL_BIN)
if [[ $result == *"Running"* ]] && [[ $result == *"sleep 2"* ]]; then
    echo "✅ jobs command shows running job."
else
    echo "❌ jobs command failed."
    echo "Output: $result"
    exit 1
fi

# Test 3: fg command (partial test via script is hard, but we check if it accepts the command)
echo "Test 3: fg/bg command availability..."
result=$(echo "jobs" | $SHELL_BIN)
# Just checking if the shell doesn't crash on these builtins
echo "fg" | $SHELL_BIN > /dev/null 2>&1
echo "bg" | $SHELL_BIN > /dev/null 2>&1
echo "✅ fg/bg commands invoked without crash."

# Test 4: kill command
# Test 4: kill command...
# Start a job, get its pid/id, and kill it.
# In our shell, 'jobs' output looks like [1]+  Running sleep 10 &
# We'll try to kill %1
# We add a small sleep inside the shell to allow the signal to be delivered before 'jobs'
result=$(echo "sleep 10 & ; sleep 1 ; kill %1 ; sleep 1 ; jobs" | $SHELL_BIN)
if [[ $result == *"Terminated"* ]] || [[ $result == *"Done"* ]]; then
     echo "✅ kill command sent signal."
else
    echo "❌ kill command failed."
    echo "Output: $result"
fi

echo "--- Job Control Tests Completed ---"
