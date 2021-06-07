/**
 * MIT License
 *
 * tui-realm-treeview - Copyright (C) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
mod utils;
use utils::context::Context;
use utils::keymap::*;

use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;
use tuirealm::components::{input, label};
use tuirealm::props::borders::{BorderType, Borders};
use tuirealm::{Msg, Payload, PropsBuilder, Update, Value, View};
// tui
use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::tui::style::Color;
// treeview
use tui_realm_treeview::{Node, Tree, TreeView, TreeViewPropsBuilder};

const COMPONENT_INPUT: &str = "INPUT";
const COMPONENT_LABEL: &str = "LABEL";
const COMPONENT_TREEVIEW: &str = "TREEVIEW";

struct Model {
    path: PathBuf,
    tree: Tree,
    quit: bool,   // Becomes true when the user presses <ESC>
    redraw: bool, // Tells whether to refresh the UI; performance optimization
    view: View,
}

impl Model {
    fn new(view: View, p: &Path) -> Self {
        Model {
            quit: false,
            redraw: true,
            view,
            tree: Tree::new(Self::dir_tree(p, 3)),
            path: p.to_path_buf(),
        }
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn redraw(&mut self) {
        self.redraw = true;
    }

    fn reset(&mut self) {
        self.redraw = false;
    }

    pub fn scan_dir(&mut self, p: &Path) {
        self.path = p.to_path_buf();
        self.tree = Tree::new(Self::dir_tree(p, 3));
    }

    pub fn upper_dir(&self) -> Option<&Path> {
        self.path.parent()
    }

    fn dir_tree(p: &Path, depth: usize) -> Node {
        let name: String = match p.file_name() {
            None => "/".to_string(),
            Some(n) => n.to_string_lossy().into_owned().to_string(),
        };
        let mut node: Node = Node::new(p.to_string_lossy().into_owned(), name);
        if depth > 0 && p.is_dir() {
            if let Ok(e) = std::fs::read_dir(p) {
                e.flatten()
                    .for_each(|x| node.add_child(Self::dir_tree(x.path().as_path(), depth - 1)));
            }
        }
        node
    }
}

fn main() {
    // let's create a context: the context contains the backend of crossterm and the input handler
    let mut ctx: Context = Context::new();
    // Enter alternate screen
    ctx.enter_alternate_screen();
    // Clear screen
    ctx.clear_screen();
    // Make model
    let mut model: Model = Model::new(
        View::init(),
        std::env::current_dir().ok().unwrap().as_path(),
    );
    // Mount the component you need; we'll use a Label and an Input
    model.view.mount(
        COMPONENT_LABEL,
        Box::new(label::Label::new(
            label::LabelPropsBuilder::default()
                .with_foreground(Color::Cyan)
                .with_text(String::from(
                    "Selected node will appear here after a submit",
                ))
                .build(),
        )),
    );
    // Mount input
    model.view.mount(
        COMPONENT_INPUT,
        Box::new(input::Input::new(
            input::InputPropsBuilder::default()
                .with_borders(Borders::ALL, BorderType::Rounded, Color::LightBlue)
                .with_label(String::from("Go to..."))
                .with_foreground(Color::LightBlue)
                .build(),
        )),
    );
    let title: String = model.path.to_string_lossy().to_string();
    // Moount tree
    model.view.mount(
        COMPONENT_TREEVIEW,
        Box::new(TreeView::new(
            TreeViewPropsBuilder::default()
                .with_borders(Borders::ALL, BorderType::Rounded, Color::LightYellow)
                .with_foreground(Color::LightYellow)
                .with_background(Color::Black)
                .with_title(Some(title))
                .with_tree(model.tree.root())
                .with_highlighted_str("🚀")
                .keep_state(true)
                .build(),
        )),
    );
    // We need to give focus to input then
    model.view.active(COMPONENT_TREEVIEW);
    // let's loop until quit is true
    while !model.quit {
        // Listen for input events
        if let Ok(Some(ev)) = ctx.input_hnd.read_event() {
            // Pass event to view
            let msg = model.view.on(ev);
            model.redraw();
            // Call the elm friend update
            model.update(msg);
        }
        // If redraw, draw interface
        if model.redraw {
            // Call the elm friend vie1 function
            view(&mut ctx, &model.view);
            model.reset();
        }
        sleep(Duration::from_millis(10));
    }
    // Let's drop the context finally
    drop(ctx);
}

fn view(ctx: &mut Context, view: &View) {
    let _ = ctx.terminal.draw(|f| {
        // Prepare chunks
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Min(5),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(f.size());
        view.render(COMPONENT_LABEL, f, chunks[0]);
        view.render(COMPONENT_TREEVIEW, f, chunks[1]);
        view.render(COMPONENT_INPUT, f, chunks[2]);
    });
}

impl Update for Model {
    fn update(&mut self, msg: Option<(String, Msg)>) -> Option<(String, Msg)> {
        let ref_msg: Option<(&str, &Msg)> = msg.as_ref().map(|(s, msg)| (s.as_str(), msg));
        match ref_msg {
            None => None, // Exit after None
            Some(msg) => match msg {
                (COMPONENT_TREEVIEW, Msg::OnChange(Payload::One(Value::Str(node_id)))) => {
                    // Update span
                    let props = label::LabelPropsBuilder::from(
                        self.view.get_props(COMPONENT_LABEL).unwrap(),
                    )
                    .with_text(format!("Selected: '{}'", node_id))
                    .build();
                    // Report submit
                    let msg = self.view.update(COMPONENT_LABEL, props);
                    self.update(msg)
                }
                (COMPONENT_TREEVIEW, Msg::OnSubmit(Payload::One(Value::Str(node_id)))) => {
                    // Update tree
                    self.scan_dir(PathBuf::from(node_id.as_str()).as_path());
                    // Update
                    let props = TreeViewPropsBuilder::from(
                        self.view.get_props(COMPONENT_TREEVIEW).unwrap(),
                    )
                    .with_tree(self.tree.root())
                    .with_title(Some(String::from(self.path.to_string_lossy())))
                    .build();
                    let msg = self.view.update(COMPONENT_TREEVIEW, props);
                    self.update(msg)
                }
                (COMPONENT_TREEVIEW, &MSG_KEY_BACKSPACE) => {
                    // Update tree
                    match self.upper_dir() {
                        None => None,
                        Some(p) => {
                            let p: PathBuf = p.to_path_buf();
                            self.scan_dir(p.as_path());
                            // Update
                            let props = TreeViewPropsBuilder::from(
                                self.view.get_props(COMPONENT_TREEVIEW).unwrap(),
                            )
                            .with_tree(self.tree.root())
                            .with_title(Some(String::from(self.path.to_string_lossy())))
                            .build();
                            let msg = self.view.update(COMPONENT_TREEVIEW, props);
                            self.update(msg)
                        }
                    }
                }
                (COMPONENT_INPUT, Msg::OnSubmit(Payload::One(Value::Str(input)))) => {
                    let p: PathBuf = PathBuf::from(input.as_str());
                    self.scan_dir(p.as_path());
                    // Update
                    let props = TreeViewPropsBuilder::from(
                        self.view.get_props(COMPONENT_TREEVIEW).unwrap(),
                    )
                    .with_tree(self.tree.root())
                    .with_title(Some(String::from(self.path.to_string_lossy())))
                    .build();
                    let msg = self.view.update(COMPONENT_TREEVIEW, props);
                    self.update(msg)
                }
                (COMPONENT_INPUT, &MSG_KEY_TAB) => {
                    self.view.active(COMPONENT_TREEVIEW);
                    None
                }
                (COMPONENT_TREEVIEW, &MSG_KEY_TAB) => {
                    self.view.active(COMPONENT_INPUT);
                    None
                }
                (_, &MSG_KEY_ESC) => {
                    // Quit on esc
                    self.quit();
                    None
                }
                _ => None,
            },
        }
    }
}
