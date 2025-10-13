use avim::editor::Editor;
use std::env;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).map(|s| s.as_str());
    
    let mut editor = Editor::new(file_path)?;
    editor.run()
}

