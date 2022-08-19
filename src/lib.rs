// Merkle Tree
//                       ------------
//                      | Hash(1, 2) | <- Root Node | 2^0 nodes
//                       ------------
//                      |            |
//                ------              ------
//               |                          |
//          ------------              ------------
//         | Hash(3, 4) |            | Hash(5, 6) | <- Internal Node | 2^1 nodes
//          ------------              ------------
//              |  |                      |  |
//          ----    ----              ----    ----
//         |            |            |            |
//    ----------   ----------   ----------   ----------
//   | Hash(b1) | | Hash(b2) | | Hash(b3) | | Hash(b4) |
//   |¯¯¯¯¯¯¯¯¯¯| |¯¯¯¯¯¯¯¯¯¯| |¯¯¯¯¯¯¯¯¯¯| |¯¯¯¯¯¯¯¯¯¯| <- Leaf | 2^2 nodes
//   |  Block1  | |  Block2  | |  Block3  | |  Block4  |
//    ----------   ----------   ----------   ----------

use blake2::{Blake2b512, Digest};
use std::collections::BinaryHeap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Node<B>
where
	B: Hash + Ord + PartialOrd + AsRef<[u8]>,
{
	Internal {
		hash: Vec<u8>,
		left: Box<Self>,
		right: Box<Self>,
	},
	Leaf(Block<B>),
}

//TODO: delete this
impl<B> From<&mut BinaryHeap<Node<B>>> for Node<B>
where
	B: Hash + Ord + PartialOrd + AsRef<[u8]>,
{
	fn from(queue: &mut BinaryHeap<Node<B>>) -> Self {
		while queue.len() > 1 {
			let n = queue.pop().unwrap().join(queue.pop().unwrap());
			queue.push(n)
		}

		queue.pop().unwrap()
	}
}

impl<B> Node<B>
where
	B: Hash + Ord + PartialOrd + AsRef<[u8]>,
{
	fn hash(&self) -> Vec<u8> {
		let hash = match self {
			Self::Internal {
				hash: h,
				left: _,
				right: _,
			} => h.to_vec(),
			Self::Leaf(b) => b.hash.to_vec(),
		};

		hash
	}

	pub fn build(blocks: Vec<B>) -> Self {
		assert!(blocks.len() > 1, "You need to provide more blocks");
		assert_eq!(
			(blocks.len() as f32).log2().fract(),
			0.0,
			"You must provide n block on base two"
		);

		//TODO: replace build method from queue
		//NOTE: this works fine for huffman tree but in merkle tree
		//we don't need a unique path for each leaf
		//instead of in a merkle tree we want a balanced tree
		let mut q = BinaryHeap::from_iter(blocks.into_iter().map(Self::leaf));

		Self::from(&mut q)
	}

	fn join(self, node: Self) -> Self {
		let parent = if self < node {
			Self::internal(self, node)
		} else {
			Self::internal(node, self)
		};
		parent
	}

	fn internal(left: Self, right: Self) -> Self {
		let mut hasher = Blake2b512::new();

		let hash_sum = hash_adder(left.hash(), right.hash());

		hasher.update(&hash_sum);

		Self::Internal {
			hash: hasher.finalize().to_vec(),
			left: Box::new(left),
			right: Box::new(right),
		}
	}

	fn leaf(block: B) -> Self {
		Self::Leaf(Block::new(block))
	}
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Block<B>
where
	B: Hash + Ord + PartialOrd + AsRef<[u8]>,
{
	hash: Vec<u8>,
	block: B,
}

impl<B> Block<B>
where
	B: Hash + Ord + PartialOrd + AsRef<[u8]>,
{
	fn new(block: B) -> Self {
		let mut hasher = Blake2b512::new();

		hasher.update(&block);

		Self {
			hash: hasher.finalize().to_vec(),
			block,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use bincode;
	use serde::Serialize;

	#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Serialize, Clone)]
	struct DummyBlock {
		id: String,
		name: String,
		encoded: Vec<u8>,
	}

	#[derive(Serialize)]
	struct Block {
		id: String,
		name: String,
	}

	impl DummyBlock {
		fn new(id: String, name: String) -> Self {
			let de = Block {
				id: id.clone(),
				name: name.clone(),
			};
			let encoded: Vec<u8> = bincode::serialize(&de).unwrap();
			Self { id, name, encoded }
		}
	}

	impl AsRef<[u8]> for DummyBlock {
		fn as_ref(&self) -> &[u8] {
			&self.encoded
		}
	}

	#[test]
	fn it_works() {
		assert!(true);
	}

	#[test]
	fn create() {
		let blocks = vec![
			DummyBlock::new(String::from("b1"), String::from("b1")),
			DummyBlock::new(String::from("b2"), String::from("b2")),
			DummyBlock::new(String::from("b3"), String::from("b3")),
			DummyBlock::new(String::from("b4"), String::from("b4")),
		];

		let root = Node::build(blocks);

		println!("{:?}", root);
	}
}

fn hash_adder(left: Vec<u8>, right: Vec<u8>) -> Vec<u8> {
	let mut res: Vec<u8> = vec![];
	let mut carry = false;

	let (left, right) = if right.len() > left.len() {
		(right, left)
	} else {
		(left, right)
	};

	for (i, v) in left.iter().rev().enumerate() {
		let mut sum = if carry { 1u8 } else { 0u8 };

		if right.len() > i {
			let r = right[right.len() - i - 1];

			let s = v.overflowing_add(r);

			sum += s.0;

			carry = s.1;
		} else {
			sum += v;

			carry = false;
		}
		res.insert(0, sum);
	}

	if carry {
		res.insert(0, 1);
	}

	res
}
