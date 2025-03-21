use std::sync::Arc;

use crate::prelude::*;
use crate::document::{Document};
use crate::document_format::{DocumentFormat};
use crate::organization::organize;
use crate::provider::Provider;
use crate::traverse::{build_document_from_meta_context};
use crate::meta_context::MetaContext;

pub async fn translate<P: Provider>(
    provider: Arc<P>,
    meta_context: Arc<MetaContext>,
    options: &Option<Options>,
    json_schema: &str,
) -> Result<Arc<MetaContext>, Errors> {
    log::trace!("In translate");

    unimplemented!()
}

pub async fn translate_meta_context<P: Provider>(
    provider: Arc<P>,
    meta_context: Arc<MetaContext>,
    options: &Option<Options>,
    json_schema: &str,
) -> Result<Arc<MetaContext>, Errors> {
    log::trace!("In translate_meta_context");

    translate(Arc::clone(&provider), meta_context, options, json_schema).await
}

pub async fn translate_text_to_meta_context<P: Provider>(
    provider: Arc<P>,
    text: String,
    options: &Option<Options>,
    json_schema: &str,
) -> Result<Arc<MetaContext>, Errors> {
    log::trace!("In translate_text_to_meta_context");

    let document = Document::from_string(text, options)?;
    let meta_context = organize(Arc::clone(&provider), document, options).await?;

    translate_meta_context(Arc::clone(&provider), meta_context, options, json_schema).await
}

pub async fn translate_text_to_document<P: Provider>(
    provider: Arc<P>,
    text: String,
    options: &Option<Options>,
    document_format: &Option<DocumentFormat>,
    json_schema: &str,
) -> Result<Document, Errors> {
    log::trace!("In translate_text_to_document");

    let meta_context = translate_text_to_meta_context(Arc::clone(&provider), text, options, json_schema).await?;

    build_document_from_meta_context(
        provider,
        meta_context,
        document_format,
    ).await
}

pub async fn translate_text<P: Provider>(
    provider: Arc<P>,
    text: String,
    options: &Option<Options>,
    document_format: &Option<DocumentFormat>,
    json_schema: &str,
) -> Result<String, Errors> {
    log::trace!("In translate_text");

    let document = translate_text_to_document(
        provider,
        text,
        options,
        document_format,
        json_schema
    ).await?;

    Ok(document.to_string())
}

pub async fn translate_document_to_meta_context<P: Provider>(
    provider: Arc<P>,
    document: Document,
    options: &Option<Options>,
    json_schema: &str,
) -> Result<Arc<MetaContext>, Errors> {
    log::trace!("In translate_document_to_meta_context");

    let meta_context = organize(Arc::clone(&provider), document, options).await?;

    translate_meta_context(Arc::clone(&provider), meta_context, options, json_schema).await
}

pub async fn translate_document<P: Provider>(
    provider: Arc<P>,
    document: Document,
    options: &Option<Options>,
    document_format: &Option<DocumentFormat>,
    json_schema: &str,
) -> Result<Document, Errors> {
    log::trace!("In translate_document");

    let meta_context = translate_document_to_meta_context(
        provider.clone(),
        document,
        options,
        json_schema
    ).await?;

    build_document_from_meta_context(
        provider,
        meta_context,
        document_format,
    ).await
}

pub async fn translate_document_to_text<P: Provider>(
    provider: Arc<P>,
    document: Document,
    options: &Option<Options>,
    document_format: &Option<DocumentFormat>,
    json_schema: &str,
) -> Result<String, Errors> {
    log::trace!("In translate_document_to_text");

    let document = translate_document(
        provider,
        document,
        options,
        document_format,
        json_schema
    ).await?;

    Ok(document.to_string())
}

pub async fn translate_file_to_meta_context<P: Provider>(
    provider: Arc<P>,
    path: &str,
    options: &Option<Options>,
    json_schema: &str,
) -> Result<Arc<MetaContext>, Errors> {
    log::trace!("In translate_file_to_meta_context");
    log::debug!("file path: {}", path);

    let text = get_file_as_text(path).map_err(|err| {
        log::error!("Failed to get file as text: {:?}", err);
        Errors::FileInputError
    })?;

    translate_text_to_meta_context(Arc::clone(&provider), text, options, json_schema).await
}

pub async fn translate_file_to_document<P: Provider>(
    provider: Arc<P>,
    path: &str,
    options: &Option<Options>,
    document_format: &Option<DocumentFormat>,
    json_schema: &str,
) -> Result<Document, Errors> {
    log::trace!("In translate_file_to_document");

    let meta_context = translate_file_to_meta_context(Arc::clone(&provider), path, options, json_schema).await?;

    build_document_from_meta_context(
        provider,
        meta_context,
        document_format,
    ).await
}

pub async fn translate_file_to_text<P: Provider>(
    provider: Arc<P>,
    path: &str,
    options: &Option<Options>,
    document_format: &Option<DocumentFormat>,
    json_schema: &str,
) -> Result<String, Errors> {
    log::trace!("In translate_file_to_text");

    let document = translate_file_to_document(
        provider,
        path,
        options,
        document_format,
        json_schema
    ).await?;

    Ok(document.to_string())
}

pub async fn translate_file<P: Provider>(
    provider: Arc<P>,
    path: &str,
    options: &Option<Options>,
    document_format: &Option<DocumentFormat>,
    json_schema: &str,
) -> Result<(), Errors> {
    log::trace!("In translate_file");
    log::debug!("file path: {}", path);

    let text = translate_file_to_text(Arc::clone(&provider), path, options, document_format, json_schema).await?;
    let new_path = append_to_filename(path, "_translated")?;

    write_text_to_file(&new_path, &text).map_err(|err| {
        log::error!("Failed to write translated text to file: {:?}", err);
        Errors::FileOutputError
    })
}
