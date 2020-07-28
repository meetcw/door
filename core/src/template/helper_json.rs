use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext,
};

pub struct JsonHelper;

impl HelperDef for JsonHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        if let Some(param) = h.param(0) {
            let data = param.value();
            out.write(&data.to_string())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::super::*;

    #[test]
    fn json_helper() {
        let tpl = "{{#file \"a.json\"}}{{json this}}{{/file}}";
        let data = json!({ "name" : "about" });
        let mut renderer = DefaultRenderer::new();
        renderer.register_template_string("about", tpl).unwrap();
        let render_result = renderer.render("about", &data).unwrap();
        println!("{:?}", render_result);
    }
}
