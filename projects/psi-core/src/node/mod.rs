use dashmap::DashMap;
use gen_iter::GenIter;

pub type NodeID = usize;

pub struct ProgramNode {
    id: NodeID,
}

pub struct FunctionNode {
    id: NodeID,
}

pub struct NodeManager {
    /// node -> parent
    tree_structure: DashMap<NodeID, NodeID>,
}

impl NodeManager {
    pub fn register_relation(&self, node: NodeID, parent: NodeID) {
        self.tree_structure.insert(node, parent);
    }
    /// Remove node and all its descendant
    pub fn remove_node(&self, node: NodeID) {
        for i in self.find_descendant(node) {
            self.tree_structure.remove(&i);
        }
        self.tree_structure.remove(&node);
    }
    pub fn garbage_collect(&mut self, _keep_roots: &[NodeID]) {}
    pub fn find_root(&self, node: NodeID) -> Option<NodeID> {
        self.find_ancestor(node).last()
    }
    pub fn is_root(&self, node: NodeID) -> bool {
        self.find_parent(node).is_none()
    }
    pub fn find_parent(&self, node: NodeID) -> Option<NodeID> {
        self.tree_structure.get(&node).map(|v| *v.value())
    }
    pub fn find_ancestor(&self, node: NodeID) -> impl Iterator<Item = NodeID> + '_ {
        let mut current = node;
        GenIter(move || {
            while let Some(parent) = self.find_parent(current) {
                if parent.eq(&current) {
                    yield parent;
                }
                current = parent;
            }
        })
    }
    pub fn find_children(&self, node: NodeID) -> impl Iterator<Item = NodeID> + '_ {
        GenIter(move || {
            for item in self.tree_structure.iter() {
                if node.eq(item.value()) {
                    yield *item.key()
                }
            }
        })
    }
    pub fn find_descendant(&self, node: NodeID) -> impl Iterator<Item = NodeID> + '_ {
        let mut collected = vec![node];
        GenIter(move || {
            while let Some(node) = collected.pop() {
                for child in self.find_children(node) {
                    collected.push(child);
                    yield child;
                }
            }
        })
    }
}
