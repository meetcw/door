use handlebars::*;
use serde_json::Value;

pub struct CountHelper;

impl HelperDef for CountHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        if let Some(param) = h.param(0) {
            let data = param.value();
            if data.is_array() {
                let result = data.as_array().unwrap().len();
                out.write(&result.to_string())?;
            }
        }
        Ok(())
    }
}
