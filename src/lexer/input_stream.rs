use unicode_segmentation::{UnicodeSegmentation, Graphemes};

pub struct InputStream {
	input_bytes: Vec<u8>,
	current_idx: usize,
}

impl InputStream {
	pub fn new(input_string: String) -> InputStream {
		InputStream {
			input_bytes: input_string.into_bytes(),
			current_idx: 0
		}
	}
	pub fn peek_byte(&self) -> Option<u8> {
		(!self.at_end()).then_some(self.input_bytes[self.current_idx + 1])
	}
	pub fn peek_str(&self, peek_len: usize) -> Option<&str> {
		if self.current_idx + peek_len >= self.input_bytes.len() {
			return None;
		} 
		std::str::from_utf8(&self.input_bytes[self.current_idx..(self.current_idx + peek_len)]).ok()
	}
	pub fn matches_byte(&self, byte_to_match: u8) -> bool {
		self.peek_byte().map_or(false, |peeked_byte| peeked_byte == byte_to_match)
	}
	pub fn matches_str(&self, str_to_match: &str) -> bool {
		self.peek_str(str_to_match.len()).map_or(false, |peeked_str| peeked_str == str_to_match)
	}
	pub fn advance(&mut self, advance_amt: usize) {
		self.current_idx += advance_amt;
	}
	pub fn advance_byte_if(&mut self, f: impl Fn(u8) -> bool) -> Option<u8> {
		self.peek_byte().and_then(|byte| f(byte).then_some(byte))
	}
	pub fn advance_if_matches_str(&mut self, str_to_match: &str) -> bool {
		let matches_str = self.matches_str(str_to_match);
		if matches_str {
			self.advance(str_to_match.len());
		}
		matches_str
	}
	pub fn at_end(&self) -> bool {
		self.current_idx > self.input_bytes.len() - 1
	}
}
