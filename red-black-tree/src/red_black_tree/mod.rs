enum Color {
    Red,
    Black,
}

struct Node<T: PartialOrd> {
    value: T,
    color: Color,
}

pub struct RedBlackTree<T: PartialOrd> {
    pub root: Node<T>,
    pub left: Option<Box<RedBlackTree<T>>>,
    pub right: Option<Box<RedBlckTree<T>>>,
    pub parent: Option<Box<Node<T>>>,
}

impl<T> RedBlackTree<T> where T: PartialOrd {
    pub fn new(root_value: T) -> RedBlackTree<T> {
        RedBlackTree {
            root: Some(Node { value: root_value }),
            left_child: None,
            right_child: None,
            parent: None,
        }
    }

    pub fn push(&mut self, value: T) {
        let find = |v: T, tree: RedBlackTree| {
            let root_value = tree.root.value;
            if v >= root_value {
                if let None = tree.right {
                    tree.right = Some(Box(RedBlackTree::new(v)));
                }
            }
        };
    }
}
