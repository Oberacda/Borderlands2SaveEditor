extern crate bit_vec;

pub mod hufman {
    //!
    //! Module to decode and encode a hufman tree.
    //!

    ///
    /// Struct describing a node in a huffman tree.
    /// 
    #[derive(Copy, Clone, Debug)]
    #[derive(Default)]
struct Node {
        /// Symbol encoded in this node of the tree.
        symbol : u8,
        /// Is this node a leaf?
        is_leaf : bool,
        /// Index of the left subtree. If node is a leaf, this is -1.
        left : i64,
        /// Index of the right subtree. If node is a leaf, this is -1.
        right : i64
    }
    
    ///
    /// Decodes a byte from the data BitVec starting at offset.
    ///
    /// If the bitvector is shorter then offset + 7.
    ///
    fn decode_byte(data: & bit_vec::BitVec, offset: & mut usize) -> u8 {
        let mut value : u8 = 0;
        for i in (0..8).rev() {
            let v = if data[*offset] {
                1
            } else {
                0
            };
            *offset += 1;
            value |= v << i;
        }
        value
    }

    ///
    /// Decodes a node in the hufman tree.
    ///
    /// The decoding starts at index in the tree and starts at offset in the data.
    /// Every decoded node is added to the tree at position index.
    ///
    fn decode_node(data: & bit_vec::BitVec, offset: & mut usize, tree: & mut [Node], index: & mut usize) -> usize{
        let current = *index;
        *index += 1;

        let is_leaf = data[*offset];
        *offset += 1;

        if is_leaf {
            let value = decode_byte(data, offset);
            tree[current].left = -1;
            tree[current].right = -1;
            tree[current].is_leaf = true;
            tree[current].symbol = value;
        } else {
            tree[current].is_leaf = false;
            tree[current].left = decode_node(data, offset, tree, index) as i64;
            tree[current].right = decode_node(data, offset, tree, index) as i64;
        }

        current
    }

    ///
    /// Decodes a input array consisting of a hufmann tree at the beginning followed by the
    /// actual data.
    ///
    /// The output_size is the maximum size of the decoded output.
    ///
    pub fn decode(input: &[u8], output_size: usize) -> Vec<u8> {
        let bit_vec = bit_vec::BitVec::from_bytes(input);
        let mut tree : [Node; 511] = [Default::default(); 511];
        let mut index : usize = 0;
        let mut offset : usize= 0;
        
        decode_node(& bit_vec, & mut offset, & mut tree, & mut index);

        let mut left : usize = output_size ;
        let mut o : usize = 0;

        let mut output =vec![0; output_size];

        while left > 0 {
            let mut branch = tree[0];
            while !branch.is_leaf {
                let t = if ! bit_vec[offset] {
                    branch.left as usize
                } else {
                    branch.right as usize
                };
                offset += 1;
                branch = tree[t];
            }
            output[o] = branch.symbol;
            o += 1;
            left -= 1;
        }
        output
    }
}
