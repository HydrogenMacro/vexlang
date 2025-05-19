pub struct Span {
	start_idx: usize,
	end_idx: usize,
	start_row: usize,
	start_col: usize,
	end_row: usize,
	end_col: usize,
	source_file_name: String
}
impl Span {
	fn from_idx(idx: usize, source: &str) {

	}
	fn from_range(start_idx: usize, end_idx: usize, source: &str) {
		
	}
}