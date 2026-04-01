#!/bin/bash
set -e

if [ -z "$1" ]; then
    echo "Usage: ralph.sh <max_iterations>"
    exit 1
fi

max="$1"

for ((i = 1; i <= max; i++)); do
    echo "=== Iteration $i / $max ==="

    result=$(claude -p --dangerously-skip-permissions "@PRD.md @progress.txt Read PRD.md to find the next task. Implement it. Run tests. If tests fail, fix them. Append a summary to progress.txt. Make a git commit. Output exactly <promise>COMPLETE</promise> when ALL tasks in PRD.md are done.")

    echo "$result"

    if [[ "$result" == *"<promise>COMPLETE</promise>"* ]]; then
        echo "=== All tasks complete at iteration $i ==="
        exit 0
    fi
done

echo "=== Reached max iterations ($max) without completion ==="
exit 1