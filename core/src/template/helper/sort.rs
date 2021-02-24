use handlebars::*;
use serde_json::Value;

pub struct SortHelper;

impl HelperDef for SortHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        r: &'reg Handlebars<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let value = h.param(0).ok_or_else(|| {
            RenderError::new("Missing parameter for helper `sort`")
        })?;
        let sort_path = h
            .param(1)
            .and_then(|v| v.value().as_str())
            .ok_or(RenderError::new("Missing parameter `path` for helper `sort`"))?;
        let invert = h
            .hash_get("invert")
            .and_then(|v| v.value().as_bool())
            .unwrap_or(false);
        let template = h.template();
        return match value.value() {
            Value::Array(ref list) => {
                let mut list = list.to_vec();
                list.sort_by(|a, b| {
                    let a = a.pointer(sort_path).unwrap().as_str().unwrap();
                    let b = b.pointer(sort_path).unwrap().as_str().unwrap();
                    if invert {
                        b.cmp(&a)
                    } else {
                        a.cmp(&b)
                    }
                });
                let block_context = BlockContext::new();
                rc.push_block(block_context);
                if let Some(ref mut block) = rc.block_mut() {
                    block.set_base_value(to_json(list));
                    template.unwrap().render(r, ctx, rc, out)?;
                }
                rc.pop_block();
                Ok(())
            }
            _ => {
                Err(RenderError::new("Invalid parameter for helper `sort`"))
            }
        };
    }
}
