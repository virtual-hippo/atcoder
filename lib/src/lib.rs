pub mod binary_search;
pub mod gcd;
pub mod mint;
pub mod prime_factorize;

// fn _dfs(visited: &mut HashMap<u32, bool>, graph: &HashMap::<u32, Vec<u32>>, pos: u32) {
//     visited.entry(pos).and_modify(|v| *v = true);
//     if let Some(vec) = graph.get(&pos) {
//         for i in vec.iter() {
//             if let Some(b) = visited.get(i) {
//                 if *b == false {
//                     dfs(visited, graph, *i);
//                 }
//             }
//         }
//     }
// }
