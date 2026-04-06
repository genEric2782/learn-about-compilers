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