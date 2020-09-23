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
        
    }
}