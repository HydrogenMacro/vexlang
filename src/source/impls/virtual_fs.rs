use std::{
    collections::HashMap,
    fmt::{Display, Write},
};

use anyhow::anyhow;

use crate::source::fs::{DirEntry, File, FileSystem};

#[derive(Debug, Clone)]
pub enum VirtualFileSystemItem {
    File {
        name: String,
        data: Vec<u8>,
    },
    Folder {
        name: String,
        contents: HashMap<String, VirtualFileSystemItem>,
    },
}
impl VirtualFileSystemItem {
    pub fn as_dir_entry(&self) -> DirEntry {
        match &self {
            VirtualFileSystemItem::File { name, data } => {
                DirEntry::File(File::new(name.clone(), data.clone()))
            }
            VirtualFileSystemItem::Folder { name, .. } => DirEntry::Folder { name: name.clone() },
        }
    }
    pub fn file(name: impl Into<String>, data: Vec<u8>) -> Self {
        let name = name.into();
        Self::File { name, data }
    }
    pub fn folder(name: impl Into<String>) -> Self {
        let name = name.into();
        Self::Folder {
            name,
            contents: HashMap::new(),
        }
    }
    pub fn folder_contents(
        &self,
        debug_path: &str,
    ) -> anyhow::Result<&HashMap<String, VirtualFileSystemItem>> {
        match self {
            VirtualFileSystemItem::Folder { contents, .. } => Ok(contents),
            VirtualFileSystemItem::File { .. } => {
                Err(anyhow!("{} is not a folder.", debug_path))
            }
        }
    }
	pub fn folder_contents_mut(
        &mut self,
        debug_path: &str,
    ) -> anyhow::Result<&mut HashMap<String, VirtualFileSystemItem>> {
        match self {
            VirtualFileSystemItem::Folder { contents, .. } => Ok(contents),
            VirtualFileSystemItem::File { .. } => {
                Err(anyhow!("{} is not a folder.", debug_path))
            }
        }
    }
}
pub struct VirtualFileSystemPath(Vec<String>);
impl VirtualFileSystemPath {
    fn new(path: &str) -> Self {
        Self(path.split("/").map(|p| p.to_owned()).collect())
    }
    fn file_name(&self) -> &str {
        &self.0[0]
    }
    fn parent_dirs(&self) -> &[String] {
        &self.0[..self.0.len().saturating_sub(1)]
    }
}
impl From<&str> for VirtualFileSystemPath {
    fn from(value: &str) -> Self {
        VirtualFileSystemPath::new(value)
    }
}
impl From<String> for VirtualFileSystemPath {
    fn from(value: String) -> Self {
        VirtualFileSystemPath::new(&value)
    }
}

impl Display for VirtualFileSystemPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .map(|p| f.write_str(p).and(f.write_char('/')))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct VirtualFileSystem {
    root: VirtualFileSystemItem,
}

impl VirtualFileSystem {
    pub fn new() -> Self {
        VirtualFileSystem {
            root: VirtualFileSystemItem::folder(""),
        }
    }
    pub fn get_item(
        &self,
        path: impl Into<VirtualFileSystemPath>,
    ) -> anyhow::Result<&VirtualFileSystemItem> {
        let path: VirtualFileSystemPath = path.into();
        let mut current_folder = &self.root;
        for (i, folder_name) in path.parent_dirs().iter().enumerate() {
            current_folder = current_folder
                .folder_contents(&path.to_string())?
                .get(folder_name)
                .ok_or_else(|| anyhow!("{} does not exist.", path.0[..=i].join("/")))?;
        }

        return current_folder.folder_contents(&path.to_string())?
            .get(path.file_name())
            .ok_or(anyhow!("{} not found.", path));
    }
    pub fn get_item_mut(
        &mut self,
        path: impl Into<VirtualFileSystemPath>,
    ) -> anyhow::Result<&mut VirtualFileSystemItem> {
        let path: VirtualFileSystemPath = path.into();
        let mut current_folder = &mut self.root;
        for (i, folder_name) in path.parent_dirs().iter().enumerate() {
            current_folder = current_folder
                .folder_contents_mut(&path.to_string())?
                .get_mut(folder_name)
                .ok_or_else(|| anyhow!("{} does not exist.", path.0[..=i].join("/")))?;
        }

        return current_folder.folder_contents_mut(&path.to_string())?
            .get_mut(path.file_name())
            .ok_or(anyhow!("{} not found.", path));
    }
}

impl FileSystem for VirtualFileSystem {
    fn read_file(&self, relative_file_path: &str) -> anyhow::Result<crate::source::fs::File> {
        if let VirtualFileSystemItem::File { name, data } = self.get_item(relative_file_path)? {
            let name = name.clone();
            let data = data.clone();
            return Ok(File { name, data });
        } else {
            return Err(anyhow!("{} does not lead to a file.", relative_file_path));
        }
    }

    fn write_file(&mut self, relative_file_path: &str, file_data: Vec<u8>) -> anyhow::Result<()> {
        let relative_file_path = VirtualFileSystemPath::new(relative_file_path);
        if let VirtualFileSystemItem::Folder { contents, .. } =
            self.get_item_mut(relative_file_path.parent_dirs().join("/"))?
        {
            contents.insert(
                relative_file_path.file_name().to_owned(),
                VirtualFileSystemItem::file(relative_file_path.file_name(), file_data),
            );
        }
        Ok(())
    }

    fn create_dir(&mut self, relative_dir_path: &str) -> anyhow::Result<()> {
        let path = VirtualFileSystemPath::new(relative_dir_path);
        let mut current_dir = &mut self.root;
        for dir in path.parent_dirs() {
			let current_dir_contents = current_dir.folder_contents_mut(relative_dir_path)?;
            current_dir = current_dir_contents.entry(dir.clone()).or_insert(VirtualFileSystemItem::folder(dir));
		}
        Ok(())
    }

    fn read_dir(&self, relative_dir_path: &str) -> anyhow::Result<Vec<DirEntry>> {
        Ok(self.get_item(relative_dir_path)?.folder_contents(relative_dir_path)?.values().map(|vfs_item| vfs_item.as_dir_entry()).collect())
    }

    fn remove_dir(&mut self, relative_dir_path: &str) -> anyhow::Result<()> {
		let path = VirtualFileSystemPath::new(relative_dir_path);
        self.get_item_mut(path.parent_dirs().join("/"))?.folder_contents_mut(relative_dir_path)?.remove(path.file_name());
		Ok(())
    }

    fn remove_file(&mut self, relative_file_path: &str) -> anyhow::Result<()> {
        let path = VirtualFileSystemPath::new(relative_file_path);
        self.get_item_mut(path.parent_dirs().join("/"))?.folder_contents_mut(relative_file_path)?.remove(path.file_name());
		Ok(())
    }
}
