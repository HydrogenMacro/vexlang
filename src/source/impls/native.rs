use anyhow::anyhow;

use crate::source::fs::{DirEntry, File, FileSystem};

pub struct NativeFileSystem;

impl FileSystem for NativeFileSystem {
    fn read_file(&self, relative_file_name: &str) -> anyhow::Result<crate::source::fs::File> {
        let contents = std::fs::read(relative_file_name)?;
        Ok(File {
            name: relative_file_name.into(),
            data: contents,
        })
    }

    fn write_file(
        &mut self,
        relative_file_name: &str,
        file_data: Vec<u8>,
    ) -> anyhow::Result<()> {
        std::fs::write(relative_file_name, file_data)?;
        Ok(())
    }

    fn create_dir(&mut self, relative_dir_path: &str) -> anyhow::Result<()> {
        std::fs::create_dir_all(relative_dir_path)?;
        Ok(())
    }

    fn read_dir(&self, relative_dir_path: &str) -> anyhow::Result<Vec<DirEntry>> {
        std::fs::read_dir(relative_dir_path)?
            .map(|dir_entry| {
                let dir_entry = dir_entry?;
				let file_type = dir_entry.file_type()?;
                let file_name = dir_entry.file_name().into_string().map_err(|file_name| {
                    anyhow::anyhow!(
                        "Could not parse {:?} at {:?} to a utf8 string.",
                        file_name,
                        relative_dir_path
                    )
                })?;
				if file_type.is_dir() {
					return Ok(DirEntry::Folder { name: file_name })
				} else if file_type.is_file() {
					return Ok(DirEntry::File(self.read_file(&file_name)?));
				} else {
					return Err(anyhow!("{} is a symlink, which is currently not supported.", relative_dir_path));
				}
            })
            .collect()
    }
	
	fn remove_dir(&mut self, relative_dir_path: &str) -> anyhow::Result<()> {
		std::fs::remove_dir_all(relative_dir_path)?;
		Ok(())
	}
	
	fn remove_file(&mut self, relative_file_path: &str) -> anyhow::Result<()> {
		std::fs::remove_file(relative_file_path)?;
		Ok(())
	}
	
}
