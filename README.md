# Smart Session Manager (SSM) for Tmux

Smart Session Manager (SSM) is a tool designed to manage Tmux sessions efficiently, focusing on project-based session management. With SSM, you can easily navigate through sessions using simple commands and aliases.

## Features

- **Session Management**: Create and manage Tmux sessions based on project directories or aliases.
- **Alias Management**: Set, list, and remove aliases for quick navigation.
- **Directory Navigation**: Easily find and navigate to directories.
- **Session Storage**: Sessions are stored in files, with session names based on the directory path or alias. _**Note:** Session storage and restoration has not been implemented yet._

## Usage

```
ssm [PATH_OR_ALIAS] or ssm [OPTIONS]
```

### Arguments

- `[PATH_OR_ALIAS]`: Path of the directory or the alias you have saved.

### Options

- `-l, --list-alias`: List all the aliases.
- `-a, --set-alias <SET_ALIAS>`: Set alias to the active directory.
- `--remove-alias <REMOVE_ALIAS>`: Remove alias. Usage: `ssm --remove-alias <ALIAS>`.
- `--clear-alias`: Clear all the aliases.
- `-s, --save`: Save the current session (Note: Save and restore functionality is not implemented yet).
- `-n, --new-session`: Start a new session, don't restore.
- `-f, --find`: Find (default is alias).
- `-d, --dir`: Opt to path. Usage: `ssm -f -d` or `ssm --find --dir`.
- `-H, --show-hidden`: Show hidden directories (use only with `--dir`). Usage: `ssm -f -d -H` or `ssm --find --dir --show-hidden`.
- `-A, --all`: Opt to both aliases and directories. Usage: `ssm -f -A` or `ssm --find --all`.
- `-h, --help`: Print help.
- `-V, --version`: Print version.

| Option           | Shorthand | Description                                                                                                 |
| ---------------- | --------- | ----------------------------------------------------------------------------------------------------------- |
| `--list-alias`   | `-l`      | List all the aliases                                                                                        |
| `--set-alias`    | `-a`      | Set alias to the active directory. Usage: ssm --set-alias <ALIAS>                                           |
| `--remove-alias` |           | Remove alias. Usage: ssm --remove-alias <ALIAS>                                                             |
| `--clear-alias`  |           | Clear all the aliases.                                                                                      |
| `--save`         | `-s`      | Save the current session (Note: Save and restore functionality is not implemented yet).                     |
| `--new-session`  | `-n`      | Start a new session, don't restore.                                                                         |
| `--find`         | `-f`      | Find (default is alias).                                                                                    |
| `--dir`          | `-d`      | Opt to path. Usage: `ssm -f -d` or `ssm --find --dir`.                                                      |
| `--show-hidden`  | `-H`      | Show hidden directories (use only with `--dir`). Usage: `ssm -f -d -H` or `ssm --find --dir --show-hidden`. |
| `--all`          | `-A`      | Opt to both aliases and directories. Usage: `ssm -f -A` or `ssm --find --all`.                              |
| `--help`         | `-h`      | Print help.                                                                                                 |
| `--version`      | `-V`      | Print version.                                                                                              |

### Prerequisites

**Compulsory:**

- **Tmux**: The terminal multiplexer required for managing sessions.

**Optional:**

- [`fzf`](https://github.com/junegunn/fzf): The fuzzy finder. Necessary for the find-related options to work.
- [`fd`](https://github.com/sharkdp/fd): A faster alternative to the find command. For an improved experience, especially when finding directories, it's recommended to install fd.

## Installation

Clone the repository and navigate to the project directory:

```bash
git clone <repository_url>
cd smart-session-manager
```

Ensure you have Tmux installed on your system. Then, add the `ssm` script to your PATH or create an alias for it in your shell configuration file (for example, `.bashrc`, `.zshrc`).

## Planned Features

- **Session Restoration**: Save and restore previous sessions (not yet implemented).

## Contribution

Contributions are welcome! Please fork the repository and create a pull request with your changes.

---

### Documentation

#### Listing Aliases

To list all the aliases:

```bash
ssm --list-alias
```

#### Setting an Alias

To set an alias for the active directory:

```bash
ssm --set-alias <ALIAS_NAME>
```

#### Removing an Alias

To remove an alias:

```bash
ssm --remove-alias <ALIAS_NAME>
```

#### Clearing All Aliases

To clear all aliases:

```bash
ssm --clear-alias
```

#### Saving a Session

To save the current session:

```bash
ssm --save
```

_Note: Save and restore functionality is not implemented yet._

#### Starting a New Session

To start a new session without restoring:

```bash
ssm --new-session
```

#### Finding Directories or Aliases

To find an alias:

```bash
ssm --find
```

To find a directory:

```bash
ssm --find --dir
```

To show hidden directories:

```bash
ssm --find --dir --show-hidden
```

To find both aliases and directories:

```bash
ssm --find --all
```

#### Getting Help

To print help:

```bash
ssm --help
```

#### Getting Version

To print version:

```bash
ssm --version
```
