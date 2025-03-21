// Solution 1: BFS
// pub fn find_all_recipes(
//     recipes: Vec<String>,
//     ingredients: Vec<Vec<String>>,
//     supplies: Vec<String>,
// ) -> Vec<String> {
//     use std::collections::HashSet;
//
//     let n = recipes.len();
//     let mut pass = vec![false; n];
//     let mut supplies: HashSet<String> = supplies.into_iter().collect();
//     let mut ans = vec![];
//
//     let mut cond = true;
//     while cond {
//         cond = false;
//         for i in 0..n {
//             if pass[i] {
//                 continue;
//             }
//             let makeable = (|| {
//                 for i in &ingredients[i] {
//                     if !supplies.contains(i) {
//                         return false;
//                     }
//                 }
//                 true
//             })();
//
//             if makeable {
//                 supplies.insert(recipes[i].clone());
//                 cond = true;
//                 pass[i] = true;
//                 ans.push(recipes[i].clone());
//                 break;
//             }
//         }
//     }
//
//     ans
// }

// Solution 2: DFS
use std::collections::{HashMap, HashSet};

fn check_recipe(
    i: usize,
    visited: &mut Vec<bool>,
    makeable: &mut Vec<bool>,
    supplies: &HashSet<String>,
    ingredients: &Vec<Vec<String>>,
    recipe2idx: &HashMap<String, usize>,
) -> bool {
    if visited[i] {
        return makeable[i];
    }
    visited[i] = true;

    for j in &ingredients[i] {
        let can = if supplies.contains(j) {
            true
        } else {
            match recipe2idx.get(j) {
                None => return false,
                Some(idx) => {
                    check_recipe(*idx, visited, makeable, supplies, ingredients, recipe2idx)
                }
            }
        };
        if !can {
            return makeable[i];
        }
    }

    makeable[i] = true;
    return makeable[i];
}

pub fn find_all_recipes(
    recipes: Vec<String>,
    ingredients: Vec<Vec<String>>,
    supplies: Vec<String>,
) -> Vec<String> {
    let n = recipes.len();
    let mut visited = vec![false; n];
    let mut makeable = vec![false; n];
    let supplies: HashSet<String> = supplies.into_iter().collect();
    let recipe2idx = recipes
        .clone()
        .into_iter()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect();

    for i in 0..n {
        check_recipe(
            i,
            &mut visited,
            &mut makeable,
            &supplies,
            &ingredients,
            &recipe2idx,
        );
    }

    let mut ans = vec![];
    for (i, make) in makeable.iter().enumerate() {
        if *make {
            ans.push(recipes[i].clone());
        }
    }
    ans
}

// Solution 3: topological sort
// pub fn find_all_recipes(
//     recipes: Vec<String>,
//     ingredients: Vec<Vec<String>>,
//     supplies: Vec<String>,
// ) -> Vec<String> {
//     use std::collections::{HashMap, HashSet, VecDeque};
//
//     let mut available_supplies = HashSet::new();
//     let mut recipe2idx = HashMap::new();
//     let mut dependency_graph: HashMap<&String, Vec<&String>> = HashMap::new();
//
//     for supply in &supplies {
//         available_supplies.insert(supply);
//     }
//
//     for (i, recipe) in recipes.iter().enumerate() {
//         recipe2idx.entry(recipe).or_insert(i);
//     }
//
//     let mut in_degree = vec![0; recipes.len()];
//
//     for i in 0..recipes.len() {
//         for ingredient in &ingredients[i] {
//             if !available_supplies.contains(ingredient) {
//                 dependency_graph
//                     .entry(ingredient)
//                     .or_default()
//                     .push(&recipes[i]);
//                 in_degree[i] += 1;
//             }
//         }
//     }
//
//     let mut queue = VecDeque::new();
//     for i in 0..recipes.len() {
//         if in_degree[i] == 0 {
//             queue.push_back(i);
//         }
//     }
//
//     let mut ans = vec![];
//     while let Some(i) = queue.pop_front() {
//         let recipe = &recipes[i];
//         ans.push(recipe.clone());
//
//         match dependency_graph.get(recipe) {
//             None => continue,
//             Some(dependent_recipes) => {
//                 for i in dependent_recipes {
//                     in_degree[recipe2idx[i]] -= 1;
//                     if in_degree[recipe2idx[i]] == 0 {
//                         queue.push_back(recipe2idx[i]);
//                     }
//                 }
//             }
//         }
//     }
//
//     ans
// }

fn main() {
    let t1 = find_all_recipes(
        vec!["bread".to_string()],
        vec![vec!["yeast".to_string(), "flour".to_string()]],
        vec!["yeast".to_string(), "flour".to_string(), "corn".to_string()],
    );
    assert!(t1.contains(&"bread".to_string()));

    let t2 = find_all_recipes(
        vec!["bread".to_string(), "sandwich".to_string()],
        vec![
            vec!["yeast".to_string(), "flour".to_string()],
            vec!["bread".to_string(), "meat".to_string()],
        ],
        vec!["yeast".to_string(), "flour".to_string(), "meat".to_string()],
    );
    assert!(t2.contains(&"bread".to_string()));
    assert!(t2.contains(&"sandwich".to_string()));

    let t3 = find_all_recipes(
        vec![
            "bread".to_string(),
            "sandwich".to_string(),
            "burger".to_string(),
        ],
        vec![
            vec!["yeast".to_string(), "flour".to_string()],
            vec!["bread".to_string(), "meat".to_string()],
            vec![
                "sandwich".to_string(),
                "meat".to_string(),
                "bread".to_string(),
            ],
        ],
        vec!["yeast".to_string(), "flour".to_string(), "meat".to_string()],
    );
    assert!(t3.contains(&"bread".to_string()));
    assert!(t3.contains(&"sandwich".to_string()));
    assert!(t3.contains(&"burger".to_string()));

    let t4 = find_all_recipes(
        vec![
            "burger".to_string(),
            "bread".to_string(),
            "sandwich".to_string(),
        ],
        vec![
            vec![
                "sandwich".to_string(),
                "meat".to_string(),
                "bread".to_string(),
            ],
            vec!["yeast".to_string(), "flour".to_string()],
            vec!["bread".to_string(), "meat".to_string()],
        ],
        vec!["yeast".to_string(), "flour".to_string(), "meat".to_string()],
    );
    assert!(t4.contains(&"bread".to_string()));
    assert!(t4.contains(&"sandwich".to_string()));
    assert!(t4.contains(&"burger".to_string()));

    println!("All passed!");
}
