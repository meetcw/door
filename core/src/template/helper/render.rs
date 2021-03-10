use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::Weak;

use handlebars::*;

pub struct RenderHelper {
    pub file_map: Weak<RwLock<HashMap<String, String>>>,
}

impl HelperDef for RenderHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        r: &'reg Handlebars<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let file_path = h.param(0).and_then(|v| v.value().as_str()).ok_or(
            RenderError::new("Missing parameter `path` for helper `file`"),
        )?;

        let template = h.template();

        let result = template
            .map(|t| t.renders(r, ctx, rc))
            .unwrap_or(Ok("".to_string()));
        let temp = self.file_map.upgrade().unwrap();
        let mut writeable_file_map = temp.write().unwrap();
        writeable_file_map
            .insert(file_path.to_string(), result.unwrap_or(String::default()));
        return match template {
            Some(ref t) => t.render(r, ctx, rc, out),
            None => Ok(()),
        };
    }
}
