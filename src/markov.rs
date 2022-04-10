use petgraph::{graphmap::DiGraphMap};
use indexmap::set::IndexSet;
use rand::{Rng, prelude::ThreadRng};

pub struct MarkovBot {
	graph: DiGraphMap<usize, u32>,
	words: IndexSet<String>,
	rng: ThreadRng
}

impl MarkovBot {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn add_link(&mut self, a: &str, b: &str) -> u32 {
		let (id_a, _) = self.words.insert_full(a.to_owned());
		let (id_b, _) = self.words.insert_full(b.to_owned());

		let weight = self.graph.edge_weight(id_a, id_b).unwrap_or(&0) + 1;
		self.graph.add_edge(id_a, id_b, weight);

		weight
	}

	pub fn chain(&mut self, word: &String) -> String {
		match self.words.get_index_of(word) {
			Some(id) => {
				let edges = self.graph.edges(id).collect::<Vec<_>>();
				let mut t = 0;
				let mut w = Vec::<u32>::with_capacity(edges.len());
				for (_, _, wt) in &edges {
					t += **wt;
					w.push(t);
				}

				let s = self.rng.gen_range(0..(t));
				// println!("{} {}", t, s);
				let ind = w.into_iter().position(|wt| wt > s).unwrap_or(0);
				match edges.get(ind) {
					Some((_, b, _)) => self.words.get_index(*b).unwrap_or(&"".to_string()).to_string(),
					None => "".to_string(),
				}
			},
			None => return "".to_string(),
		}
	}

	pub fn ingest(&mut self, sentence: &String) {
		let split: Vec<&str> = sentence.trim().split(" ").filter(|word| word.len() > 0).collect();
		let mut curr = "";
		for word in &split {
			self.add_link(curr, word);
			curr = *word;
		}
		self.add_link(curr, "");
	}

	pub fn generate(&mut self, max_words: usize) -> String {
		let mut words: Vec<String> = Vec::with_capacity(max_words);
		let mut i = 0;
		loop {
			let w = self.chain(words.last().unwrap_or(&"".to_string()));
			if w == "" || i >= max_words {
				break;
			}
			words.push(w);
			i += 1;
		}

		words.join(" ")
	}
}

impl Default for MarkovBot {
    fn default() -> Self {
        let mut s = Self { graph: Default::default(), words: IndexSet::with_capacity(1), rng: rand::thread_rng() };
		s.words.insert("".to_string());
		s
    }
}