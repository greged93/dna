use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
#[archive(compare(PartialEq), check_bytes)]
pub struct Segment<T> {
    pub blocks: Vec<T>,
}

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
#[archive(compare(PartialEq), check_bytes)]
pub struct BlockData<T> {
    pub block_number: u64,
    pub data: Vec<T>,
}

impl<T> Default for Segment<T> {
    fn default() -> Self {
        Self { blocks: Vec::new() }
    }
}

impl<T> Segment<T> {
    pub fn reset(&mut self) {
        self.blocks.clear();
    }
}
