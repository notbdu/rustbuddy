use std::vec;

const NODE_UNUSED: u8 = 0;
const NODE_USED: u8 = 1;
const NODE_SPLIT: u8 = 2;
const NODE_FULL: u8 = 3;


pub struct BuddyAllocator {
    levels: usize,
    pub tree: vec::Vec<u8>,
}

impl BuddyAllocator {
    pub fn new(levels: usize) -> BuddyAllocator {
        let size: usize = (1 << levels + 1) - 1;
        return BuddyAllocator{
            levels: levels,
            tree: vec![NODE_UNUSED; size],
        };
    }
    
    // Takes a size (# of blocks requested) and returns an index offset
    pub fn allocate(&mut self, s: usize) -> isize {
        // Get the number of blocks requested
        let requested_blocks: f64;
        if s == 0 {
            requested_blocks = 1.0;
        } else {
            requested_blocks = s.next_power_of_two() as f64;
        }
        let requested_level = requested_blocks.log(2.0) as usize;
        if requested_level > self.levels {
            return -1;
        }

        // start at index 0 and move in
        let mut index = 0;
        let mut current_level = self.levels;
        'forward: loop {
            let has_buddy = index & 1 == 1;
            if current_level != requested_level {
                match self.tree[index] { 
                    NODE_USED | NODE_FULL => {
                        // Check the buddy node if we haven't already
                        if has_buddy {
                            index += 1;
                        }
                        continue 'forward;
                    }
                    NODE_UNUSED => {
                        // Split the node and descend
                        self.tree[index] = NODE_SPLIT;
                        index = index * 2 + 1;
                        current_level -= 1;
                        continue 'forward;
                    }
                    NODE_SPLIT => {
                        // Just descend
                        index = index * 2 + 1;
                        current_level -= 1;
                        continue 'forward;
                    }
                    _ => panic!("unknkown type {}", self.tree[index])
                }
            } else {
                // Requested level and current level match up
                if self.tree[index] == NODE_UNUSED {
                    self.tree[index] = NODE_USED;
                    // Recursively check if parents are full and mark them as such
                    self.update_parents((index + 1) / 2 - 1);
                    break 'forward;
                }
            }
            // Check buddy node if we haven't already
            if has_buddy {
                index += 1;
                continue 'forward;
            }
            // Backtrack if we reach a level match AND we've checked both nodes
            'backward: loop {
                index = (index + 1) / 2 - 1;
                current_level += 1;
                let has_buddy_inner = index & 1 == 1;
                if has_buddy_inner {
                    index += 1;
                    break 'backward;
                }
            }
        }

        return index as isize;
    }

    pub fn free(&mut self, index_offset: usize) {
        if index_offset > self.tree.len() - 1 {
            panic!("offset {} is > length of tree {}", index_offset, self.tree.len());
        }
        // Recursively free and combine nodes
        self.free_and_combine(index_offset);
        
        // Recursively update parents
        self.update_parents((index_offset + 1) / 2 - 1);
    }

    fn free_and_combine(&mut self, index: usize) {
        self.tree[index] = NODE_UNUSED;
        // We are already at the top of the tree, we're done
        if index == 0 {
            return;
        }
        let other_node: usize;
        let has_right_buddy = (index & 1) == 1;
        if has_right_buddy {
            other_node = index + 1;
        } else {
            other_node = index - 1;
        }
        // Recursively combine nodes
        if self.tree[other_node] == NODE_UNUSED {
            self.free_and_combine((index + 1) / 2 - 1);
        }
        return;
    }

    // Propagate changes up to parent nodes
    fn update_parents(&mut self, index: usize) {
        // Check both child nodes to see if they are both either FULL or USED
        let left_child = index * 2 + 1;
        let right_child = index * 2 + 2;
        let left_child_used_or_full = self.tree[left_child] == NODE_FULL || self.tree[left_child] == NODE_USED;
        let right_child_used_or_full = self.tree[right_child] == NODE_FULL || self.tree[right_child] == NODE_USED;
        if left_child_used_or_full && right_child_used_or_full {
            // Both children USED or FULL
            self.tree[index] = NODE_FULL;
        } else if self.tree[left_child] == NODE_UNUSED && self.tree[right_child] == NODE_UNUSED {
            // Both children are UNUSED
            self.tree[index] = NODE_UNUSED;
        } else {
            // Default to split node if neither FULL or UNUSED
            self.tree[index] = NODE_SPLIT;
        }
        // We're at the top of the tree, we're done
        if index == 0 {
            return;
        }
        self.update_parents((index + 1) / 2 - 1);
    }

    pub fn dump(&self) -> String { 
        let mut out = "".to_string();
        let mut row = "".to_string();
        let mut level = 0;
        let mut index = 0;
        loop {
            if index == self.tree.len() {
                break
            }
            match self.tree[index] {
                NODE_USED => row += "U",
                NODE_UNUSED => row += "O",
                NODE_SPLIT => row += "S",
                NODE_FULL => row += "F",
                _ => panic!("unknown node type {}", self.tree[index]),
            }
            if row.len() == 1 << level {
                out += &(row + "\n");
                row = "".to_string();
                level += 1;
            }
            index += 1;
        }
        return out;
    }
}
