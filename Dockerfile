FROM node:20-slim

RUN apt-get update && apt-get install -y \
    git \
    bash \
    curl \
    jq \
    && rm -rf /var/lib/apt/lists/*

RUN npm install -g @anthropic-ai/claude-code

RUN useradd -m -u 1001 -s /bin/bash claude

WORKDIR /workspace

USER claude

CMD ["/bin/bash"]