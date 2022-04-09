pub mod markov {
	pub struct Link<'a> {
		pub value: u32,
		node: &'a Node<'a>
	}
	
	pub struct Node<'a> {
		pub value: &'a str,
		links: Vec<Link<'a>>
	}
	
	pub trait MarkovNode {
		
	}

	impl Link<'_> {
		pub fn strengthen(&mut self) -> u32 {
			self.value += 1;

			self.value
		}
	}

	impl<'a> Node<'a> {
		fn search_links(&mut self, search: &str) -> Option<&'a mut Link> {
			for i in self.links.iter_mut() {
				if i.node.value.eq_ignore_ascii_case(search) {
					return Some(i)
				}
			}

			None
		}

		fn find_link(&mut self, n: &'a Node<'a>) -> Option<&'a mut Link> {
			for i in self.links.iter_mut() {
				if std::ptr::eq(i.node, n) {
					return Some(i)
				}
			}

			None
		}

		pub fn add_new_link(&mut self, n: &'a Node<'a>) {
			self.links.push(Link { value: 1, node: n });
		}

		pub fn strengthen_or_link(&'a mut self, n: &'a Node<'a>) {
			match self.find_link(n) {
				Some(l) => {
					l.strengthen();
				}
				None => {}
			}
		}
	}
}
