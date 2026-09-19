use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use tera::{Context, Tera};
use crate::app::AppError;
use crate::model::summary::FeedSummary;

/// render data into html templates
pub fn render_html(groups:BTreeMap<String,Vec<FeedSummary>>,template_path:&PathBuf)->Result<String,AppError>{
    let mut tera = Tera::new();
    tera.add_template_file(template_path,Some("template"))?;
    let mut ctx=Context::default();
    ctx.insert("groups",&groups);
    let output=tera.render("template",&ctx)?;
    Ok(output)
}

#[cfg(test)]
mod tests{
    use std::collections::{BTreeMap, HashMap};
    use tera::{Context, Tera};
    use crate::app::AppError;
    use crate::config::example::{EXAMPLE_SUMMARY_OUTPUT, EXAMPLE_SUMMARY_TEMPLATE};
    use crate::model::summary::FeedSummary;
    use crate::render::render_html;

    #[test]
    fn test_render_html() -> Result<(), AppError>{
        let mut groups:BTreeMap<String,Vec<FeedSummary>>=BTreeMap::new();
        let v:FeedSummary=serde_json::from_str(EXAMPLE_SUMMARY_OUTPUT)?;
        let v1=v.clone();
        groups.insert("group1".to_string(),vec![v.clone(),v1]);
        let v2=v.clone();
        let v3 =v.clone();
        groups.insert("group2".to_string(),vec![v2,v3]);
        groups.insert("group3".to_string(),vec![]);
        // println!("{:?}",groups);

        let template=EXAMPLE_SUMMARY_TEMPLATE;
        let mut tera=Tera::new();
        let mut ctx=Context::default();
        tera.add_raw_template("template",template)?;
        ctx.insert("groups",&groups);
        let output = tera
            .render("template", &ctx)
            .unwrap_or_else(|error| panic!("Tera 渲染失败：{error:#}"));        println!("{output}");
        Ok(())
    }
}