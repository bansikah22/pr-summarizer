# PR Summarizer with Jokes

![GitHub Test Actions Status](https://github.com/bansikah22/fibbot/actions/workflows/test.yml/badge.svg)
![GitHub Test Actions Status](https://github.com/bansikah22/fibbot/actions/workflows/release.yml/badge.svg)
![GitHub Issues](https://img.shields.io/github/issues/bansikah22/fibbot?color=red)
![Open Source](https://img.shields.io/github/license/bansikah22/fibbot?color=green)

A GitHub Action that automatically summarizes pull requests and adds a random programming joke to make code reviews more enjoyable.

## Features

- 📝 **PR Change Analysis**: Detects modified, added, and deleted files
- 📊 **Summary Generation**: Converts PR changes into short, meaningful descriptions
- 💬 **GitHub Commenting**: Posts the summary as a comment on the PR
- 😄 **Random Jokes**: Fetches and appends a programming joke to the comment
- 🔄 **External Repository Support**: Can be used across multiple GitHub repositories

## Example Output

```
## PR Summary:
- Implemented signup functionality using JWT authentication
- Refactored user service to support email validation
- Updated README.md with API documentation

## Affected files:
- [M] src/auth.rs
- [M] src/user_service.rs
- [M] README.md

### Here's something to lighten the mood:
Why do programmers prefer dark mode? Because light attracts bugs! 😆
```

## Usage

Add this to your repository's `.github/workflows/pr-summary.yml`:

```yaml
name: PR Summarizer

on:
  pull_request:
    types: [opened, synchronize, reopened]

permissions:
  pull-requests: write  # Required to comment on PRs
  issues: write        # Required for PR comments via issues API

jobs:
  summarize:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v3
        with:
          fetch-depth: 0  # Ensures full history for diff analysis

      - name: Run PR Summarizer
        uses: bansikah22/pr-summarizer@master  # Uses the local action in your repository
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
```

## How It Works

1. When a pull request is opened or updated, the action is triggered
2. It analyzes the changes made in the PR
3. It generates a concise summary of the changes
4. It fetches a programming joke from several APIs
5. It posts a comment on the PR with the summary and joke

## Development

### Prerequisites

- Rust 1.56+
- Cargo

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

## License

[MIT](./LICENSE) License