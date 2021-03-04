use handlebars::*;
use serde_json::Value;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct HashHelper;

impl HelperDef for HashHelper {
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
                let mut state = DefaultHasher::new();
                data.as_str().unwrap().hash(&mut state);
                let result = state.finish();
                out.write(&result.to_string())?;
            }
        }
        Ok(())
    }
}
