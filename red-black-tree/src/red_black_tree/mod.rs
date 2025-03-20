#[derive(Debug, PartialEq, Eq)]
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
    pub right: Option<Box<RedBlackTree<T>>>,
    pub parent: Option<Box<RedBlackTree<T>>>,
}

impl<T> RedBlackTree<T> where T: PartialOrd {
    pub fn new(root_value: T) -> RedBlackTree<T> {
        RedBlackTree {
            root: Node { value: root_value, color: Color::Red },
            left: None,
            right: None,
            parent: None,
        }
    }

    fn recoloring(&mut self, is_left: bool) {
        if is_left {
            let left = self.left.as_mut().unwrap();
            if Color::Red == (*left).root.color && Color::Red == self.root.color {
                self.root.color = Color::Black;
                if let Some(parent) = self.parent.as_mut() {
                    (*parent).root.color = Color::Red;
                    (*parent).recoloring(true);
                }
            }
        }
    }

    pub fn push(&mut self, value: T) {
        let find = |v: T, mut tree: RedBlackTree<T>| {
            let root_value = tree.root.value;
            if v >= root_value {
                if let None = tree.right {
                    tree.right = Some(Box::new(RedBlackTree::new(v)));
                }
            }
        };
    }
}
