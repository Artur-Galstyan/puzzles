use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    vec,
};

// medium
pub fn problem_207() {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut requirements: HashMap<i32, Vec<i32>> = HashMap::new();

        for preq in prerequisites {
            let (c, r) = (preq[0], preq[1]);
            requirements.entry(c).or_default().push(r);
        }

        let mut path: HashSet<i32> = HashSet::new();

        let mut memo: HashMap<i32, bool> = HashMap::new();

        fn dfs(
            course: i32,
            path: &mut HashSet<i32>,
            memo: &mut HashMap<i32, bool>,
            requirements: &HashMap<i32, Vec<i32>>,
        ) -> bool {
            if let Some(r) = memo.get(&course) {
                return *r;
            }
            if path.contains(&course) {
                memo.insert(course, true);
                return true;
            }

            path.insert(course);

            if let Some(req) = requirements.get(&course) {
                for c in req {
                    if dfs(*c, path, memo, requirements) {
                        memo.insert(course, true);
                        return true;
                    }
                }
            }

            path.remove(&course);

            memo.insert(course, false);
            false
        }

        for i in 0..num_courses {
            if dfs(i, &mut path, &mut memo, &requirements) {
                return false;
            }
        }

        true
    }

    // let prerequisites = vec![vec![1, 0], vec![0, 1]];
    let prerequisites = vec![
        vec![0, 10],
        vec![0, 10],
        vec![3, 18],
        vec![5, 5],
        vec![6, 11],
        vec![11, 14],
        vec![13, 1],
        vec![15, 1],
        vec![17, 4],
    ];
    can_finish(3, prerequisites);
}

// easy
pub fn problem_104() {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, PartialEq, Eq)]
    pub struct TreeNode {
        pub val: i32,
        pub left: Option<Rc<RefCell<TreeNode>>>,
        pub right: Option<Rc<RefCell<TreeNode>>>,
    }

    impl TreeNode {
        #[inline]
        pub fn new(val: i32) -> Self {
            TreeNode {
                val,
                left: None,
                right: None,
            }
        }
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(current: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
            let Some(tree_node) = current else {
                return depth;
            };
            let node = tree_node.borrow();
            max(
                dfs(node.left.clone(), depth + 1),
                dfs(node.right.clone(), depth + 1),
            )
        }

        dfs(root, 0)
    }
}

// medium
pub fn problem_200() {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid.clone();
        let n_cols = grid[0].len();

        grid.insert(0, vec!['0'; n_cols]);
        grid.push(vec!['0'; n_cols]);

        for row in &mut grid {
            row.insert(0, '0');
            row.push('0');
        }

        let mut island_counter = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        for (i, row) in grid.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == '0' {
                    continue;
                } else {
                    if visited.contains(&(i, j)) {
                        continue;
                    } else {
                        island_counter += 1;
                        traverse(i, j, &mut visited, &grid);
                    }
                }
            }
        }

        fn get_neighbor_deltas(x: usize, y: usize) -> Vec<(usize, usize)> {
            let upper = (x, y - 1);
            let lower = (x, y + 1);
            let right = (x + 1, y);
            let left = (x - 1, y);

            vec![upper, lower, right, left]
        }

        fn traverse(
            x: usize,
            y: usize,
            visited: &mut HashSet<(usize, usize)>,
            grid: &Vec<Vec<char>>,
        ) {
            visited.insert((x, y));
            let neighbors = get_neighbor_deltas(x, y);
            for neighbor in neighbors {
                let (n_x, n_y) = neighbor;
                if grid[n_x][n_y] == '0' || visited.contains(&(n_x, n_y)) {
                    continue;
                } else {
                    traverse(n_x, n_y, visited, grid);
                }
            }
        }

        island_counter
    }
}
