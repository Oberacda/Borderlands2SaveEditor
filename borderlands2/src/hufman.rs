extern crate bit_vec;

pub mod hufman {
    ///
    /// Struct describing a node in a huffman tree.
    /// 
    #[derive(Copy, Clone, Debug)]
    struct Node {
        /// Symbol encoded in this node of the tree.
        symbol : u8,
        /// Is this node a leaf?
        is_leaf : bool,
        /// Index of the left subtree. If node is a leaf, this is -1.
        left : usize,
        /// Index of the right subtree. If node is a leaf, this is -1.
        right : usize
    }
    impl Default for Node {
    fn default() -> Node {
        Node {symbol: 0, is_leaf: false, right: 0, left: 0}
   }
}

    fn decode_byte(data: & bit_vec::BitVec, offset: & mut usize) -> u8 {
        let mut value : u8 = 0;
        for i in 7..0 {
            let v = if data[*offset] {
                1
            } else {
                0
            };
            *offset = *offset + 1;
            value |= v << i;
        }
        return value;
    }

    fn decode_node(data: & bit_vec::BitVec, offset: & mut usize, tree: & mut [Node], index: & mut usize) -> usize{
        let current = *index;
        *index = *index + 1;

        let is_leaf = data[*offset];
        *offset = *offset + 1;

        if is_leaf {
            let value = decode_byte(&data, offset);
            tree[current].left = 0;
            tree[current].right = 0;
            tree[current].is_leaf = true;
            tree[current].symbol = value;
        } else {
            tree[current].is_leaf = false;
            tree[current].left = decode_node(&data, offset, tree, index);
            tree[current].left = decode_node(&data, offset, tree, index);
        }

        return current;
    }

    pub fn decode(input: &[u8]) -> &[u8] {
        let bit_vec = bit_vec::BitVec::from_bytes(input);
        let mut tree : [Node; 511] = [Default::default(); 511];
        let mut index : usize = 0;
        let mut offset : usize= 0;
        
        decode_node(& bit_vec, & mut offset, & mut tree, & mut index);

        return input;
    }
}