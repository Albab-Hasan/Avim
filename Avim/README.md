# Avim

Terminal-based modal text editor written in Rust.

## Build Instructions

### Prerequisites
- Rust (latest stable version)
- Cargo (comes with Rust)

### Building
```bash
cd Avim
cargo build --release
```

The compiled binary will be located at `target/release/avim`.

### Running
```bash
# Run directly with cargo
cargo run --release

# Run with a file
cargo run --release -- path/to/file.txt

# Or use the compiled binary
./target/release/avim
./target/release/avim path/to/file.txt
```

### Installing
To install the binary to your system:
```bash
cargo install --path .
```
Then you can run `avim` from anywhere.

## Implemented Features

- Modal editing (Normal, Insert, Visual, Command modes)
- Basic cursor movement (h, j, k, l, w, b, 0, $, gg, G)
- File I/O (open, save, save as)
- Text editing (insert, delete, yank, paste)
- Line operations (dd, yy, cc, o, O)
- Motion-based operations (dw, d$, cw, c$, yw, y$)
- Visual selection (character and line-wise)
- Command mode (:w, :q, :wq, :e)
- Status line with mode indicator and file information
- Undo/redo (u, Ctrl+r)
- Search (/, ?, n, N) with forward and backward search
- Line numbers display
- Join lines (J)

## Usage Guide

### Normal Mode
- `h,j,k,l` - Move left, down, up, right
- `w` - Move forward by word
- `b` - Move backward by word
- `0` - Move to start of line
- `$` - Move to end of line
- `gg` - Go to first line
- `G` - Go to last line
- `x` - Delete character under cursor
- `dd` - Delete current line
- `dw` - Delete from cursor to end of word
- `d$` - Delete from cursor to end of line
- `yy` - Yank (copy) current line
- `yw` - Yank word
- `y$` - Yank to end of line
- `cc` - Change (delete and enter insert mode) current line
- `cw` - Change word
- `c$` - Change to end of line
- `p` - Paste below current line
- `P` - Paste above current line
- `J` - Join current line with next line
- `i` - Enter insert mode before cursor
- `a` - Enter insert mode after cursor
- `I` - Enter insert mode at line start
- `A` - Enter insert mode at line end
- `o` - Open new line below and enter insert mode
- `O` - Open new line above and enter insert mode
- `v` - Enter visual character mode
- `V` - Enter visual line mode
- `u` - Undo last change
- `Ctrl+r` - Redo last undone change
- `/` - Start forward search (type pattern and press Enter)
- `?` - Start backward search (type pattern and press Enter)
- `n` - Jump to next search match
- `N` - Jump to previous search match
- `:` - Enter command mode
- `Ctrl+C` - Quit (force quit)

### Insert Mode
- `Esc` - Return to normal mode
- `Backspace` - Delete character before cursor
- `Enter` - Insert newline
- Arrow keys - Move cursor

### Visual Mode
- `h,j,k,l` - Extend selection
- `d` or `x` - Delete selection
- `y` - Yank selection
- `Esc` - Return to normal mode

### Command Mode
- `:w` - Save file
- `:q` - Quit (fails if unsaved changes)
- `:q!` - Force quit (discard changes)
- `:wq` or `:x` - Save and quit
- `:w filename` - Save as filename
- `:e filename` - Edit a different file

## TODO

- Replace functionality
- Syntax highlighting
- Window splits and multiple buffers
- Configuration system
- Advanced text objects and motions
- Macros and registers
- Block visual mode
