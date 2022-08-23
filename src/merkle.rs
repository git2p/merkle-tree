use crate::tree::*;
use std::hash::Hash;

pub struct Merkle<B>
where
	B: Clone + Hash + Ord + PartialOrd + Eq + PartialEq + AsRef<[u8]>,
{
	head: Node<B>,

	//https://en.wikipedia.org/wiki/Merkle_tree#Second_preimage_attack
	height: usize,
}

impl<B> Merkle<B>
where
	B: Clone + Hash + Ord + PartialOrd + Eq + PartialEq + AsRef<[u8]>,
{
	pub fn new(blocks: Vec<B>) -> Self {
		assert!(blocks.len() > 1, "You need to provide more blocks");
		//only allow to work with perfect trees
		assert_eq!(
			(blocks.len() as f32).log2().fract(),
			0.0,
			"You must to provide n bloks in n^2 form"
		);

		let head = Node::from(blocks);
		let height = head.clone().height();

		Self { head, height }
	}

	pub fn head(&self) -> &Node<B> {
		&self.head
	}

	pub fn height(&self) -> usize {
		self.height
	}
}
