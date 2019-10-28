extern crate bit_vec;

pub mod hufman {
    //!
    //! Module to decode and encode a huffman tree encoded data.
    //!

    use super::bit_vec::BitVec;

    ///
    /// Struct describing a node in a huffman tree.
    ///
    #[derive(Copy, Clone, Debug)]
    struct Node {
        /// Symbol encoded in this node of the tree.
        symbol: u8,
        /// Is this node a leaf?
        is_leaf: bool,
        /// Index of the left subtree. If node is a leaf, this is -1.
        left: i64,
        /// Index of the right subtree. If node is a leaf, this is -1.
        right: i64,
    }

    impl Default for Node {
        fn default() -> Node {
            Node { symbol: 0, is_leaf: false, right: 0, left: 0 }
        }
    }

    ///
    /// Decodes a byte from the data BitVec starting at offset.
    ///
    /// # Errors
    ///
    /// If the bitvector is shorter then offset + 7 the system will panic.
    ///
    /// # Arguments
    ///
    /// * `data` -  The bit vector the byte should be decoded from. All indices smaller than
    ///             offset +7 have to be valid.
    /// * `offset`- The current offset in the bit vector.
    ///
    /// # Return values
    ///
    /// * The decoded byte from the bit vector.
    ///
    fn decode_byte(data: &bit_vec::BitVec<u32>, offset: &mut usize) -> u8 {
        let mut value: u8 = 0;
        for i in (0..8).rev() {
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

    ///
    /// Decodes a huffman (sub) tree from a bitvector.
    ///
    /// The decoding starts at index in the tree and starts at offset in the data.
    /// Every decoded node is added to the tree at position index.
    ///
    /// # Arguments
    ///
    /// * `data` - The data containing the huffman tree followed by the encoded data.
    /// * `offset` - The current offset in the bit vector.
    /// * `tree` - The tree structure as a array of nodes.
    /// * `index` - The current index in the tree. The next decoded tree node will be placed here.
    ///
    /// # Return Values
    ///
    /// * The next index for the next decoded subtree.
    ///
    fn decode_huffman_tree(data: &bit_vec::BitVec, offset: &mut usize, tree: &mut [Node], index: &mut usize) -> usize {
        let current = *index;
        *index += 1;

        let is_leaf = data[*offset];
        *offset += 1;

        if is_leaf {
            let value = decode_byte(&data, offset);
            tree[current].left = -1;
            tree[current].right = -1;
            tree[current].is_leaf = true;
            tree[current].symbol = value;
        } else {
            tree[current].is_leaf = false;
            tree[current].left = decode_huffman_tree(&data, offset, tree, index) as i64;
            tree[current].right = decode_huffman_tree(&data, offset, tree, index) as i64;
        }

        return current;
    }

    ///
    /// Function to decode data from a bit array given a fixed offset in the bit array
    /// and a fixed huffman encoding tree.
    ///
    /// The output size defines the size of the decoded data.
    /// This is used to avoid costly reallocation during encoding.
    /// Only output_size number of bytes will be decoded.
    ///
    /// # Arguments
    ///
    /// * `data` - The data containing the huffman tree followed by the encoded data.
    /// * `offset` - The current offset in the bit vector.
    /// * `tree` - The tree structure as a array of nodes.
    /// * `output_size` -   The exact size of the decoded data.
    ///
    /// # Return Values
    ///
    /// * Decoded data from the input. Is exact output_size long.
    ///
    fn decode_huffman_data(data: &bit_vec::BitVec<u32>, offset: &mut usize, tree: &[Node], output_size: usize) -> Vec<u8> {
        let mut left: usize = output_size;
        let mut o: usize = 0;

        let mut output = vec![0; output_size];

        while left > 0 {
            let mut branch = tree[0];
            while !branch.is_leaf {
                let t = if !data[(*offset)] {
                    branch.left as usize
                } else {
                    branch.right as usize
                };
                *offset += 1;
                branch = tree[t];
            }
            output[o] = branch.symbol;
            o += 1;
            left -= 1;
        }
        return output;
    }

    ///
    /// Decodes a input array consisting of a hufmann tree at the beginning followed by the
    /// actual data.
    ///
    /// The output_size is the maximum size of the decoded output.
    ///
    /// # Arguments
    /// * `input` - Byte array containing the encoded huffman tree followed by the encoded
    ///             data.
    /// * `output_size` - Size of the decoded data. This exact number of bytes will be decoded.
    ///                   If this is not the exact size of the decoded data the system will
    ///                   panic.
    ///
    /// # Return Values
    ///
    /// * Decoded data from the input. Is exact output_size long.
    ///
    pub fn decode(input: &[u8], output_size: usize) -> Vec<u8> {
        let bit_vec: BitVec<u32> = bit_vec::BitVec::from_bytes(input);
        let mut tree: [Node; 511] = [Default::default(); 511];
        let mut index: usize = 0;
        let mut offset: usize = 0;

        decode_huffman_tree(&bit_vec, &mut offset, &mut tree, &mut index);
        return decode_huffman_data(&bit_vec, &mut offset, &tree, output_size);
    }
}