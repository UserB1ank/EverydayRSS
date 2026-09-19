use std::path::PathBuf;
use tera::Tera;
use crate::app::AppError;
use crate::model::summary::FeedSummary;

/// render data into html templates
fn render_html(summaries:Vec<FeedSummary>,template_path:&PathBuf)->Result<String,AppError>{
    let mut tera = Tera::new();
    tera.add_template_file(template_path,Some("template"))?;

    Ok("".to_string())
}