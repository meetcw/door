use handlebars::*;

pub struct StringHelper;

impl HelperDef for StringHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let s: String = h
            .params()
            .iter()
            .map(|s| s.value().as_str().unwrap_or_default())
            .collect();

        out.write(&s)?;
        Ok(())
    }
}
