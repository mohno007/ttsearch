# ttsearch: Typetalk full-text search CLI tool

ttsearch is a command-line tool for importing and searching through large volumes of data with high speed and flexibility.

Features
--------

-   **Versatile Import Options** : Bulk import CSV and ZIP files from a specified directory, or import directly from a single ZIP or CSV file.

-   **Flexible Search Syntax:**
    -   Search with single or multiple keywords using `AND`/`OR` operators.
    -   Perform field-specific searches (e.g., `topic_id:1234`).
    -   Conduct range searches for date/time fields \
        (e.g., `created_at:[2024-06-05T00:00:00+0900 TO 2025-06-05T20:01:00+0900]`).

-   **High-Speed Performance**: Experience fast search times, averaging 0.03 seconds for 200K records.

-   **Multiple Output Formats**:
    -   `pretty`: Human-readable, multi-line output (default).
    -   `oneline`: Displays messages in a single line with newlines replaced by spaces.
    -   `json`: Outputs search results in JSON format.

Installation
------------

### 1\. Build the Project

First, build the project using Cargo.

```
$ cargo build --release
```

### 2\. Add to Your PATH

To use the `ttsearch` command from any directory, add the compiled binary to your system's `PATH`.

#### For the current session:

```
$ export PATH="$PATH:$PWD/target/release/"
```

#### To make it permanent:

Add the following line to your shell configuration file (e.g., `.bashrc`, `.zshrc`).

```
$ echo 'export PATH="$PATH:'$PWD'/target/release/"' >> "$HOME/.bashrc"
```

Remember to restart your shell or source the configuration file for the changes to take effect.

Usage
-----

### Importing Data

First, you need to import messages.

Use the `import` (or `i`) subcommand to load your data. You can specify a directory or a ZIP / CSV file.

```
$ ttsearch import "path/to/your/file_or_directory"

# Example
$ ttsearch i "./1234-topic/"
```

### Searching

Use the `search` (or `s`) subcommand to find what you're looking for.

```
$ ttsearch search "your_keyword"

# Example with multiple keywords and a field search
$ ttsearch s "Rust topic_id:1234"
```

### Tips: Alias

For quicker access, you can set up a shell alias.

```
$ alias tts="ttsearch s"
```

Now you can simply run:

```
$ tts "Rust topic_id:1234"
```

Managing the Index
------------------

### Deleting the Index

Currently, the search index must be deleted manually. To do so, remove the following directory:

```
$ rm -rf ~/.local/state/ttsearch/messages/
```
