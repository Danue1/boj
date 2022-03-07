use serde::Deserialize;
use std::fs::{create_dir_all, read_to_string, File};
use std::path::Path;
use std::process::Command;

fn main() {
    if let Err(error) = build() {
        dbg!(error);
    }
}

fn build() -> Result<(), Error> {
    use std::io::Write;

    let config = read_to_string("Cargo.toml")?;
    let RootConfig { workspace } = toml::from_str(&config)?;
    let mut command = Command::new("rustfmt");

    for member in workspace.members {
        let source_path = Path::new(&member).join("src");
        create_dir_all(&source_path)?;

        let main_path = source_path.join("main.rs");
        command.args(&main_path);

        let mut file = File::create(&main_path)?;
        write!(file, "{}", build_solver(&source_path.join("lib.rs"))?)?;
    }
    command.spawn()?.wait()?;

    Ok(())
}

fn build_solver(path: &Path) -> Result<String, Error> {
    Ok(format!(
        r#"fn main() {{
    use std::io::Read;

    let mut io = std::io::stdin();
    let mut input = String::new();
    match io.read_to_string(&mut input) {{
        Ok(_) => match solve(&input) {{
            Ok(output) => print!("{{}}", output),
            Err(error) => print!("{{:#?}}", error),
        }}
        Err(error) => print!("{{:#?}}", error),
    }}
}}

{solver}"#,
        solver = read_to_string(path)?,
    ))
}

#[derive(Debug, Deserialize)]
struct RootConfig {
    workspace: Workspace,
}

#[derive(Debug, Deserialize)]
struct Workspace {
    members: Vec<String>,
}

#[derive(Debug)]
enum Error {
    Io(std::io::Error),
    Toml(toml::de::Error),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self {
        Error::Toml(error)
    }
}
