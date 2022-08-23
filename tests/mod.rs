use bincode;
use merkle_tree::merkle::*;
use serde::Serialize;

#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Serialize, Clone)]
struct DummyBlock {
	id: String,
	name: String,
	#[serde(skip_serializing)]
	encoded: Vec<u8>,
}

impl DummyBlock {
	fn new(id: String, name: String) -> Self {
		let mut block = Self {
			id: id.clone(),
			name: name.clone(),
			encoded: vec![],
		};

		let encoded: Vec<u8> = bincode::serialize(&block).unwrap();

		block.encoded = encoded;

		block
	}
}

impl AsRef<[u8]> for DummyBlock {
	fn as_ref(&self) -> &[u8] {
		&self.encoded
	}
}

fn dummy_data() -> Vec<DummyBlock> {
	vec![
		DummyBlock::new(String::from("b1"), String::from("b1")),
		DummyBlock::new(String::from("b2"), String::from("b2")),
		DummyBlock::new(String::from("b3"), String::from("b3")),
		DummyBlock::new(String::from("b4"), String::from("b4")),
		DummyBlock::new(String::from("b5"), String::from("b5")),
		DummyBlock::new(String::from("b6"), String::from("b6")),
		DummyBlock::new(String::from("b7"), String::from("b7")),
		DummyBlock::new(String::from("b8"), String::from("b8")),
	]
}

#[test]
fn it_works() {
	let merkle = Merkle::new(dummy_data());

	assert_eq!(merkle.height(), 4);
}
