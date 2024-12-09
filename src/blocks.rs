use std::{collections::HashMap, fmt::Display};

type FileId = usize;

pub struct FileBlocks {
    blocks: Vec<Block>,
}

impl FileBlocks {
    pub fn from_disk_map(disk_map: &[usize]) -> Self {
        let mut blocks = Vec::with_capacity(10 * disk_map.len());
        for (file_id, chunk) in disk_map.chunks(2).enumerate() {
            if chunk.len() == 1 {
                // last chunk, always full
                blocks.push(Block::new(chunk[0], BlockKind::Used(file_id)));
                continue;
            }
            let (used_len, empty_len) = (chunk[0], chunk[1]);
            blocks.push(Block::new(used_len, BlockKind::Used(file_id)));
            blocks.push(Block::new(empty_len, BlockKind::Empty));
        }

        Self { blocks }
    }

    pub fn checksum(&self) -> usize {
        let mut sum = 0;
        let mut pos = 0;
        for &block in &self.blocks {
            if block.is_used() {
                let file_id = block.file_id().unwrap();
                for _ in 0..block.len {
                    sum += file_id * pos;
                    pos += 1;
                }
            } else {
                // move the pointer through the entire empty block
                pos += block.len;
            }
        }

        sum
    }

    pub fn compact_fragmented(&mut self) {
        let Some(mut index_of_next_empty) = self.get_next_empty(0) else {
            return;
        };

        let mut last_full = self.get_index_of_previous_used(self.blocks.len());
        while let Some(mut index_of_used_block) = last_full {
            if index_of_used_block < index_of_next_empty {
                return;
            }

            let used_block = self.blocks[index_of_used_block];
            let file_id = used_block.file_id().unwrap();
            // remove the used block
            self.blocks[index_of_used_block] = Block::new(used_block.len, BlockKind::Empty);
            let mut remaining_len = used_block.len;
            loop {
                let empty_block = self.blocks[index_of_next_empty];
                if remaining_len < empty_block.len {
                    // use the first 'remaining_len' of the empty block for the used block
                    self.blocks.insert(
                        index_of_next_empty,
                        Block::new(remaining_len, BlockKind::Used(file_id)),
                    );
                    self.blocks[index_of_next_empty + 1].len -= remaining_len;
                    // need to shift the indices because we inserted something
                    index_of_next_empty += 1;
                    index_of_used_block += 1;
                    break;
                } else if remaining_len == empty_block.len {
                    self.blocks[index_of_next_empty] =
                        Block::new(remaining_len, BlockKind::Used(file_id));
                    index_of_next_empty = self.get_next_empty(index_of_next_empty).unwrap();
                    break;
                } else {
                    // remaining_len > empty_block.len
                    self.blocks[index_of_next_empty] =
                        Block::new(empty_block.len, BlockKind::Used(file_id));
                    remaining_len -= empty_block.len;
                    index_of_next_empty = self.get_next_empty(index_of_next_empty).unwrap();
                }
            }
            last_full = self.get_index_of_previous_used(index_of_used_block);
        }
    }

    pub fn compact_whole(&mut self) {
        let mut last_used_index = self.blocks.len();
        while let Some(mut used_block_idx) = self.get_index_of_previous_used(last_used_index) {
            last_used_index = used_block_idx;
            let used_block = self.blocks[used_block_idx];
            let Some(mut empty_block_idx) = self.get_next_empty(0) else {
                return;
            };
            loop {
                // go through all empty blocks to the left of the used block
                if empty_block_idx > used_block_idx {
                    break;
                }
                let empty_block = self.blocks[empty_block_idx];
                if empty_block.len == used_block.len {
                    // the used block fits exactly, just swap
                    self.blocks.swap(used_block_idx, empty_block_idx);
                    break;
                } else if empty_block.len > used_block.len {
                    // the used block fits, but some space remains in the empty block
                    self.blocks[used_block_idx] = Block::new(used_block.len, BlockKind::Empty);
                    self.blocks.insert(empty_block_idx, used_block);
                    self.blocks[empty_block_idx + 1].len -= used_block.len;

                    last_used_index += 1; // adjust index due to insertion
                    break;
                } else {
                    // need to look for another empty block
                    empty_block_idx = self.get_next_empty(empty_block_idx).unwrap();
                }
            }
        }
    }

    fn get_next_empty(&self, start: usize) -> Option<usize> {
        (start + 1..self.blocks.len()).find(|&i| self.blocks[i].is_empty())
    }

    fn get_index_of_previous_used(&self, start: usize) -> Option<usize> {
        (0..start).rev().find(|&i| self.blocks[i].is_used())
    }

    pub fn get_string_if_possible(&self) -> Option<String> {
        if self
            .blocks
            .iter()
            .all(|b| b.file_id().is_none() || (b.file_id().is_some() && b.file_id().unwrap() < 10))
        {
            Some(format!(
                "{}",
                self.blocks
                    .iter()
                    .map(|b| {
                        let c = match b.kind {
                            BlockKind::Used(id) => char::from_digit(id as u32, 10).unwrap(),
                            BlockKind::Empty => '.',
                        };
                        std::iter::repeat(c).take(b.len)
                    })
                    .flatten()
                    .collect::<String>()
            ))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Block {
    len: usize,
    kind: BlockKind,
}

impl Block {
    fn new(len: usize, kind: BlockKind) -> Self {
        Self { len, kind }
    }

    #[must_use]
    fn is_used(&self) -> bool {
        matches!(self.kind, BlockKind::Used(_))
    }

    #[must_use]
    fn is_empty(&self) -> bool {
        matches!(self.kind, BlockKind::Empty)
    }

    fn file_id(&self) -> Option<FileId> {
        match self.kind {
            BlockKind::Used(file_id) => Some(file_id),
            BlockKind::Empty => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockKind {
    Used(FileId),
    Empty,
}
