pub mod huffman {
    use std::boxed::Box;
    use std::cmp::Ordering;
    use std::collections::*;

    ///	Node is a binary tree data structure.
	///	It will be used by huffman compression algorithm
    #[derive(Clone, PartialEq, Eq, std::fmt::Debug)]
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
    fn freq_count(text: std::str::Char)->Vec<Node>{
        let mut freq_vec = Vec::new();
        let mut chars: Vec<char> = text.collect();
        chars.sort();
        let mut freq = 0;
        let mut prev: char = *char.first().expect("Input cannot be empty");
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
    fn construct_huffman_tree(freq: Vec<node>)->Node{
        let mut pq = BinaryHeap::new();
        for node in freq_vec {
            pq.push(node);
        }
        while pq.len()>1 {
            let (a,b) = (pq.pop().unwrap,pq.pop().unwrap());
            let new_node = Node {
                letter = '\0',
                freq = a.freq + b.freq,
                left: Option::from(Box::from(a)),
                right: Option::from(Box::from(b)),
            }
            pq.push(new_node);
        }
        pq.pop().unwrap()
    }
    // convert huffman tree to hashmap with key as char and value as encoding 
    // E.g = 'a', value = '1000'
    fn to_hashmap(node:&node)->HashMap<char, String> {
        
    }
}