use handlebars::*;
use serde_json::Value;

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
        #[derive(Serialize, Debug)]
        struct GroupItem<'a> {
            pub name: String,
            pub list: Vec<&'a Value>,
        }
        let value = h.param(0).ok_or_else(|| {
            RenderError::new("Missing parameter for helper `group`")
        })?;
        let group_path = h.param(1).and_then(|v| v.value().as_str()).ok_or(
            RenderError::new("Missing parameter `path` for helper `group`"),
        )?;
        let converter =
            h.hash_get("converter").and_then(|v| v.value().as_str());

        let template = h.template();
        return match value.value() {
            Value::Array(ref list) => {
                let mut groups: Vec<GroupItem> = vec![];
                for item in list {
                    let name = match converter {
                        Some(converter_template) => {
                            let value = item.pointer(group_path).unwrap();
                            r.render_template(converter_template, value)
                                .unwrap()
                        }
                        None => item
                            .pointer(group_path)
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string(),
                    };
                    match groups.iter_mut().find(|g| g.name == name) {
                        Some(ref mut group) => group.list.push(item),
                        None => {
                            let group = GroupItem {
                                name: name.to_string(),
                                list: vec![item],
                            };
                            groups.push(group);
                        }
                    };
                }
                let block_context = BlockContext::new();
                rc.push_block(block_context);
                if let Some(ref mut block) = rc.block_mut() {
                    block.set_base_value(to_json(groups));
                    template.unwrap().render(r, ctx, rc, out)?;
                }
                rc.pop_block();
                Ok(())
            }
            _ => Err(RenderError::new("Invalid parameter for helper `group`")),
        };
    }
}
