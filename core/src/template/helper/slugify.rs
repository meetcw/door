use handlebars::*;
use slug::slugify;

pub struct SlugifyHelper;

impl HelperDef for SlugifyHelper {
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
            if data.is_string() {
                let result = slugify(data.as_str().unwrap());
                out.write(&result)?;
            }
        }
        Ok(())
    }
}
