#!/bin/bash
# Run the emulator for a short time and capture output

cargo build --release 2>&1 | tail -5
echo "Running emulator for 2 seconds..."
( cargo run --release 2>&1 & PID=$! ; sleep 2 ; kill $PID 2>/dev/null ) | head -100
echo "Done."
