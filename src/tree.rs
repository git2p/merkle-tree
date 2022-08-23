use crate::utils::hash_adder;
use blake2::{Blake2b512, Digest};
use std::hash::Hash;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Node<B>
where
	B: Hash + Ord + PartialOrd + Eq + PartialEq + AsRef<[u8]>,
{
	Internal {
		hash: Vec<u8>,
		left: Box<Self>,
		right: Box<Self>,
	},
	Leaf {
		hash: Vec<u8>,
		block: B,
	},
}

impl<B> From<Vec<B>> for Node<B>
where
	B: Hash + Ord + PartialOrd + Eq + PartialEq + AsRef<[u8]>,
{
	fn from(blocks: Vec<B>) -> Self {
		let mut b = blocks
			.into_iter()
			.map(Self::new_leaf)
			.collect::<Vec<Self>>();

		while b.len() > 1 {
			let n = b.remove(0).join(b.remove(0));

			b.push(n);
		}

		b.remove(0)
	}
}

impl<B> Node<B>
where
	B: Hash + Ord + PartialOrd + Eq + PartialEq + AsRef<[u8]>,
{
	fn hash(&self) -> Vec<u8> {
		match self {
			Self::Internal { hash: h, .. } | Self::Leaf { hash: h, .. } => h.to_vec(),
		}
	}

	fn join(self, node: Self) -> Self {
		if self < node {
			Self::new_internal(self, node)
		} else {
			Self::new_internal(node, self)
		}
	}

	fn new_internal(left: Self, right: Self) -> Self {
		let mut hasher = Blake2b512::new();

		let hash_sum = hash_adder(left.hash(), right.hash());

		hasher.update(&hash_sum);

		Self::Internal {
			hash: hasher.finalize().to_vec(),
			left: Box::new(left),
			right: Box::new(right),
		}
	}

	fn new_leaf(block: B) -> Self {
		let mut hasher = Blake2b512::new();

		hasher.update(&block);

		Self::Leaf {
			hash: hasher.finalize().to_vec(),
			block,
		}
	}

	fn _is_leaf(&self) -> bool {
		matches!(self, Self::Leaf { .. })
	}

	fn advance(self, side: Side) -> Option<Self> {
		match self {
			Self::Leaf { .. } => None,
			Self::Internal {
				hash: _,
				left: l,
				right: r,
			} => match side {
				Side::Left => Some(*l),
				Side::Right => Some(*r),
			},
		}
	}

	fn left(self) -> Option<Self> {
		self.advance(Side::Left)
	}

	/*fn right(self) -> Option<Self> {
		self.advance(Side::Right)
	}

	pub fn walk<F>(self, mode: WalkMode, apply: F)
	where
		F: FnMut(&Self),
	{
		match mode {
			WalkMode::Order => {
				todo!()
			}
			WalkMode::PreOrder => {
				todo!()
			}
			WalkMode::PostOrder => {
				todo!()
			}
		}
	}*/

	pub fn height(self) -> usize {
		match self.left() {
			Some(n) => n.height() + 1,
			None => 1
		}
	}
}

enum Side {
	Left,
	Right,
}

/*#[derive(Debug)]
pub enum WalkMode {
	Order,
	PreOrder,
	PostOrder,
}*/
