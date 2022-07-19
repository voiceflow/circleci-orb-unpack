use anyhow::{bail, Result};
use std::{
    fs::{self, create_dir_all},
    path::{Path, PathBuf},
};
use yaml_rust::{yaml::Hash, Yaml, YamlEmitter, YamlLoader};

pub fn unpack_from_file<S: AsRef<Path>, D: AsRef<Path>>(source: S, dest: D) -> Result<()> {
    let orb = get_orb_from_path(source)?;
    let orb_root = unpack_to_dir(orb, &dest)?;
    write_root_file(orb_root, dest)
}

/// Unpacks the relevant sections into separate files and directories
/// Returns the remaining YAML after those sections are removed
fn unpack_to_dir<D: AsRef<Path>>(mut orb: Hash, dest: D) -> Result<Hash> {
    // Write sections to subdirectories
    for section_name in ["commands", "jobs", "executors", "examples"] {
        if let Some(Yaml::Hash(section)) = orb.remove(&Yaml::from_str(section_name)) {
            extract_to_subdirectory(section, dest.as_ref().join(section_name))?;
        }
    }
    Ok(orb)
}

fn extract_to_subdirectory(section: Hash, dest: PathBuf) -> Result<()> {
    // Ensure that target subdirectory exists
    create_dir_all(&dest)?;

    for (key, value) in section.iter() {
        if let Yaml::String(key) = key {
            let file_name = format!("{key}.yml");
            let file_path = dest.join(file_name);
            write_yaml_to_file(file_path, value)?;
        } else {
            bail!("all keys must be strings")
        }
    }
    Ok(())
}

fn write_root_file<D: AsRef<Path>>(root: Hash, dest: D) -> Result<()> {
    let root_path = dest.as_ref().join("@orb.yml");
    write_yaml_to_file(root_path, &Yaml::Hash(root))
}

fn write_yaml_to_file(path: PathBuf, value: &Yaml) -> Result<()> {
    let mut output = String::new();
    let mut emitter = YamlEmitter::new(&mut output);
    emitter.dump(value)?;
    fs::write(path, &output[4..])?; // Skip the first three characters because the emitter writes --- at the start
    Ok(())
}

fn get_orb_from_path<S: AsRef<Path>>(path: S) -> Result<Hash> {
    let file_contents = fs::read_to_string(path)?;
    let yaml_documents = YamlLoader::load_from_str(&file_contents)?;

    match &yaml_documents[..] {
        [Yaml::Hash(orb)] => Ok(orb.clone()),
        _ => bail!(
            "expected 1 document in file, but found {}",
            yaml_documents.len()
        ),
    }
}
