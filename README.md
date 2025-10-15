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

## Quick Example

```bash
# Create and edit a new Python file
avim hello.py

# Type the following in insert mode (press 'i'):
def greet(name):
    print(f"Hello, {name}!")

# Features you'll see:
# - Syntax highlighting with colors
# - Auto-closing brackets when you type (
# - Smart indentation when you press Enter
# - Tab inserts 4 spaces
# - Smart bracket deletion with Backspace
```

## Window Splits Example

```bash
# Start with a file
avim main.rs

# Split horizontally to work on multiple files
:split helper.rs

# Or use Ctrl+w s for horizontal split
# Ctrl+w v for vertical split

# Navigate between windows
Ctrl+w w    # Cycle through windows
Ctrl+w h    # Move to left window
Ctrl+w j    # Move to window below
Ctrl+w k    # Move to window above  
Ctrl+w l    # Move to right window

# Close current window
:close
# Or Ctrl+w c

# Each window maintains its own:
# - Cursor position
# - Viewport (scroll position)
# - Buffer (file content)
# - Syntax highlighting
```

## Implemented Features

### Core Editor Features
- Modal editing (Normal, Insert, Visual, Command modes)
- Basic cursor movement (h, j, k, l, w, b, 0, $, gg, G)
- File I/O (open, save, save as, create new files)
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

### Advanced Features
- **Window Splits**: Full Vim-like window management system
  - Horizontal splits (`:split` or `Ctrl+w s`)
  - Vertical splits (`:vsplit` or `Ctrl+w v`)
  - Window navigation (`Ctrl+w h/j/k/l` or `Ctrl+w w/W`)
  - Close windows (`:close` or `Ctrl+w c/q`)
  - Each window maintains its own buffer, cursor, and viewport
  - Buffer sharing between windows (same file, different positions)
- **Syntax Highlighting**: Full syntax highlighting using syntect with true color (24-bit RGB) support
  - Supports Rust, Python, JavaScript, C/C++, and many other languages
  - Automatic language detection from file extensions
  - Professional color themes with graceful fallback
- **Smart Indentation**: 
  - Tab key inserts 4 spaces
  - Auto-indentation preservation on new lines
  - Extra indentation after opening brackets/braces
- **Auto-Closing Brackets**: 
  - Automatically closes `()`, `[]`, `{}`, `""`, `''`, `` ` ` ``
  - Smart bracket deletion (removes both when deleting opening bracket if empty)
  - Cursor positioning between auto-closed brackets
- **Performance Optimized**:
  - Batched screen rendering for smooth performance
  - Efficient syntax highlighting integration
  - Real-time text editing with instant visual feedback

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
- `Backspace` - Delete character before cursor (smart bracket deletion)
- `Enter` - Insert newline with auto-indentation
- `Tab` - Insert 4 spaces for indentation
- Arrow keys - Move cursor
- **Auto-closing brackets**: Type `(`, `[`, `{`, `"`, `'`, or `` ` `` to auto-close
- **Smart bracket deletion**: Delete opening bracket to remove both if empty
- **Auto-indentation**: Extra indentation after `{`, `(`, `[` when pressing Enter

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
- `:split` or `:sp` - Split window horizontally
- `:vsplit` or `:vs` - Split window vertically
- `:split filename` - Split and open file horizontally
- `:vsplit filename` - Split and open file vertically
- `:close` or `:clo` - Close current window
- `:only` or `:on` - Close all windows except current

### Window Management (Ctrl+w Commands)
- `Ctrl+w s` - Split window horizontally
- `Ctrl+w v` - Split window vertically
- `Ctrl+w h/j/k/l` - Navigate to window in direction
- `Ctrl+w w` - Cycle to next window
- `Ctrl+w W` - Cycle to previous window
- `Ctrl+w c` or `Ctrl+w q` - Close current window
- `Ctrl+w o` - Close other windows (keep only current)
- `Ctrl+w +` - Increase window height
- `Ctrl+w -` - Decrease window height
- `Ctrl+w >` - Increase window width
- `Ctrl+w <` - Decrease window width
- `Ctrl+w =` - Equal size all windows

## Technical Details

### Dependencies
- `crossterm` - Cross-platform terminal manipulation
- `syntect` - Syntax highlighting engine
- `ropey` - Efficient text rope data structure
- `unicode-segmentation` - Unicode text processing

### Performance Features
- Optimized rendering with batched screen updates
- Efficient syntax highlighting with syntect
- Smart bracket color normalization
- Real-time text editing with minimal latency

### Terminal Support
- True color (24-bit RGB) support for modern terminals
- Graceful fallback for older terminals
- Cross-platform compatibility (Windows, macOS, Linux)
- Works with Windows Terminal, iTerm2, and modern Linux terminals

## TODO

- Multi-window rendering with borders and separators
- Window resizing commands (Ctrl+w +/-/</>)
- Configuration system with themes and keybindings
- Advanced text objects and motions
- Macros and registers
- Block visual mode
- Replace functionality
- Plugin system
- LSP integration
