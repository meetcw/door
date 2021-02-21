use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::Weak;

use handlebars::*;

pub struct FileHelper {
    pub file_map: Weak<RwLock<HashMap<String, String>>>,
}

impl HelperDef for FileHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        r: &'reg Handlebars<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let parms: Vec<String> = h
            .params()
            .iter()
            .map(|x| {
                let mut param = String::new();
                if x.value().is_string() {
                    if let Some(temp) = x.value().as_str() {
                        param = temp.to_string();
                    }
                } else if !x.value().is_null() && !x.value().is_object() {
                    param = x.value().to_string();
                }
                return param;
            })
            .collect();

        let file_path = parms.join("");

        let template = h.template();

        let result = template
            .map(|t| t.renders(r, ctx, rc))
            .unwrap_or(Ok("".to_string()));
        let temp = self.file_map.upgrade().unwrap();
        let mut writeable_file_map = temp.write().unwrap();

        writeable_file_map
            .insert(file_path, result.unwrap_or(String::default()));

        return match template {
            Some(ref t) => t.render(r, ctx, rc, out),
            None => Ok(()),
        };
    }
}

#[cfg(test)]
mod tests {

    use super::super::*;

    #[test]
    fn file_helper() {
        let tpl = "{{#file \"./post/\" name \".html\"}}{{name}}{{/file}}";
        let data = json!({ "name" : "about" });
        let mut renderer = DefaultRenderer::new();
        renderer.register_template_string("about", tpl).unwrap();
        let file_map = renderer.render("about", &data).unwrap();
        println!("{:?}", file_map);
        assert_eq!(file_map.len(), 1);
    }
}
