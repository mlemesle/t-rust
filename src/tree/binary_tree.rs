pub struct BinaryTree<'a, T> {
    value: T,
    left_child: Option<&'a Self>,
    right_child: Option<&'a Self>,
}

impl<T> BinaryTree<'static, T> {
    pub fn new(value: T) -> BinaryTree<'static, T> {
        BinaryTree {
            value: value,
            left_child: None,
            right_child: None,
        }
    }

    pub fn get_value(self) -> T {
        self.value
    }

    pub fn set_value(mut self, value: T) {
        self.value = value;
    }

    pub fn get_left_child(self) -> Option<&'static Self> {
        self.left_child
    }
    
    pub fn set_left_child(mut self, child: &'static BinaryTree<T>) {
        self.left_child = Some(child);
    }

    pub fn get_right_child(self) -> Option<&'static Self> {
        self.right_child
    }
    
    pub fn set_right_child(mut self, child: &'static BinaryTree<T>) {
        self.right_child = Some(child);
    }
}


