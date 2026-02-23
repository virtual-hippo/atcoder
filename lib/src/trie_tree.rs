/// トライ木（Trie）
///
/// 文字列を効率的に管理する木構造。各ノードが1文字に対応し、根から葉へのパスが1つの文字列を表す。
/// 文字列の追加・検索が O(|s|) で行え、共通の接頭辞を持つ文字列はノードを共有するためメモリ効率も高い。
///
/// 参考:
/// - https://algo-logic.info/trie-tree/
pub mod trie_tree {

    ///
    /// a-z の 26 文字から成る文字列のみに対応
    /// 他の文字を対象にする場合は to を HashMap<char, usize> などに変更する必要あり
    ///
    #[derive(Debug)]
    pub struct TrieNode {
        to: [Option<usize>; 26],
        end_of_word: bool,
    }

    impl TrieNode {
        pub fn new() -> Self {
            Self { to: [None; 26], end_of_word: false }
        }
    }

    impl Default for TrieNode {
        fn default() -> Self {
            Self::new()
        }
    }

    #[derive(Debug)]
    pub struct TrieTree {
        nodes: Vec<TrieNode>,
    }

    impl Default for TrieTree {
        fn default() -> Self {
            Self::new()
        }
    }

    impl TrieTree {
        pub fn new() -> Self {
            Self { nodes: vec![TrieNode::new()] }
        }

        pub fn add(&mut self, s: &str) {
            let mut v = 0;

            for u in s.bytes().map(|ch| {
                debug_assert!(ch.is_ascii_lowercase());
                (ch - b'a') as usize
            }) {
                if self.nodes[v].to[u].is_none() {
                    self.nodes[v].to[u] = Some(self.nodes.len());
                    self.nodes.push(TrieNode::new());
                }

                v = self.nodes[v].to[u].unwrap();
            }

            // ルートノード (すべての文字列の接頭辞) は true にしない
            if v == 0 {
                return;
            }
            self.nodes[v].end_of_word = true;
        }

        /// 文字列 `s` が完全一致で登録されているか判定する
        pub fn contains(&self, s: &str) -> bool {
            if let Some(v) = self.traverse(s) {
                self.nodes[v].end_of_word
            } else {
                false
            }
        }

        /// `s` が登録済みのいずれかの文字列の接頭辞（または完全一致）であるか判定する
        pub fn is_prefix_of_any(&self, s: &str) -> bool {
            if let Some(v) = self.traverse(s) {
                v > 0
            } else {
                false
            }
        }

        fn traverse(&self, s: &str) -> Option<usize> {
            s.bytes()
                .map(|ch| {
                    debug_assert!(ch.is_ascii_lowercase());
                    (ch - b'a') as usize
                })
                .try_fold(0, |v, u| self.nodes[v].to[u])
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

    #[test]
    fn starts_with_basic() {
        let mut trie = TrieTree::new();
        trie.add("apple");
        trie.add("app");
        trie.add("banana");

        // 登録済み単語の接頭辞は true
        assert!(trie.is_prefix_of_any("app"));
        assert!(trie.is_prefix_of_any("appl"));
        assert!(trie.is_prefix_of_any("ban"));

        // 完全一致も接頭辞とみなす
        assert!(trie.is_prefix_of_any("apple"));
        assert!(trie.is_prefix_of_any("banana"));

        // トライに存在しないパスは false
        assert!(!trie.is_prefix_of_any("apz"));
        assert!(!trie.is_prefix_of_any("orange"));
    }

    #[test]
    fn starts_with_longer_than_registered() {
        let mut trie = TrieTree::new();
        trie.add("app");

        // "app" しか登録していないので "apple" は false
        assert!(!trie.is_prefix_of_any("apple"));
    }

    #[test]
    fn empty_str() {
        let mut trie = TrieTree::new();
        assert!(!trie.contains(""));
        assert!(!trie.is_prefix_of_any(""));

        trie.add("");

        assert!(!trie.contains(""));
        assert!(!trie.is_prefix_of_any(""));
    }
}
