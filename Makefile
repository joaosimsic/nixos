sandbox:
	docker build -t claude-sandbox .
	docker run -it -v $(PWD):/workspace -v $(HOME)/.claude:/home/claude/.claude claude-sandbox /bin/bash -c "bash ./scripts/ralph/ralph.sh 5"
