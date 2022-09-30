use handlebars::*;

pub struct JsonHelper;

impl HelperDef for JsonHelper {
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
            out.write(&serde_json::to_string(data).unwrap())?;
        }
        Ok(())
    }
}
