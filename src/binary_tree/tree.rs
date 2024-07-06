use super::node::TreeNode;

#[derive(Debug)]
pub struct BinaryTree<T> {
	pub root: Option<Box<TreeNode<T>>>,
}

impl<T: PartialOrd> BinaryTree<T> {
	// Cria uma nova árvore binária vazia
	pub fn new() -> Self {
		BinaryTree { root: None }
	}

	// Insere um novo valor na árvore
	pub fn insert(&mut self, value: T) {
		let new_node = Box::new(TreeNode::new(value));
		match self.root {
			Some(ref mut node) => Self::insert_node(node, new_node),
			None => self.root = Some(new_node),
		}
	}

	// Função auxiliar para inserir um nó na árvore
	pub fn insert_node(current: &mut Box<TreeNode<T>>, new_node: Box<TreeNode<T>>) {
		if new_node.value < current.value {
			match current.left {
				Some(ref mut left_child) => Self::insert_node(left_child, new_node),
				None => current.left = Some(new_node),
			}
		} else {
			match current.right {
				Some(ref mut right_child) => Self::insert_node(right_child, new_node),
				None => current.right = Some(new_node),
			}
		}
	}

	// Pesquisa um valor na árvore
	pub fn search(&self, value: T) -> bool {
		Self::search_node(&self.root, value)
	}

	// Função auxiliar para pesquisar um valor na árvore
	pub fn search_node(current: &Option<Box<TreeNode<T>>>, value: T) -> bool {
		match current {
			Some(ref node) => {
				if value == node.value {
					true
				} else if value < node.value {
					Self::search_node(&node.left, value)
				} else {
					Self::search_node(&node.right, value)
				}
			}
			None => false,
		}
	}
}
