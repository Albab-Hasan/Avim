# Avim Usage Guide

## Running the Editor

```bash
cd Avim
cargo run --release              # Open empty buffer
cargo run --release -- test.txt  # Open a file
```

## Basic Commands

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
- `yy` - Yank (copy) current line
- `cc` - Change (delete and enter insert mode) current line
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

