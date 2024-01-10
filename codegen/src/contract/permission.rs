use crate::contract::EnumDescriptor;
use crate::error::Error;
use crate::template::TemplateContext;
use log::info;
use std::fs::File;
use std::path::Path;

pub fn build_context(data_dir: &Path) -> Result<TemplateContext, Error> {
    let path = data_dir.join("permissions.json");
    info!("Context path is {:?}", path);

    let file = File::open(path)?;
    let permissions: Vec<EnumDescriptor> = serde_json::from_reader(file)?;

    let mut context = TemplateContext::with_capacity(1);
    context.insert("permissions".into(), handlebars::to_json(permissions));

    Ok(context)
}
