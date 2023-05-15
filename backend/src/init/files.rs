use std::path::Path;

pub static EXTERNAL_FOLDER: &str = "./external_files";
pub static SHIPPED_FOLDER: &str = "./shipped_plugins";
pub static EXTERNAL_PLUGIN_FOLDER: &str = "./external_files/plugins";

pub static ENV_EXAMPLE_FILENAME: &str = "./.env.example";


/*due to how docker works, the external_folder that can be mapped to a local file, cannot be filled on startup, otherwise, the host folder will overlay the container folder
 => needs to be empty first and when started, we copy the content from another location in the external folder and make the content therefore also available on the docker host
*/
pub fn copy_files_into_external_folder() -> std::io::Result<()> {
    if !Path::new(EXTERNAL_PLUGIN_FOLDER).exists() {
        let src = Path::new(SHIPPED_FOLDER);
        let dst = Path::new(EXTERNAL_FOLDER);
        copy_dir_all(src, dst)?;
    }
    let env_file_path = Path::new(super::ENV_FILENAME);

    if !env_file_path.exists() {
        std::fs::copy(Path::new(ENV_EXAMPLE_FILENAME), env_file_path)?;
    }

    Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}