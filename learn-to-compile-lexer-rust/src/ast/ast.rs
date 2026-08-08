use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FlatASTNode {
    node_type: String,
    value: String,
    children_indices: Vec<usize>,
}

//  public List<ASTNode> Children { get; set; } = new();

#[derive(serde::Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ASTNode {
    node_type: String,
    value: String,
    children: Vec<ASTNode>,
}

// Since the tree had to get falttened for pass it from C# it gets rebuilt in the expected way over here
pub fn rebuild_tree(flat: &Vec<FlatASTNode>, head: usize) -> ASTNode {
    let node = &flat[head];
    ASTNode {
        node_type: node.node_type.clone(),
        value: node.value.clone(),
        children: node.children_indices.iter().map(|&i| rebuild_tree(flat, i)).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flat_node(node_type: &str, value: &str, children_indices: Vec<usize>) -> FlatASTNode {
        FlatASTNode {
            node_type: node_type.to_string(),
            value: value.to_string(),
            children_indices,
        }
    }

    #[test]
    fn rebuilds_a_single_leaf_node() {
        let flat = vec![flat_node("NUMBER", "7", vec![])];

        let tree = rebuild_tree(&flat, 0);

        assert_eq!(tree.node_type, "NUMBER");
        assert_eq!(tree.value, "7");
        assert!(tree.children.is_empty());
    }

    #[test]
    fn rebuilds_a_binary_operation_tree() {
        // Flattened form of: (7 + 5), stored with the root ("+") at index 2,
        // and its two operand leaves at indices 0 and 1.
        let flat = vec![
            flat_node("NUMBER", "7", vec![]),
            flat_node("NUMBER", "5", vec![]),
            flat_node("PLUS", "+", vec![0, 1]),
        ];

        let tree = rebuild_tree(&flat, 2);

        assert_eq!(tree.node_type, "PLUS");
        assert_eq!(tree.value, "+");
        assert_eq!(tree.children.len(), 2);
        assert_eq!(tree.children[0].node_type, "NUMBER");
        assert_eq!(tree.children[0].value, "7");
        assert_eq!(tree.children[1].node_type, "NUMBER");
        assert_eq!(tree.children[1].value, "5");
    }

    #[test]
    fn rebuilds_nested_children_in_order() {
        // (4 - 1) + 2, root at index 3
        let flat = vec![
            flat_node("NUMBER", "4", vec![]),
            flat_node("NUMBER", "1", vec![]),
            flat_node("MINUS", "-", vec![0, 1]),
            flat_node("NUMBER", "2", vec![]),
            flat_node("PLUS", "+", vec![2, 3]),
        ];

        let tree = rebuild_tree(&flat, 4);

        assert_eq!(tree.node_type, "PLUS");
        assert_eq!(tree.children[0].node_type, "MINUS");
        assert_eq!(tree.children[0].children[0].value, "4");
        assert_eq!(tree.children[0].children[1].value, "1");
        assert_eq!(tree.children[1].node_type, "NUMBER");
        assert_eq!(tree.children[1].value, "2");
    }
}