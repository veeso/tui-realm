//! # Mock
//!
//! This module provides mock data

use crate::{Node, Tree};

pub fn mock_tree() -> Tree<String> {
    Tree::new(
        Node::new("/".to_string(), "/".to_string())
            .with_child(
                Node::new(String::from("a"), String::from("a"))
                    .with_child(
                        Node::new(String::from("aA"), String::from("aA"))
                            .with_child(Node::new(String::from("aA0"), String::from("aA0")))
                            .with_child(Node::new(String::from("aA1"), String::from("aA1")))
                            .with_child(Node::new(String::from("aA2"), String::from("aA2"))),
                    )
                    .with_child(
                        Node::new(String::from("aB"), String::from("aB"))
                            .with_child(Node::new(String::from("aB0"), String::from("aB0")))
                            .with_child(Node::new(String::from("aB1"), String::from("aB1")))
                            .with_child(Node::new(String::from("aB2"), String::from("aB2"))),
                    )
                    .with_child(
                        Node::new(String::from("aC"), String::from("aC"))
                            .with_child(Node::new(String::from("aC0"), String::from("aC0"))),
                    ),
            )
            .with_child(
                Node::new(String::from("b"), String::from("b"))
                    .with_child(
                        Node::new(String::from("bA"), String::from("bA"))
                            .with_child(
                                Node::new(String::from("bA0"), String::from("bA0")).with_child(
                                    Node::new(String::from("bA0!"), String::from("bA0!")),
                                ),
                            )
                            .with_child(Node::new(String::from("bA1"), String::from("bA1")))
                            .with_child(Node::new(String::from("bA2"), String::from("bA2"))),
                    )
                    .with_child(
                        Node::new(String::from("bB"), String::from("bB"))
                            .with_child(Node::new(String::from("bB0"), String::from("bB0")))
                            .with_child(Node::new(String::from("bB1"), String::from("bB1")))
                            .with_child(Node::new(String::from("bB2"), String::from("bB2")))
                            .with_child(Node::new(String::from("bB3"), String::from("bB3")))
                            .with_child(Node::new(String::from("bB4"), String::from("bB4")))
                            .with_child(Node::new(String::from("bB5"), String::from("bB5"))),
                    ),
            )
            .with_child(
                Node::new(String::from("c"), String::from("c")).with_child(
                    Node::new(String::from("cA"), String::from("cA"))
                        .with_child(Node::new(String::from("cA0"), String::from("cA0")))
                        .with_child(Node::new(String::from("cA1"), String::from("cA1")))
                        .with_child(Node::new(String::from("cA2"), String::from("cA2"))),
                ),
            ),
    )
}
