use crate::contract::FieldDescriptor;
use crate::error::Error;
use crate::template::TemplateContext;
use log::info;
use std::fs::File;
use std::path::Path;

pub fn build_context(data: &Path) -> Result<TemplateContext, Error> {
    let path = data.join("clients/hub/models/hub_user.json");
    info!("Context path is {:?}", path);

    let file = File::open(path)?;
    let fields: Vec<FieldDescriptor> = serde_json::from_reader(file)?;

    let mut context = TemplateContext::new();
    context.insert("fields".into(), handlebars::to_json(fields));

    Ok(context)
}
