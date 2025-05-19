use vexlang::source::impls::virtual_fs::VirtualFileSystem;
use vexlang::source::fs::FileSystem;

#[test]
fn a() -> anyhow::Result<()> {
	let mut vfs = VirtualFileSystem::new();
	vfs.create_dir("abc")?;
	vfs.create_dir("abc/def")?;
	vfs.create_dir("abc/def/ghi")?;
	vfs.write_file("abc/def/ghi/jkl.file", vec![0, 1, 2])?;
	vfs.write_file("abc/def/ghi/mno.file", vec![0, 1, 2])?;
	vfs.write_file("abc/def/ghi.file", vec![3, 4, 5])?;
	
	dbg!(vfs);
	Ok(())
}