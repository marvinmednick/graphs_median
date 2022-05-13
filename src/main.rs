use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;

extern crate minheap;
use minheap::MinHeap;
mod cmd_line;
use crate::cmd_line::CommandArgs;


#[derive(Debug)]
struct Upper {
    heap: MinHeap<i32>,
    next_id: u32
}

impl Upper {

    pub fn new() -> Self {
        Upper { heap: MinHeap::<i32>::new(), next_id: 1 }
    }

    pub fn add(&mut self,value: i32) {
        self.heap.insert(self.next_id,-value);
        self.next_id += 1;

    }

    pub fn remove(&mut self)  -> Option<i32>{
        if let Some(x) = self.heap.get_min() {
            Some(-x)
        }
        else {
            None
        }
    }

    pub fn peek(&mut self) -> Option<i32> {
        self.heap.peek_min()
    }
}

#[derive(Debug)]
struct Lower {
    heap: MinHeap<i32>,
    next_id: u32
}

impl Lower {

    pub fn new() -> Self {
        Lower { heap: MinHeap::<i32>::new(), next_id: 1 }
    }

    pub fn add(&mut self,value: i32) {
        self.heap.insert(self.next_id,value);
        self.next_id += 1;
    }

    pub fn remove(&mut self)  -> Option<i32> {
        self.heap.get_min()
    }

    pub fn peek(&mut self) -> Option<i32> {
        self.heap.peek_min()
    }
}

#[derive(Debug)]
struct MedianHeap {
    upper:  Upper,
    lower:  Lower,
    sum:    i32,
    count:  u32,

}

impl MedianHeap {

    pub fn new() -> Self {
        MedianHeap { upper: Upper::new(), lower: Lower::new(), sum: 0, count: 0 }
    }

    pub fn add_value(&mut self,value: i32) {

    }

    pub fn get_median(self) -> Option<i32> {
        None

    }
}

fn main() {


    let cmd_line = CommandArgs::new();

    println!("Hello, {:?}!",cmd_line);

    println!("Calulating median");
  // Create a path to the desired file
    let path = Path::new(&cmd_line.filename);
    let display = path.display();


    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    let mut h = MedianHeap::new();

	let mut _count = 0;
    for line in reader.lines() {
		_count += 1;	
		let line_data = line.unwrap();

        // find the the input number, ignoring leading whitespace 
        let re_input = Regex::new(r"^\s*(?P<number>\d+).*$").unwrap();

        if let Some(x) = re_input.captures(&line_data) {
               let match_numstr = x.name("number").map_or("", |m| m.as_str());
               let num = match_numstr.parse::<i32>().unwrap();
               println!("Found {}",num);
               h.add_value(num);
        } 
        else {
            println!("Skipping line #{} {}",_count,line_data)
        };

    }


}


/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */
/*
// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;

	fn setup_basic1() -> Graph {
		let mut g = Graph::new();
		assert_eq!(g.add_edge(1,2),Some(1));
		assert_eq!(g.add_edge(1,3),Some(2));
		assert_eq!(g.add_edge(2,3),Some(1));
		assert_eq!(g.add_edge(2,4),Some(2));
		assert_eq!(g.add_edge(3,4),Some(1));
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.get_outgoing(2),&[3,4]);
		assert_eq!(g.get_outgoing(3),&[4]);
		assert_eq!(g.get_outgoing(4),&[]);
		g
	} 

    #[test]
    fn basic() {
		let mut g = Graph::new();
		assert_eq!(g.create_vertex(&1),Some(1));
		assert_eq!(g.create_vertex(&2),Some(2));
		assert_eq!(g.add_edge(1,2),Some(1));
		assert_eq!(g.get_vertexes(),vec!(1,2));
		assert_eq!(g.create_vertex(&3),Some(3));
		assert_eq!(g.add_edge(1,3),Some(2));
		assert_eq!(g.add_edge(2,3),Some(1));
		assert_eq!(g.get_vertexes(),vec!(1,2,3));
		assert_eq!(g.add_edge(1,4),Some(3));
		assert_eq!(g.get_vertexes(),vec!(1,2,3,4));
		println!("{:?}",g);

    }

	#[test]
	fn test_add() {
		let mut g = Graph::new();
		assert_eq!(g.add_edge(1,2),Some(1));
		assert_eq!(g.get_outgoing(1),&[2]);
		assert_eq!(g.get_incoming(2),&[1]);
		assert_eq!(g.add_edge(1,3),Some(2));
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.get_incoming(2),&[1]);
	}

	#[test]
	fn test_add_del() {
		let mut g = setup_basic1();
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.add_edge(1,2),Some(3));
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.get_outgoing(2),&[3,4]);
		assert_eq!(g.get_outgoing(3),&[4]);
		assert_eq!(g.delete_edge(1,2),Ok(()));
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.delete_edge(1,2),Ok(()));
		assert_eq!(g.get_outgoing(1),&[3]);
		
	}


 }
 */
