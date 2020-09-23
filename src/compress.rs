pub mod huffman {
    use std::boxed::Box;
    use std::cmp::Ordering;
    use std::cmp::Ord;
    use std::collections::*;

    ///	Node is a binary tree data structure.
	///	It will be used by huffman compression algorithm
    #[derive(Clone, PartialEq, Eq, Ord, std::fmt::Debug)]
    struct Node {
        letter: char,
        freq: i32,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    }
    impl Node {
        // create a leaf node
        fn new (letter:char, freq:i32)->Node{
            Node {
                letter,
                freq,
                left: None,
                right: None,
            }
        }
    }
    fn freq_count(text: std::str::Chars)->Vec<Node>{
        let mut freq_vec = Vec::new();
        let mut chars: Vec<char> = text.collect();
        chars.sort();
        let mut freq = 0;
        let mut prev: char = *chars.first().expect("Input cannot be empty");
        for c in chars {
            if c == prev {
                freq += 1;
            } else {
                freq_vec.push(Node::new(prev, freq));
                freq = 1;
                prev = c;
            }
        }
        freq_vec.push(Node::new(prev,freq));
        freq_vec
    }
    fn construct_huffman_tree(freq: Vec<Node>)->Node{
        let mut pq = BinaryHeap::new();
        for node in freq {
            pq.push(node);
        }
        while pq.len()>1 {
            let (a,b) = (pq.pop().unwrap,pq.pop().unwrap());
            let new_node = Node {
                letter: '\0',
                freq: a.freq + b.freq,
                left: Option::from(Box::from(a)),
                right: Option::from(Box::from(b)),
            };
            pq.push(new_node);
        }
        pq.pop().unwrap()
    }
    // convert huffman tree to hashmap with key as char and value as encoding 
    // E.g = 'a', value = '1000'
    fn to_hashmap(node:&Node)->HashMap<char, String> {
        let mut hm = HashMap::new();
        // Huffman tree is complete binary tree, a node will have either 0 or 2 children, 1 is not possible
        if node.left.is_none(){
            hm.insert(node.letter, "0".to_string());
            return hm;
        }
        fn encode(hm:&mut HashMap<char,String>, node:&Node, encoding: String){
            if node.left.is_none(){
                hm.insert(node.letter, encoding);
            } else {
                let left_path = String::from(&encoding)+"0";
                let right_path = String::from(&encoding)+"1";
                if let Some(left) = &node.left{
                    encode(hm,&left,left_path);
                }
                if let Some(right) = &node.right {
                    encode(hm,&right,right_path);
                }   
            }    
        };
        encode(&mut hm, &node, "".to_string());
        return hm;
    }
    // convert huffam node to string of chars using post-order traversal
    fn to_string(huffman_node: &Node)->String{
       let mut output = String::new();
       fn post_order(node:&Node, output_str:&mut String){
           if let Some(left) = &node.left {
               post_order(left.as_ref(), output_str);
           }
           if let Some(right) = &node.right {
                post_order(right.as_ref(), output_str);
           }
           output_str.push(node.letter);
        }
    }
    // Convert huffman tree to vector of bytes
	//
	// First element is length of tree
	//
	// There are only 100 or so printable characters 
	// based on python's string.printable
	// So worst case tree size is 2N-1 = 199
	// So a unsigned char will suffice for length of tree
	//
	// Following elements are charectars in post-order traversal of tree
    fn embed_tree(huffman_node: &Node)->Vec<u8>{
        let mut compressed_data = to_string(huffman_node).into_bytes();
        compressed_data.insert(0, compressed_data.len() as u8);
        compressed_data
    }
    // Simply maps input characters to their corresponding encoding and return as byte array
	//
	// The first element is padding, (Number of zeroes appended for last encoding), as encoding might not fit into 8 bits
    fn compress_data(text: &String, huffman_node:&Node)->Vec<u8>{
        let mut byte_stream: Vec<u8> = Vec::new();
        let (mut byte, mut count) = (0,0);

        let huffman_map = to_hashmap(huffman_node);
        for c in text.chars(){
            let encoding = huffman_map.get(&c).unwrap;
            for e in encoding.bytes(){
                let bit: bool = (e - '0' as u8) != 0;
                byte = byte << 1 | (bit as u8);
                count = (count +1) % 8;
                if count == 0 {
                    byte_stream.push(byte);
                    byte = 0;
                }
            }
        }
        if count != 0 {
            let padding:u8 = 8 - count;
            byte <<= padding;
            byte_stream.push(byte);
            byte_stream.insert(0, padding);
        } else {
            byte_stream.insert(0, 0);
        }
        byte_stream
    }
    // Compression using huffman's algorithm
	// # Data Format
	// First byte (n): Length of post-order traversal of huffman tree
	//     Following n bytes contain post-order traversal
	// Padding byte (p): Padding for final byte
    // All remaining bytes are data
    pub fn compress(text: &String) -> Vec<u8> {
        let frequency = freq_count(text.chars());
        let huffman_tree = construct_huffman_tree(frequency);
        let mut compressed_data = Vec::from(embed_tree(&huffman_tree));
        compressed_data.extend(compress_data(text, &huffman_tree));
        compressed_data
    }
    fn construct_tree_from_postorder(postorder: &[u8])->Node {
        let mut stack = Vec::new();
        for c in postorder {
            if *c ==0 as u8 {
                let (left, right) = (
                    stack.pop().expect("Input Contains Null byte "),
                    stack.pop().expect("Input Contains Null byte "),
                );
                stack.push(Node{
                    letter: '\0',
                    freq: 0,
                    left: Option::from(Box::from(right)),
                    right: Option::from(Box::from(left)),
                });
            } else {
                stack.push(Node{
                    letter: *c as char,
                    freq: 0,
                    left: None,
                    right: None,
                });
            }
        }
        stack.pop().unwrap();
    }

    fn decompress_data(){}
    pub fn decompress(){}
}