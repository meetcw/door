use handlebars::*;

pub struct AssignHelper;

impl HelperDef for AssignHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        ctx: &Context,
        rc: &mut RenderContext,
        _: &mut dyn Output,
    ) -> HelperResult {
        // debug!("///////{:?}",h);
        // debug!("+++++++{:?}",ctx);
        // debug!("======={:?}",rc);
        let name = h.param(0).and_then(|v| v.value().as_str()).ok_or(
            RenderError::new("Missing parameter `name` for helper `assign`"),
        )?;
        let value =
            h.param(1)
                .map(|v| v.value())
                .cloned()
                .ok_or(RenderError::new(
                    "Missing parameter `value` for helper `assign`",
                ))?;
        let mut ctx = ctx.clone();
        match ctx.data_mut() {
            serde_json::value::Value::Object(m) => {
                m.insert(name.to_owned(), value)
            }
            _ => None,
        };
        // debug!("-------{:?}",ctx);
        rc.set_context(ctx);
        Ok(())
    }
}
