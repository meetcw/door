use handlebars::*;

pub struct GroupHelper;

impl HelperDef for GroupHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        r: &'reg Handlebars<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let value = h.param(0).ok_or_else(|| {
            RenderError::new("Param not found for helper \"sort\"")
        })?;
        let template = h.template();
        let mut block_context = BlockContext::new();

        if let Some(path) = value.context_path() {
            // debug!("-------------\n{:?}", path);
            *block_context.base_path_mut() = path.to_vec();
        }
        rc.push_block(block_context);
        debug!("-------------\n{:?}", rc);
        if let Some(ref mut block) = rc.block_mut() {
            block.set_local_var("@group".to_string(), to_json("hello"));
            template.unwrap().render(r, ctx, rc, out)?;
        }
        rc.pop_block();
        Ok(())
    }
}
