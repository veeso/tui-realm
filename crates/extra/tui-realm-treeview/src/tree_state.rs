//! # Tree State
//!
//! This module implements the tree state.

use super::Node;

/// ## TreeState
///
/// Tree state tracks the current state for the component tree.
#[derive(Default, Clone)]
pub struct TreeState {
    /// Tracks open nodes
    open: Vec<String>,
    /// Current selected item
    selected: Option<String>,
}

impl TreeState {
    // -- getters

    /// ### is_open
    ///
    /// Returns whether `node` is open
    pub fn is_open<V>(&self, node: &Node<V>) -> bool {
        self.open.contains(node.id())
    }

    /// ### is_closed
    ///
    /// Returns whether `node` is closed
    pub fn is_closed<V>(&self, node: &Node<V>) -> bool {
        !self.is_open(node)
    }

    /// ### selected
    ///
    /// Get current selected item
    pub fn selected(&self) -> Option<&str> {
        self.selected.as_deref()
    }

    /// ### is_selected
    ///
    /// Returns whether provided node is currently selected
    pub fn is_selected<V>(&self, node: &Node<V>) -> bool {
        self.selected
            .as_ref()
            .map(|x| x == node.id())
            .unwrap_or(false)
    }

    /// ### first_sibling
    ///
    /// Get first sibling in children of current selected node's parent
    pub fn first_sibling<'a, V>(&self, tree: &'a Node<V>) -> Option<&'a Node<V>> {
        let selected = self.selected.as_ref()?;
        let parent = tree.parent(selected)?;
        parent.iter().next()
    }

    /// ### last_sibling
    ///
    /// Get last sibling in children of current selected node's parent
    pub fn last_sibling<'a, V>(&self, tree: &'a Node<V>) -> Option<&'a Node<V>> {
        let selected = self.selected.as_ref()?;
        let parent = tree.parent(selected)?;
        parent.iter().last()
    }

    // -- modifiers

    /// ### tree_changed
    ///
    /// The tree has changed, so this method must check whether to keep states or not
    pub fn tree_changed<V>(&mut self, root: &Node<V>, preserve: bool) {
        if preserve {
            // Check whether selected is still valid; if doesn't exist, use root
            self.selected = self
                .selected
                .take()
                .map(|selected| root.query(&selected).unwrap_or(root).id().to_string());
            // Check whether open nodes still exist
            self.open.retain(|x| root.query(x).is_some());
        } else {
            // Reset state
            self.open = Vec::new();
            self.selected = Some(root.id().to_string());
        }
    }

    /// ### open
    ///
    /// Open currently selected `node`. Node can be open only if it is closed and it is NOT a leaf
    pub fn open<V>(&mut self, root: &Node<V>) {
        if let Some(selected) = self.selected.as_ref() {
            if let Some(node) = root.query(selected) {
                self.open_node(root, node);
            }
        }
    }

    /// ### close
    ///
    /// Close currently selected `node`.
    /// If node has children, then all children are closed recursively
    pub fn close<V>(&mut self, root: &Node<V>) {
        if let Some(selected) = self.selected.as_ref() {
            if let Some(node) = root.query(selected) {
                if self.is_open(node) {
                    self.close_node(node);
                }
            }
        }
    }

    /// ### move_down
    ///
    /// Move cursor down in current tree from current position. Rewind if required
    pub fn move_down<V>(&mut self, root: &Node<V>) {
        if let Some(selected) = self.selected.take() {
            // Get current node
            if let Some(node) = root.query(&selected) {
                // If node is open, then move to its first child
                if !node.is_leaf() && self.is_open(node) {
                    // NOTE: unwrap is safe; checked by `is_leaf()`
                    self.selected = Some(node.iter().next().unwrap().id().to_string());
                } else {
                    // If has a "next sibling", let's get it
                    if let Some(sibling) = self.next_sibling(root, node) {
                        self.selected = Some(sibling.id().to_string());
                    } else {
                        // Then the next element becomes the next sibling of the parent
                        // this thing has to be performed recursively for all parents, until one is found (or root is reached)
                        let mut current = &selected;
                        loop {
                            if let Some(parent) = root.parent(current) {
                                current = parent.id();
                                if let Some(sibling) = self.next_sibling(root, parent) {
                                    self.selected = Some(sibling.id().to_string());
                                    break;
                                }
                            } else {
                                // has no parent, keep selectd
                                self.selected = Some(selected);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    /// ### move_up
    ///
    /// Move cursor up in current tree from current position. Rewind if required
    pub fn move_up<V>(&mut self, root: &Node<V>) {
        if let Some(selected) = self.selected.take() {
            // Get parent
            if let Some(parent) = root.parent(&selected) {
                // Selected becomes previous sibling's last child; or if None, the parent
                self.selected = Some(
                    self.previous_sibling(root, root.query(&selected).unwrap())
                        .map(|x| self.get_last_open_heir(x))
                        .unwrap_or(parent)
                        .id()
                        .to_string(),
                );
            } else {
                // Is root; then keep selected
                self.selected = Some(selected);
            }
        }
    }

    /// ### select
    ///
    /// Set current selected node.
    /// When selecting a node, all its ancestors will be opened
    pub fn select<V>(&mut self, root: &Node<V>, node: &Node<V>) {
        self.open_ancestors(root, node);
        self.selected = Some(node.id().to_string());
    }

    // -- private

    /// ### close_node
    ///
    /// Close `node`
    fn close_node<V>(&mut self, node: &Node<V>) {
        // Remove from open nodes
        self.open.retain(|x| x != node.id());
        // Close children for node
        self.close_children(node);
    }

    /// ### open_node
    ///
    /// Open provided node
    /// Node is opened only if is NOT a leaf and it is closed.
    /// It will also open all the ancestors for `node`
    fn open_node<V>(&mut self, root: &Node<V>, node: &Node<V>) {
        if !node.is_leaf() && self.is_closed(node) {
            self.open.push(node.id().to_string());
        }
        self.open_ancestors(root, node);
    }

    /// ### close_children
    ///
    /// Close all node children recursively
    fn close_children<V>(&mut self, node: &Node<V>) {
        node.iter().for_each(|x| self.close_node(x));
    }

    /// ### open_ancestors
    ///
    /// Open all ancestors for `node` in the current `tree`
    fn open_ancestors<V>(&mut self, root: &Node<V>, node: &Node<V>) {
        if let Some(parent) = root.parent(node.id()) {
            self.open_node(root, parent);
        }
    }

    /// ### previous_sibling
    ///
    /// Returns the previous sibling of `node` in `root`
    fn previous_sibling<'a, V>(
        &mut self,
        root: &'a Node<V>,
        node: &'a Node<V>,
    ) -> Option<&'a Node<V>> {
        let parent = root.parent(node.id())?;
        let mut prev_node = None;
        for child in parent.iter() {
            if child.id() == node.id() {
                break;
            }
            prev_node = Some(child);
        }
        prev_node
    }

    /// ### next_sibling
    ///
    /// Returs next sibling of `node` in `tree`
    fn next_sibling<'a, V>(&mut self, root: &'a Node<V>, node: &'a Node<V>) -> Option<&'a Node<V>> {
        let parent = root.parent(node.id())?;
        let mut keep_next = false;
        for child in parent.iter() {
            if keep_next {
                // Return child
                return Some(child);
            } else if child.id() == node.id() {
                // keep next element
                keep_next = true;
            }
        }
        // No next sibling
        None
    }

    /// Get last open heir for node
    fn get_last_open_heir<'a, V>(&self, node: &'a Node<V>) -> &'a Node<V> {
        if self.is_open(node) {
            // If node is open, get its last child and call this function recursively
            self.get_last_open_heir(node.iter().last().unwrap())
        } else {
            // Else return `node`
            node
        }
    }

    #[cfg(test)]
    /// ### force_open
    ///
    /// Force open nodes
    pub fn force_open(&mut self, open: &[&str]) {
        self.open = open.iter().map(|x| x.to_string()).collect();
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::mock::mock_tree;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_initialize_tree_state() {
        let state = TreeState::default();
        assert!(state.open.is_empty());
        assert!(state.selected().is_none());
    }

    #[test]
    fn should_select_nodes() {
        let mut state = TreeState::default();
        let tree = mock_tree();
        // select 'bA'
        state.select(tree.root(), tree.root().query(&String::from("bA")).unwrap());
        assert_eq!(state.selected().unwrap(), "bA");
        // All ancestors should be opened
        assert_eq!(state.open.len(), 2);
        assert!(state.is_open(tree.root().query(&String::from("b")).unwrap()));
        assert!(state.is_open(tree.root()));
    }

    #[test]
    fn should_open_and_close_nodes() {
        let mut state = TreeState::default();
        let tree = mock_tree();
        // Select node 'bA'
        let ba = tree.root().query(&String::from("bA")).unwrap();
        state.select(tree.root(), ba);
        // Open
        state.open(tree.root());
        assert!(state.is_open(ba));
        // At this point also its ancestors should be open
        assert!(state.is_open(tree.root().query(&String::from("b")).unwrap()));
        assert!(state.is_open(tree.root()));
        assert_eq!(state.open.len(), 3);
        // let's open its child 'bA0'
        let ba0 = tree.root().query(&String::from("bA0")).unwrap();
        state.select(tree.root(), ba0);
        state.open(tree.root());
        assert!(state.is_open(ba0));
        assert_eq!(state.open.len(), 4);
        // Now let's close 'b'
        let b = tree.root().query(&String::from("b")).unwrap();
        state.select(tree.root(), b);
        state.close(tree.root());
        assert!(state.is_closed(b));
        // All its children should be close
        assert!(state.is_closed(ba));
        assert!(state.is_closed(ba0));
        // But root should still be open
        assert!(state.is_open(tree.root()));
    }

    #[test]
    fn should_not_open_twice() {
        let mut state = TreeState::default();
        let tree = mock_tree();
        // Open root
        state.select(tree.root(), tree.root());
        state.open(tree.root());
        assert_eq!(state.open.len(), 1);
        assert!(state.is_open(tree.root()));
        // Open twice
        state.open(tree.root());
        assert_eq!(state.open.len(), 1);
        assert!(state.is_open(tree.root()));
    }

    #[test]
    fn should_find_previous_sibling() {
        let mut state = TreeState::default();
        let tree = mock_tree();
        let bb4 = tree.root().query(&String::from("bB4")).unwrap();
        // Prev siblign should be bb3
        let bb3 = tree.root().query(&String::from("bB3")).unwrap();
        assert_eq!(state.previous_sibling(tree.root(), bb4).unwrap(), bb3);
        // bb0 shouldn't have a previous sibling
        let bb0 = tree.root().query(&String::from("bB0")).unwrap();
        assert!(state.previous_sibling(tree.root(), bb0).is_none());
    }

    #[test]
    fn should_find_next_sibling() {
        let mut state = TreeState::default();
        let tree = mock_tree();
        let bb4 = tree.root().query(&String::from("bB4")).unwrap();
        // Next siblign should be bb3
        let bb5 = tree.root().query(&String::from("bB5")).unwrap();
        assert_eq!(state.next_sibling(tree.root(), bb4).unwrap(), bb5);
        // bb5 shouldn't have a previous sibling
        assert!(state.next_sibling(tree.root(), bb5).is_none());
    }

    #[test]
    fn should_find_first_sibling() {
        let mut state = TreeState::default();
        let tree = mock_tree();
        let bb4 = tree.root().query(&String::from("bB4")).unwrap();
        state.select(tree.root(), bb4);
        // First siblign should be bb1
        let bb0 = tree.root().query(&String::from("bB0")).unwrap();
        assert_eq!(state.first_sibling(tree.root()).unwrap(), bb0);
        // / shouldn't have a first sibling
        state.select(tree.root(), tree.root());
        assert!(state.first_sibling(tree.root()).is_none());
    }

    #[test]
    fn should_find_last_sibling() {
        let mut state = TreeState::default();
        let tree = mock_tree();
        let bb2 = tree.root().query(&String::from("bB2")).unwrap();
        state.select(tree.root(), bb2);
        // First siblign should be bb1
        let bb5 = tree.root().query(&String::from("bB5")).unwrap();
        assert_eq!(state.last_sibling(tree.root()).unwrap(), bb5);
        // / shouldn't have a last sibling
        state.select(tree.root(), tree.root());
        assert!(state.last_sibling(tree.root()).is_none());
    }

    #[test]
    fn should_preserve_tree_state() {
        let mut state = TreeState::default();
        let tree = mock_tree();
        // Open cA branch
        let ca = tree.root().query(&String::from("cA")).unwrap();
        state.select(tree.root(), ca);
        state.open(tree.root());
        let bb5 = tree.root().query(&String::from("bB5")).unwrap();
        state.select(tree.root(), bb5);
        // Reinitialize tree
        state.tree_changed(tree.root(), true);
        // States should have been preserved
        assert_eq!(state.open.len(), 5);
        assert!(state.is_open(ca));
        assert_eq!(state.selected().unwrap(), "bB5");
    }

    #[test]
    fn should_not_preserve_tree_state() {
        let mut state = TreeState::default();
        let mut tree = mock_tree();
        // Open cA branch
        let ca = tree.root().query(&String::from("cA")).unwrap();
        state.select(tree.root(), ca);
        state.open(tree.root());
        let bb5 = tree.root().query(&String::from("bB5")).unwrap();
        state.select(tree.root(), bb5);
        // New tree without 'c' and without 'bB'
        tree.root_mut()
            .query_mut(&String::from("b"))
            .unwrap()
            .remove_child(&String::from("bB"));
        tree.root_mut().remove_child(&String::from("c"));
        // Re initialize tree
        state.tree_changed(tree.root(), true);
        // Select should be root
        assert_eq!(state.selected().unwrap(), "/");
        // No node should be open, except for root and 'b'
        assert_eq!(state.open.len(), 2);
        assert_eq!(state.open, vec![String::from("/"), String::from("b")]);
        assert!(state.is_open(tree.root()));
    }

    #[test]
    fn should_reinitialize_tree_state() {
        let mut state = TreeState::default();
        let tree = mock_tree();
        // Open branches
        let ca = tree.root().query(&String::from("cA")).unwrap();
        state.select(tree.root(), ca);
        state.open(tree.root());
        let bb5 = tree.root().query(&String::from("bB5")).unwrap();
        state.select(tree.root(), bb5);
        // Re-initialize tree
        state.tree_changed(tree.root(), false);
        // No node should be open; selected should be root
        assert!(state.open.is_empty());
        assert_eq!(state.selected().unwrap(), "/");
    }

    #[test]
    fn should_move_cursor_down_on_sibling() {
        let mut state = TreeState::default();
        let tree = mock_tree();
        // Let's start from bB5
        let bb3 = tree.root().query(&String::from("bB3")).unwrap();
        state.select(tree.root(), bb3);
        // Move down (should become bb4)
        state.move_down(tree.root());
        assert_eq!(state.selected().unwrap(), "bB4");
        // Move down (should become bb5)
        state.move_down(tree.root());
        assert_eq!(state.selected().unwrap(), "bB5");
    }

    #[test]
    fn should_move_cursor_down_to_next_lower_node_if_last_child() {
        let mut state = TreeState::default();
        let tree = mock_tree();
        // Let's start from bB5
        let bb5 = tree.root().query(&String::from("bB5")).unwrap();
        state.select(tree.root(), bb5);
        // Move down (should become 'c')
        state.move_down(tree.root());
        assert_eq!(state.selected().unwrap(), "c");
    }

    #[test]
    fn should_move_cursor_down_to_child_if_parent_is_open() {
        let mut state = TreeState::default();
        let tree = mock_tree();
        // Let's start from b
        let b = tree.root().query(&String::from("b")).unwrap();
        state.select(tree.root(), b);
        // Open b
        state.open(tree.root());
        // Move down (should become 'bA')
        state.move_down(tree.root());
        assert_eq!(state.selected().unwrap(), "bA");
        // Open bA
        state.open(tree.root());
        // Move down (should become 'bA0')
        state.move_down(tree.root());
        assert_eq!(state.selected().unwrap(), "bA0");
    }

    #[test]
    fn should_move_cursor_down_to_next_sibling_if_node_is_closed() {
        let mut state = TreeState::default();
        let tree = mock_tree();
        // Let's start from b
        let aa = tree.root().query(&String::from("aA")).unwrap();
        state.select(tree.root(), aa);
        // Move down (should become 'aB')
        state.move_down(tree.root());
        assert_eq!(state.selected().unwrap(), "aB");
        // Move down (should become 'aC')
        state.move_down(tree.root());
        assert_eq!(state.selected().unwrap(), "aC");
    }

    #[test]
    fn should_not_move_cursor_down_if_root_is_closed() {
        let mut state = TreeState::default();
        let tree = mock_tree();
        state.select(tree.root(), tree.root());
        // Move cursor down
        state.move_down(tree.root());
        assert_eq!(state.selected().unwrap(), "/");
    }

    #[test]
    fn should_not_move_cursor_down_if_last_element_is_selected() {
        let mut state = TreeState::default();
        let tree = mock_tree();
        let ca2 = tree.root().query(&String::from("cA2")).unwrap();
        state.select(tree.root(), ca2);
        // Move cursor down (keep last element)
        state.move_down(tree.root());
        assert_eq!(state.selected().unwrap(), "cA2");
    }

    #[test]
    fn should_move_cursor_up_on_sibling() {
        let mut state = TreeState::default();
        let tree = mock_tree();
        // Let's start from bB5
        let bb4 = tree.root().query(&String::from("bB4")).unwrap();
        state.select(tree.root(), bb4);
        // Move up (should become 'bB3')
        state.move_up(tree.root());
        assert_eq!(state.selected().unwrap(), "bB3");
        // Move up (should become 'bB2')
        state.move_up(tree.root());
        assert_eq!(state.selected().unwrap(), "bB2");
    }

    #[test]
    fn should_move_cursor_up_on_deepest_child_of_previous_sibling() {
        let mut state = TreeState::default();
        let tree = mock_tree();
        // Open 'bB'
        state.select(tree.root(), tree.root().query(&String::from("bB")).unwrap());
        state.open(tree.root());
        // Select 'c'
        state.select(tree.root(), tree.root().query(&String::from("c")).unwrap());
        // Move up (should become 'bB5')
        state.move_up(tree.root());
        assert_eq!(state.selected().unwrap(), "bB5");
    }

    #[test]
    fn should_move_cursor_up_to_parent_if_first_child() {
        let mut state = TreeState::default();
        let tree = mock_tree();
        // Open 'bB'
        state.select(
            tree.root(),
            tree.root().query(&String::from("bB0")).unwrap(),
        );
        // Move up (should become 'bB')
        state.move_up(tree.root());
        assert_eq!(state.selected().unwrap(), "bB");
        // Move up (should become 'bA')
        state.move_up(tree.root());
        assert_eq!(state.selected().unwrap(), "bA");
        // Move up (should become 'b')
        state.move_up(tree.root());
        assert_eq!(state.selected().unwrap(), "b");
    }

    #[test]
    fn should_not_move_cursor_up_if_root_is_selected() {
        let mut state = TreeState::default();
        let tree = mock_tree();
        state.select(tree.root(), tree.root());
        // Move cursor down
        state.move_up(tree.root());
        assert_eq!(state.selected().unwrap(), "/");
    }

    #[test]
    fn should_get_last_open_heir() {
        let mut state = TreeState::default();
        let tree = mock_tree();
        // Open aA, aB, aC
        state.select(tree.root(), tree.root().query(&String::from("aA")).unwrap());
        state.open(tree.root());
        state.select(tree.root(), tree.root().query(&String::from("aB")).unwrap());
        state.open(tree.root());
        state.select(tree.root(), tree.root().query(&String::from("aC")).unwrap());
        state.open(tree.root());
        // Open bB
        state.select(tree.root(), tree.root().query(&String::from("bB")).unwrap());
        state.open(tree.root());
        // Get last open heir from root
        assert_eq!(
            state
                .get_last_open_heir(tree.root().query(&String::from("bB")).unwrap())
                .id()
                .as_str(),
            "bB5"
        );
        // Get last open heir from a
        assert_eq!(
            state
                .get_last_open_heir(tree.root().query(&String::from("a")).unwrap())
                .id()
                .as_str(),
            "aC0"
        );
    }
}
