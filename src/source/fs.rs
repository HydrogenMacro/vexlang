pub trait FileSystem {
    
    fn read_file(&self, relative_file_path: &str) -> anyhow::Result<File>;
    fn write_file(
        &mut self,
        relative_file_path: &str,
        file_data: Vec<u8>,
    ) -> anyhow::Result<()>;
    fn create_dir(&mut self, relative_dir_path: &str) -> anyhow::Result<()>;
    fn read_dir(&self, relative_dir_path: &str) -> anyhow::Result<Vec<DirEntry>>;
	fn remove_dir(&mut self, relative_dir_path: &str) -> anyhow::Result<()>;
	fn remove_file(&mut self, relative_file_path: &str) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub data: Vec<u8>,
}
impl File {
	pub fn new(name: String, data: Vec<u8>) -> Self {
		File { name, data }
	}
}

#[derive(Debug)]
pub enum DirEntry {
	File(File),
	Folder { name: String }
}