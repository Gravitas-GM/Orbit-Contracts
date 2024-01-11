use crate::contract::Contract;
use crate::error::Error;
use crate::lang::Language;
use handlebars::Handlebars;
use log::{debug, info};
use serde_json::{Map, Value};
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

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

pub fn build<'a, C, L>(
    contracts: C,
    languages: L,
    config: Config,
) -> Result<(), Error>
where
    C: IntoIterator<Item = &'a Contract> + Copy,
    L: IntoIterator<Item = &'a Language> + Copy,
{
    debug!("{:?}", config);

    for contract in contracts {
        for lang in languages {
            build_contract(contract, lang, &config)?;
        }
    }

    Ok(())
}

fn build_contract(contract: &Contract, language: &Language, config: &Config) -> Result<(), Error> {
    info!("Building {:?} for {:?}", contract, language);

    let template_path = language.get_template_path(contract);

    let input_path = config.get_template_dir().join(template_path);
    info!("Input path is {:?}", input_path);

    let input = fs::read_to_string(input_path)?;
    let output_file = create_output_file(template_path, config.get_output_dir())?;

    let handlebars = Handlebars::new();

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

fn to_output_path(template_path: &Path) -> Option<PathBuf> {
    let stem = template_path.file_stem()?;
    let parent = template_path.parent()?;

    Some(Path::new(parent).join("src/").join(stem))
}
