use crate::error::Error;
use crate::template::TemplateContext;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::path::Path;

pub(super) fn for_enum<P>(path: P) -> Result<TemplateContext, Error>
where
    P: AsRef<Path>,
{
    build_simple_context::<Vec<EnumDescriptor>>("variants", path)
}

pub(super) fn for_struct<P>(path: P) -> Result<TemplateContext, Error>
where
    P: AsRef<Path>,
{
    build_simple_context::<Vec<FieldDescriptor>>("fields", path)
}

fn build_simple_context<T>(key: &str, path: impl AsRef<Path>) -> Result<TemplateContext, Error>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    let path = path.as_ref();
    info!("Context path is {path:?}");

    let file = File::open(path)?;
    let items: T = serde_json::from_reader(file)?;

    let mut context = TemplateContext::new();
    context.insert(key.into(), handlebars::to_json(items));

    Ok(context)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnumDescriptor {
    key: String,
    value: Value,
    description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FieldDescriptor {
    pub field: String,
    pub kind: String,

    #[serde(rename = "innerKind")]
    pub inner_kind: Option<String>,
}

#[derive(Debug)]
pub enum FieldKind<'a> {
    String,
    U32,
    Array(Option<&'a str>),
    Class(&'a str),
}

impl<'a> FieldKind<'a> {
    pub fn from_descriptor(descriptor: &'a FieldDescriptor) -> Self {
        let kind = descriptor.kind.to_ascii_lowercase();

        match kind.as_str() {
            "string" => Self::String,
            "u32" => Self::U32,
            "array" => Self::Array(descriptor.inner_kind.as_deref()),
            _ => Self::Class(descriptor.kind.as_str()),
        }
    }
}
