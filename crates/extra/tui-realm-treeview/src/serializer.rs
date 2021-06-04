//! # serializer
//!
//! Serializer for tree and prop payload
//!

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
use super::{Node, PropPayload, PropValue, Tree, TuiTree, TuiTreeItem};
use std::{collections::LinkedList, slice::IterMut};

impl Node {
    // -- conversion :: prop payload -> tree

    /// ### from_prop_payload
    ///
    /// Fill a tree using prop payload recursively
    pub(crate) fn from_prop_payload(
        &mut self,
        mut list: LinkedList<PropPayload>,
    ) -> Option<Box<PropPayload>> {
        match list.pop_front() {
            Some(PropPayload::Tup3((
                PropValue::Str(id),
                PropValue::Str(label),
                PropValue::Str(parent),
            ))) => {
                // Get parent
                let parent: &mut Node = self
                    .query_mut(parent.as_str())
                    .expect("Parent node doesn't exist");
                // Push node to parent
                parent.children.push(Node::new(id, label));
                // Set next
                self.from_prop_payload(list)
            }
            None => None,
            _ => panic!("Invalid payload"),
        }
    }

    // -- tree -> prop payload

    /// ### to_prop_payload
    ///
    /// Convert a Node into PropPayload.
    /// This is achieved using the `Vec` variant.
    /// So basically is a flat tree
    pub(crate) fn to_prop_payload(&self, depth: usize, parent: &str) -> PropPayload {
        PropPayload::Linked(Self::to_payload_list(self, depth, parent))
    }

    /// ### to_payload_list
    ///
    /// Iterates over tree, to fill the linked list
    fn to_payload_list(&self, depth: usize, parent: &str) -> LinkedList<PropPayload> {
        let this: PropPayload = Self::to_prop_value(self, parent);
        let mut items: LinkedList<PropPayload> = LinkedList::new();
        // Push this
        items.push_back(this);
        // Push children
        if depth > 0 {
            self.children
                .iter()
                .for_each(|x| items.extend(Self::to_payload_list(x, depth - 1, self.id.as_str())));
        }
        items
    }

    /// ### to_prop_value
    ///
    /// Convert node to a prop value representing the node
    fn to_prop_value(&self, parent: &str) -> PropPayload {
        PropPayload::Tup3((
            PropValue::Str(self.id.to_string()),
            PropValue::Str(self.label.to_string()),
            PropValue::Str(parent.to_string()),
        ))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_serializer_to_and_from_prop_value() {
        let tree: Tree = Tree::new(
            Node::new("/", "/")
                .add_child(
                    Node::new("/bin", "bin/")
                        .add_child(Node::new("/bin/ls", "ls"))
                        .add_child(Node::new("/bin/pwd", "pwd")),
                )
                .add_child(
                    Node::new("/home", "home/").add_child(
                        Node::new("/home/omar", "omar/")
                            .add_child(Node::new("/home/omar/readme.md", "readme.md"))
                            .add_child(Node::new("/home/omar/changelog.md", "changelog.md")),
                    ),
                ),
        );
        let props: PropPayload = tree.root().to_prop_payload(usize::MAX, "");
        let tree: Tree = Tree::from(props);
        let root: &Node = tree.root();
        // count
        assert_eq!(root.count(), 8);
        // verify members
        assert_eq!(root.id.as_str(), "/");
        assert_eq!(root.label.as_str(), "/");
        assert_eq!(root.children.len(), 2);
        let bin: &Node = &root.children[0];
        assert_eq!(bin.id.as_str(), "/bin");
        assert_eq!(bin.label.as_str(), "bin/");
        assert_eq!(bin.children.len(), 2);
        let bin_ids: Vec<&str> = bin.children.iter().map(|x| x.id.as_str()).collect();
        assert_eq!(bin_ids, vec!["/bin/ls", "/bin/pwd"]);
        let home: &Node = &tree.root.children[1];
        assert_eq!(home.id.as_str(), "/home");
        assert_eq!(home.label.as_str(), "home/");
        assert_eq!(home.children.len(), 1);
        let omar_home: &Node = &home.children[0];
        let omar_home_ids: Vec<&str> = omar_home.children.iter().map(|x| x.id.as_str()).collect();
        assert_eq!(
            omar_home_ids,
            vec!["/home/omar/readme.md", "/home/omar/changelog.md"]
        );
        // -- Query
        assert_eq!(
            tree.query("/home/omar/changelog.md").unwrap().id.as_str(),
            "/home/omar/changelog.md"
        );
    }

    #[test]
    #[should_panic]
    fn test_serializer_bad_props_payload() {
        Tree::from(PropPayload::None);
    }

    #[test]
    #[should_panic]
    fn test_serializer_bad_props_value() {
        let mut linked_list: LinkedList<PropPayload> = LinkedList::new();
        linked_list.push_back(PropPayload::One(PropValue::Str("pippo".to_string())));
        Tree::from(PropPayload::Linked(linked_list));
    }
}
