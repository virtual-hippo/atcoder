/// トライ木
/// 参考:
/// - https://algo-logic.info/trie-tree/
pub mod trie_tree {

    #[derive(Debug)]
    pub struct TrieNode {
        to: Vec<usize>,
        end_of_word: bool,
    }

    impl TrieNode {
        pub fn new() -> Self {
            Self { to: vec![usize::MAX; 26], end_of_word: false }
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
            self.nodes[v].end_of_word = true;
        }

        /// 文字列 `s` が完全一致で登録されているか判定する
        pub fn contains(&self, s: &str) -> bool {
            let mut v = 0;
            for ch in s.chars() {
                let u = (ch as u8 - 'a' as u8) as usize;
                if self.nodes[v].to[u] == usize::MAX {
                    return false;
                }
                v = self.nodes[v].to[u];
            }
            self.nodes[v].end_of_word
        }
    }
}

#[cfg(test)]
mod tests {
    use super::trie_tree::TrieTree;

    #[test]
    fn empty_trie_contains_nothing() {
        let trie = TrieTree::new();
        assert!(!trie.contains("a"));
        assert!(!trie.contains("abc"));
    }

    #[test]
    fn added_word_is_found() {
        let mut trie = TrieTree::new();
        trie.add("abc");
        assert!(trie.contains("abc"));
    }

    #[test]
    fn prefix_is_not_found() {
        let mut trie = TrieTree::new();
        trie.add("abc");
        // "ab" は add していないので false
        assert!(!trie.contains("ab"));
        assert!(!trie.contains("a"));
    }

    #[test]
    fn prefix_added_separately_is_found() {
        let mut trie = TrieTree::new();
        trie.add("abc");
        trie.add("ab");
        assert!(trie.contains("ab"));
        assert!(trie.contains("abc"));
    }

    #[test]
    fn multiple_words_with_common_prefix() {
        let mut trie = TrieTree::new();
        trie.add("apple");
        trie.add("app");
        trie.add("application");
        trie.add("banana");

        assert!(trie.contains("apple"));
        assert!(trie.contains("app"));
        assert!(trie.contains("application"));
        assert!(trie.contains("banana"));

        assert!(!trie.contains("appl"));
        assert!(!trie.contains("ban"));
        assert!(!trie.contains("orange"));
    }

    #[test]
    fn single_char_words() {
        let mut trie = TrieTree::new();
        trie.add("a");
        trie.add("z");
        assert!(trie.contains("a"));
        assert!(trie.contains("z"));
        assert!(!trie.contains("b"));
    }

    #[test]
    fn duplicate_add_is_idempotent() {
        let mut trie = TrieTree::new();
        trie.add("hello");
        trie.add("hello");
        assert!(trie.contains("hello"));
    }
}
