use handlebars::*;
use std::collections::hash_map::DefaultHasher;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};

pub struct IdentityHelper;

impl HelperDef for IdentityHelper {
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
                let result = identity_of(data.as_str().unwrap());
                out.write(&result)?;
            }
        }
        Ok(())
    }
}

fn identity_of(source: &str) -> String {
    let mut state = DefaultHasher::new();
    source.hash(&mut state);
    let mut hash_code = state.finish();
    let mut roman_number_list = Vec::new();
    while hash_code > 0 {
        let number = i32::try_from(hash_code % 10).unwrap();
        roman_number_list.push(
            roman::to(number)
                .unwrap_or(String::from("z"))
                .to_lowercase(),
        );
        hash_code = hash_code / 10;
    }
    roman_number_list.reverse();
    let result = roman_number_list.join("-");
    return result;
}
