use crate::contract::context::{FieldDescriptor, FieldKind};
use handlebars::{
    Context, Handlebars, Helper, Output, RenderContext, RenderError, RenderErrorReason,
};

pub fn register(handlebars: &mut Handlebars) {
    handlebars.register_helper("php_field", Box::new(php_field));
    handlebars.register_helper("comment", Box::new(comment));
}

fn php_field(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let descriptor: FieldDescriptor = h
        .param(0)
        .and_then(|param| serde_json::from_value(param.value().clone()).ok())
        .ok_or(RenderErrorReason::ParamNotFoundForIndex("php_field", 0))?;

    let kind = FieldKind::from_descriptor(&descriptor);

    let (kind, doc_kind) = match kind {
        FieldKind::String => ("string", None),
        FieldKind::U32 => ("int", None),
        FieldKind::Array(inner) => ("array", inner.map(|v| format!("{v}[]"))),
        FieldKind::Class(inner) => (inner, None),
    };

    if let Some(doc_kind) = doc_kind {
        write!(out, include_str!("./php_field_doc_format.txt"), doc_kind)?;
    }

    write!(out, "\t\tpublic {kind} ${};", descriptor.field)?;

    Ok(())
}

/// Renders the `<prefix>` argument, followed by the first item in `<...content_options>` that is `Some`.
/// Usage: {{#comment <prefix> <...content_options>}}
fn comment(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let comment_prefix: String = h
        .param(0)
        .and_then(|param| serde_json::from_value(param.value().clone()).ok())
        .ok_or(RenderErrorReason::ParamNotFoundForIndex("comment", 0))?;

    for param in h.params().iter().skip(1) {
        let content: Option<String> =
            serde_json::from_value(param.value().clone()).map_err(|_| {
                RenderErrorReason::ParamTypeMismatchForName(
                    "comment",
                    "...content_options".to_string(),
                    "Option<String>".to_string(),
                )
            })?;

        if let Some(content) = content {
            write!(out, "{comment_prefix} {content}")?;
            break;
        }
    }

    Ok(())
}
