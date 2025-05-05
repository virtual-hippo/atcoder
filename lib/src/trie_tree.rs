/// トライ木
/// 参考:
/// - https://algo-logic.info/trie-tree/
pub mod trie_tree {

    #[derive(Debug)]
    pub struct TrieNode {
        to: Vec<usize>,
    }

    impl TrieNode {
        pub fn new() -> Self {
            Self { to: vec![usize::MAX; 26] }
        }
    }

    #[derive(Debug)]
    pub struct TrieTree {
        nodes: Vec<TrieNode>,
    }

    impl TrieTree {
        pub fn new() -> Self {
            Self { nodes: vec![TrieNode::new()] }
        }

        pub fn add(&mut self, s: &str) {
            let mut v = 0;
            for ch in s.chars() {
                let u = (ch as u8 - 'a' as u8) as usize;

                if self.nodes[v].to[u] == usize::MAX {
                    self.nodes[v].to[u] = self.nodes.len();
                    self.nodes.push(TrieNode::new());
                }

                v = self.nodes[v].to[u];
            }
        }
    }
}
