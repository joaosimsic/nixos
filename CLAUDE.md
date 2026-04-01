You are running an iterative implementation loop for this repository.

Context files:
- `prd.json` (single source of truth for task status)
- `progress.txt` (append-only iteration memory)

On this iteration:
1. Read `prd.json` and identify the single highest-priority unfinished story (`passes: false`).
2. Implement only that story in the codebase.
3. Run relevant checks/tests for the change.
4. If checks fail, fix and re-run until green or clearly blocked.
5. If implementation is complete and checks pass, update that story in `prd.json` to `passes: true`.
6. Append a concise update to `progress.txt` with:
   - what was implemented
   - what was validated
   - any blockers/next step

Do not perform git operations.
Read-only git context is allowed:
- `git status`
- `git diff`
- `git log --oneline -n 20`

Do not run any git command that mutates repository state or history, including:
- `git add`, `git commit`, `git merge`, `git rebase`, `git reset`
- `git switch`, `git checkout`, `git stash`, `git push`, `git pull`
- `git tag`, `git cherry-pick`, or any equivalent write operation

When all stories in `prd.json` have `passes: true`, output exactly:
`<promise>COMPLETE</promise>`
