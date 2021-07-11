use std::slice::{Iter, IterMut};

static BLOCK_SEPARATOR: &str = ",";

/// Used to build a block (block | row), specifying additional details.
#[derive(Default)]
pub struct BlockBuilder {
    size: BlockSize,
    min_size: f64,
    max_size: f64,
}

impl BlockBuilder {
    /// Creates a new `BlockBuilder` with default values.
    pub fn new() -> Self {
        BlockBuilder::default()
    }

    /// Inserts a new size.
    pub fn size(mut self, size: BlockSize) -> Self {
        self.size = size;
        self
    }

    /// Inserts a new min size.
    pub fn min_size(mut self, min_size: f64) -> Self {
        self.min_size = min_size;
        self
    }

    /// Inserts a new max size.
    pub fn max_size(mut self, max_size: f64) -> Self {
        self.max_size = max_size;
        self
    }

    /// Builds the block.
    pub fn build(self) -> Block {
        Block {
            size: self.size,
            min_size: self.min_size,
            max_size: self.max_size,
            current_size: 0.0,
        }
    }
}

/// Used to define the block of the `Grid`.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Block {
    pub size: BlockSize,
    pub min_size: f64,
    pub max_size: f64,
    current_size: f64,
}

impl Block {
    /// Creates a new `BlockBuilder` object with default values.
    #[inline]
    pub fn create() -> BlockBuilder {
        BlockBuilder::new()
    }

    /// Gets the block size.
    pub fn size(&self) -> BlockSize {
        self.size
    }

    /// Gets the current size.
    pub fn current_size(&self) -> f64 {
        self.current_size
    }

    /// Sets the current size.
    pub fn set_current_size(&mut self, size: f64) {
        self.current_size = if self.min_size == 0.0 && self.max_size == 0.0 && size > 0.0 {
            size
        } else if size < self.min_size && self.min_size > 0.0 {
            self.min_size
        } else if size > self.max_size && self.max_size > 0.0 {
            self.max_size
        } else {
            size
        };
    }
}

impl From<&str> for Block {
    fn from(t: &str) -> Self {
        if let Ok(size) = t.parse::<f64>() {
            return Block::create().size(BlockSize::Size(size)).build();
        }
        match t {
            "Auto" | "auto" => Block::create().size(BlockSize::Auto).build(),
            _ => Block::create().size(BlockSize::Stretch).build(),
        }
    }
}

impl From<f64> for Block {
    fn from(t: f64) -> Self {
        Block::create().size(BlockSize::Size(t)).build()
    }
}

impl From<i32> for Block {
    fn from(t: i32) -> Self {
        Block::create().size(BlockSize::Size(t.into())).build()
    }
}

/// Used to define the size of a grid block.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlockSize {
    /// Block is measured by the largest child.
    Auto,

    /// Block expands to the rest available size.
    Stretch,

    /// Defines a fixed size for the block.
    Size(f64),
}

impl Default for BlockSize {
    fn default() -> Self {
        BlockSize::Stretch
    }
}

/// Used to build a blocks, specifying additional details.
#[derive(Default)]
pub struct BlocksBuilder {
    blocks: Vec<Block>,
}

/// Used to build a blocks, specifying additional details.
impl BlocksBuilder {
    /// Creates a new `BlocksBuilder` with default values.
    pub fn new() -> Self {
        BlocksBuilder::default()
    }

    /// Inserts a new block.
    pub fn push<C: Into<Block>>(mut self, block: C) -> Self {
        self.blocks.push(block.into());
        self
    }

    /// Inserts a new block.
    #[inline(always)]
    #[deprecated = "Use push instead"]
    pub fn block<C: Into<Block>>(self, block: C) -> Self {
        self.push(block)
    }

    /// Inserts a list of blocks.
    pub fn blocks<R: Into<Block> + Clone>(mut self, blocks: &[R]) -> Self {
        for block in blocks.to_vec() {
            self.blocks.push(block.into());
        }
        self
    }

    /// Inserts the given block as often as given.
    pub fn repeat<R: Into<Block> + Copy>(mut self, block: R, count: usize) -> Self {
        for _ in 0..count {
            self.blocks.push(block.into())
        }
        self
    }

    /// Builds the blocks.
    pub fn build(self) -> Blocks {
        Blocks(self.blocks)
    }
}

impl From<BlocksBuilder> for Blocks {
    fn from(builder: BlocksBuilder) -> Self {
        builder.build()
    }
}

/// Helper struct used inside of the blocks property.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Blocks(pub Vec<Block>);

/// Alias type for `Blocks` to don't break old api.
pub type Columns = Blocks;

/// Alias type for `Blocks` to don't break old api.
pub type Rows = Blocks;

impl Blocks {
    /// Creates a new `BlocksBuilder` object with default values.
    #[inline]
    pub fn create() -> BlocksBuilder {
        BlocksBuilder::new()
    }

    /// Returns the number of elements in the blocks list, also referred to as its 'length'.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns a boolean if the block is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a reference to a block.
    pub fn get(&self, block: usize) -> Option<&Block> {
        self.0.get(block)
    }

    /// Returns a mutable reference to a block.
    pub fn get_mut(&mut self, block: usize) -> Option<&mut Block> {
        self.0.get_mut(block)
    }

    /// Returns an iterator over the slice.
    pub fn iter(&self) -> Iter<Block> {
        self.0.iter()
    }

    /// Returns a mutable iterator over the slice.
    pub fn iter_mut(&mut self) -> IterMut<Block> {
        self.0.iter_mut()
    }
}

impl From<&str> for Blocks {
    fn from(blocks: &str) -> Self {
        let blocks = blocks.replace(" ", "");
        let mut block_builder = BlocksBuilder::new();
        for block in blocks.split(BLOCK_SEPARATOR) {
            block_builder = block_builder.push(Block::from(block));
        }

        block_builder.build()
    }
}

impl From<String> for Blocks {
    fn from(blocks: String) -> Self {
        Self::from(blocks.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ERROR: f64 = f64::EPSILON;

    #[test]
    fn test_size() {
        let size = BlockSize::Size(64.0);

        let builder = BlockBuilder::new();
        let block = builder.size(size).build();

        assert_eq!(block.size, size);
    }

    #[test]
    fn test_min_size() {
        let min_size = 64.0;

        let builder = BlockBuilder::new();
        let block = builder.min_size(min_size).build();

        assert!((block.min_size - min_size).abs() < ERROR);
    }

    #[test]
    fn test_max_size() {
        let max_size = 64.0;

        let builder = BlockBuilder::new();
        let block = builder.max_size(max_size).build();

        assert!((block.max_size - max_size) < ERROR);
    }

    #[test]
    fn test_set_current_size() {
        let out_one_size = 10.0;
        let out_two_size = 66.0;
        let min_size = 14.0;
        let max_size = 64.0;

        let builder = BlockBuilder::new();
        let mut block = builder.min_size(min_size).max_size(max_size).build();

        block.set_current_size(out_one_size);
        assert!((block.current_size() - min_size).abs() < ERROR);

        block.set_current_size(out_two_size);
        assert!((block.current_size() - max_size).abs() < ERROR);
    }

    #[test]
    fn test_block() {
        let builder = BlocksBuilder::new();
        let blocks = builder.build();

        assert_eq!(blocks.len(), 0);

        let builder = BlocksBuilder::new();
        let blocks = builder
            .push(Block::create().build())
            .push(Block::create().build())
            .build();

        assert_eq!(blocks.len(), 2);
    }

    #[test]
    fn test_block_size_into() {
        let block: Block = "Auto".into();
        assert_eq!(block.size(), BlockSize::Auto);

        let block: Block = "auto".into();
        assert_eq!(block.size(), BlockSize::Auto);

        let block: Block = "Stretch".into();
        assert_eq!(block.size(), BlockSize::Stretch);

        let block: Block = "stretch".into();
        assert_eq!(block.size(), BlockSize::Stretch);

        let block: Block = "*".into();
        assert_eq!(block.size(), BlockSize::Stretch);

        let block: Block = "other".into();
        assert_eq!(block.size(), BlockSize::Stretch);

        let block: Block = 64.0.into();
        assert_eq!(block.size(), BlockSize::Size(64.0));
    }
}
