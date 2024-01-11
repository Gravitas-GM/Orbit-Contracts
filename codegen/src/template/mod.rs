use crate::contract::Contract;
use crate::error::Error;
use crate::lang::Language;
use handlebars::Handlebars;
use log::{debug, info};
use serde_json::{Map, Value};
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

mod helpers;

pub type TemplateContext = Map<String, Value>;

#[derive(Debug, Default)]
pub struct Config {
    pub data_dir: Option<PathBuf>,
    pub template_dir: Option<PathBuf>,
    pub output_dir: Option<PathBuf>,
}

impl Config {
    fn get_data_dir(&self) -> &Path {
        self.data_dir.as_deref().unwrap_or(Path::new("data/"))
    }

    fn get_template_dir(&self) -> &Path {
        self.template_dir
            .as_deref()
            .unwrap_or(Path::new("templates/"))
    }

    fn get_output_dir(&self) -> &Path {
        self.output_dir.as_deref().unwrap_or(Path::new("packages/"))
    }
}

pub fn build<'a, C, L>(contracts: C, languages: L, config: Config) -> Result<(), Error>
where
    C: IntoIterator<Item = &'a Contract> + Copy,
    L: IntoIterator<Item = &'a Language> + Copy,
{
    debug!("{:?}", config);

    let mut handlebars = Handlebars::new();
    helpers::register(&mut handlebars);

    for contract in contracts {
        for lang in languages {
            build_contract(&handlebars, contract, lang, &config)?;
        }
    }

    Ok(())
}

fn build_contract(
    handlebars: &Handlebars,
    contract: &Contract,
    language: &Language,
    config: &Config,
) -> Result<(), Error> {
    info!("Building {:?} for {:?}", contract, language);

    let Some(template_path) = language.get_template_path(contract) else {
        info!("Contract {contract:?} is not supported for {language:?}");
        return Ok(());
    };

    let input_path = config.get_template_dir().join(template_path);
    info!("Input path is {:?}", input_path);

    let input = fs::read_to_string(input_path)?;
    let output_file = create_output_file(template_path, config.get_output_dir())?;

    let context = contract.build_context(config.get_data_dir())?;
    handlebars.render_template_to_write(&input, &context, output_file)?;

    Ok(())
}

fn create_output_file(template_path: &'static Path, output_dir: &Path) -> Result<File, Error> {
    let path = to_output_path(template_path).ok_or_else(|| Error::PathConversion(template_path))?;
    let path = output_dir.join(path);
    info!("Output path is {:?}", path);

    Ok(File::create(path)?)
}

/// Template paths are in the format "<lang>/<template-dir?>/<template>.<extension>.handlebars".
/// Output paths should be in the format "<lang>/src/<template-dir?>/<template>.<extension>".
///
/// To convert that to an output path, we need to take the following steps:
///     1) Extract the "<lang>" component from the path, which should always be the path's first element
///     2) Collect the remaining elements into the path's tail, which consists of the "<template-dir>" (if there is one)
///        and the "<template>.<extension>.handlebars" compound component.
///     3) Extract the stem and its parent path from the tail.
///     4) Join the root element, the literal path "src/", the parent component, and the stem component to form the
///        complete output path
fn to_output_path(template_path: &Path) -> Option<PathBuf> {
    let mut components = template_path.components();
    let root: PathBuf = components.next()?.as_os_str().into();
    let tail: PathBuf = components.collect();

    let stem = tail.file_stem()?;
    let parent = tail.parent()?;

    Some(root.join("src/").join(parent).join(stem))
}
