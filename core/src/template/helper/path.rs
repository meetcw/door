use handlebars::*;
use path_absolutize::*;
use std::path::PathBuf;

pub struct PathHelper;

impl HelperDef for PathHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let mut path = PathBuf::new();
        for item in h.params() {
            let item_string = item.value().as_str().unwrap_or_default();
            path.push(item_string);
        }
        let path = path.absolutize().unwrap();
        let result = path.to_str().unwrap();
        out.write(result)?;
        Ok(())
    }
}
