use petgraph::{graphmap::DiGraphMap};
use indexmap::set::IndexSet;
use rand::{Rng, prelude::ThreadRng};

pub struct MarkovBot<'a> {
	graph: DiGraphMap<usize, u32>,
	words: IndexSet<&'a str>,
	rng: ThreadRng
}

impl<'a> MarkovBot<'a> {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn add_link(&mut self, a: &'a str, b: &'a str) -> u32 {
		let (id_a, _) = self.words.insert_full(a);
		let (id_b, _) = self.words.insert_full(b);

		let weight = self.graph.edge_weight(id_a, id_b).unwrap_or(&0) + 1;
		self.graph.add_edge(id_a, id_b, weight);

		weight
	}

	pub fn chain(&mut self, word: &str) -> &'a str {
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
					Some((_, b, _)) => *self.words.get_index(*b).unwrap_or(&""),
					None => "",
				}
			},
			None => return "",
		}
	}

	pub fn ingest(&mut self, sentence: &'a str) {
		let split: Vec<&str> = sentence.trim().split(" ").filter(|word| word.len() > 0).collect();
		let mut curr = "";
		for word in &split {
			self.add_link(curr, word);
			curr = *word;
		}
		self.add_link(curr, "");
	}

	pub fn generate(&mut self, max_words: usize) -> String {
		let mut words: Vec<&str> = Vec::with_capacity(max_words);
		let mut i = 0;
		let mut w = "";
		loop {
			w = self.chain(w);
			if w == "" || i >= max_words {
				break;
			}
			words.push(w);
			i += 1;
		}

		words.join(" ")
	}
}

impl Default for MarkovBot<'_> {
    fn default() -> Self {
        let mut s = Self { graph: Default::default(), words: IndexSet::with_capacity(1), rng: rand::thread_rng() };
		s.words.insert("");
		s
    }
}