# App Be Gone

A simple Rust based application to remove applications and their associated files from your system.

## Usage

Dry run

```bash
cargo run Slack --dry-run
Would run: rm -rf /Applications/Slack.app
```

Run

Use caution when using this command. It will remove all files and folders that match the application name you provide.

```bash
cargo run Slack
Running: rm -rf /Applications/Slack.app
```

## Why?

This is a simple way to find all files and applications that are installed on your system and remove them in one go.

Use caution when using this application. It will remove all files and folders that match the application name you provide.

It also uses sudo to remove the files. This is because some applications are installed by the system and require root access to remove them.

## License

MIT - details soon. For now, use at your own risk.

## Contributing

Please feel free to contribute to this project. I'm happy to accept pull requests and issues.

## Maintainers

[@erictherobot - Eric David Smith](https://ericdavidsmith.com)
