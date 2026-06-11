use std::sync::Arc;

#[derive(Copy, Clone)]
enum Color { R, B }

enum RbTree<T> {
    Empty,
    Tree(Color, Arc<RbTree<T>>, T, Arc<RbTree<T>>),
}

fn balance<T: Copy>(tree: Arc<RbTree<T>>) -> Arc<RbTree<T>> {
    let make = |a: &Arc<RbTree<T>>, x: T, b: &Arc<RbTree<T>>, y: T, c: &Arc<RbTree<T>>, z: T, d: &Arc<RbTree<T>>| {
        Arc::new(RbTree::Tree(Color::R,
            Arc::new(RbTree::Tree(Color::B, a.clone(), x, b.clone())), y,
            Arc::new(RbTree::Tree(Color::B, c.clone(), z, d.clone()))))
    };

    match tree.as_ref() {
        RbTree::Tree(Color::B, l_tree, elem, r_tree) => {
            match (l_tree.as_ref(), r_tree.as_ref()) {
                (RbTree::Tree(Color::R, ll_tree, l_elem, lr_tree), _) => {
                    match (ll_tree.as_ref(), lr_tree.as_ref()) {
                        (RbTree::Tree(Color::R, a, x, b), _) =>
                            make(a, *x, b, *l_elem, lr_tree, *elem, r_tree),
                        (_, RbTree::Tree(Color::R, b, y, c)) =>
                            make(ll_tree, *l_elem, b, *y, c, *elem, r_tree),
                        _ => Arc::clone(&tree),
                    }
                }
                (_, RbTree::Tree(Color::R, rl_tree, r_elem, rr_tree)) => {
                    match (rl_tree.as_ref(), rr_tree.as_ref()) {
                        (RbTree::Tree(Color::R, b, y, c), _) =>
                            make(l_tree, *elem, b, *y, c, *r_elem, rr_tree),
                        (_, RbTree::Tree(Color::R, c, z, d)) =>
                            make(l_tree, *elem, rl_tree, *r_elem, c, *z, d),
                        _ => Arc::clone(&tree),
                    }
                }
                _ => Arc::clone(&tree),
            }
        }
        _ => Arc::clone(&tree),
    }
}

fn ins<T: Copy + PartialOrd>(tree: &Arc<RbTree<T>>, val: T) -> Arc<RbTree<T>> {
    match tree.as_ref() {
        RbTree::Empty => Arc::new(RbTree::Tree(
            Color::R,
            Arc::new(RbTree::Empty),
            val,
            Arc::new(RbTree::Empty),
        )),
        RbTree::Tree(color, left, v, right) => {
            if val < *v {
                balance(Arc::new(RbTree::Tree(*color, ins(left, val), *v, right.clone())))
            } else if val > *v {
                balance(Arc::new(RbTree::Tree(*color, left.clone(), *v, ins(right, val))))
            } else {
                Arc::clone(tree)
            }
        }
    }
}

fn insert<T: Copy + PartialOrd>(tree: Arc<RbTree<T>>, val: T) -> Arc<RbTree<T>> {
    let result = ins(&tree, val);
    match result.as_ref() {
        RbTree::Tree(_, left, v, right) =>
            Arc::new(RbTree::Tree(Color::B, left.clone(), *v, right.clone())),
        RbTree::Empty => result,
    }
}

fn contains<T: Copy + PartialOrd>(tree: &Arc<RbTree<T>>, val: T) -> bool {
    match tree.as_ref() {
        RbTree::Empty => false,
        RbTree::Tree(_, left, v, right) => {
            if val < *v {
                contains(left, val)
            } else if val > *v {
                contains(right, val)
            } else {
                true
            }
        }
    }
}

fn to_sorted_list<T: Copy>(tree: &Arc<RbTree<T>>) -> Vec<T> {
    match tree.as_ref() {
        RbTree::Empty => vec![],
        RbTree::Tree(_, left, v, right) => {
            let mut result = to_sorted_list(left);
            result.push(*v);
            result.extend(to_sorted_list(right));
            result
        }
    }
}

fn main() {
    let mut tree: Arc<RbTree<i32>> = Arc::new(RbTree::Empty);

    let values = [7, 3, 18, 10, 22, 8, 11, 26, 2, 6, 13];
    println!("Inserting: {:?}", values);
    for v in values {
        tree = insert(tree, v);
    }
    println!("Sorted: {:?}", to_sorted_list(&tree));
    for v in [10, 5, 22] {
        println!("contains({}) = {}", v, contains(&tree, v));
    }

    let mut t: Arc<RbTree<i32>> = Arc::new(RbTree::Empty);
    for v in 1..=20 { t = insert(t, v); }
    assert_eq!(to_sorted_list(&t), (1..=20).collect::<Vec<_>>());
    println!("Ascending 1..20: OK");

    let mut t: Arc<RbTree<i32>> = Arc::new(RbTree::Empty);
    for v in (1..=20).rev() { t = insert(t, v); }
    assert_eq!(to_sorted_list(&t), (1..=20).collect::<Vec<_>>());
    println!("Descending 20..1: OK");

    let mut t: Arc<RbTree<i32>> = Arc::new(RbTree::Empty);
    for v in [5, 5, 5, 3, 3] { t = insert(t, v); }
    assert_eq!(to_sorted_list(&t), vec![3, 5]);
    println!("Duplicates ignored: OK");

    println!("\nAll tests passed!");
}
