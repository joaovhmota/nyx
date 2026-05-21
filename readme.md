# Git Hooks

For a better developing experience, run this command in the terminal to use the repository's git hooks.

```bash
git config core.hooksPath hooks
```

# Docker Compose

Run the script below to run:

```bash
docker compose up --build --detach
```

> [!TIP]
> To hide the build logs, use the `--quiet-build` option:
> ```bash
> docker compose up --build --detach --quiet-build
> ```