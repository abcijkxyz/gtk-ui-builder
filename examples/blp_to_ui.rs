#![allow(unused)]
#![allow(dead_code)]
#![allow(unused_imports)]

use gtk_ui_builder::prelude::*;

fn main() {
    // Read main.blp file
    let pattern = std::fs::read_to_string("assets/ui/main.blp")
        .expect("Failed to read pattern");

    // Parse AST
    let tree = Parser::parse(pattern)
        .expect("Failed to parse blueprint");

    // Output prettified AST
    println!("{}", tree.root.dbg());

    // Get XML representation of this AST
    let ui = tree.get_xml();

    // Write this representation to the file
    // now you can import it as any GTK UI file
    std::fs::write("assets/ui/main.ui", &ui);
}