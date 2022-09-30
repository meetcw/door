use handlebars::*;
use pulldown_cmark::*;
use slug::slugify;

#[derive(Serialize, Debug)]
struct TOC {
    pub root: TOCNode,
}

impl<'a> TOC {
    pub fn get_parent_node(&mut self, level: u32) -> &mut TOCNode {
        let mut current = &mut self.root;
        loop {
            let current_level = current.level;
            if current_level == level {
                return current;
            } else {
                if current.list.is_none() {
                    current.list = Some(vec![]);
                    current.list.as_mut().unwrap().push(TOCNode {
                        name: None,
                        level: current_level + 1,
                        list: None,
                    });
                }
                current =
                    current.list.as_mut().unwrap().last_mut().unwrap();
            }
        }
    }

    pub fn insert(&mut self, node: TOCNode) {
        let parent = &mut self.get_parent_node(node.level - 1);
        if parent.list.is_none() {
            parent.list = Some(vec![]);
        }
        parent.list.as_mut().unwrap().push(node);
    }
}

#[derive(Serialize, Debug)]
struct TOCNode {
    pub name: Option<String>,
    pub level: u32,
    pub list: Option<Vec<TOCNode>>,
}

#[derive(Clone, Copy)]
pub struct MarkdownTOCHelper;

impl HelperDef for MarkdownTOCHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        r: &'reg Handlebars,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let markdown_input = h
            .param(0)
            .ok_or(RenderError::new("Missing content for helper `markdown`"))?;

        if let Some(markdown_input) = markdown_input.value().as_str() {
            let toc = markdown_to_toc(markdown_input)?;
            let template = h.template();
            let block_context = BlockContext::new();
            rc.push_block(block_context);
            if let Some(ref mut block) = rc.block_mut() {
                block.set_base_value(to_json(toc));
                template.unwrap().render(r, ctx, rc, out)?;
            }
            rc.pop_block();
            Ok(())
        } else {
            Err(RenderError::new(
                "Require string data for helper `markdown`",
            ))
        }
    }
}

fn markdown_to_toc<S: AsRef<str>>(
    markdown_input: S,
) -> Result<TOCNode, RenderError> {
    let options = Options::all();
    let parser = Parser::new_ext(markdown_input.as_ref(), options);

    let mut in_header = false;
    let mut header_text = String::new();
    let mut toc = TOC {
        root: TOCNode {
            name: None,
            level: 0,
            list: None,
        },
    };
    for event in parser {
        match event {
            Event::Start(Tag::Heading(_level)) => {
                in_header = true;
            }
            Event::Text(text) => {
                if in_header {
                    let text = text.into_string();
                    header_text.push_str(&text);
                }
            }
            Event::End(Tag::Heading(level)) => {
                let node = TOCNode {
                    name: Some(header_text.clone()),
                    level: level,
                    list: None,
                };
                toc.insert(node);
                in_header = false;
                header_text.clear();
            }
            _ => {}
        }
    }

    return Ok(toc.root);
}

#[derive(Clone, Copy)]
pub struct MarkdownHTMLHelper;

impl HelperDef for MarkdownHTMLHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _: &'reg Handlebars,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let markdown_input = h
            .param(0)
            .ok_or(RenderError::new("Missing content for helper `markdown`"))?;

        if let Some(markdown_input) = markdown_input.value().as_str() {
            let html_output = markdown_to_html(markdown_input)?;
            out.write(&html_output)?;

            Ok(())
        } else {
            Err(RenderError::new(
                "Require string data for helper `markdown`",
            ))
        }
    }
}

fn markdown_to_html<S: AsRef<str>>(
    markdown_input: S,
) -> Result<String, RenderError> {
    let options = Options::all();
    let parser = Parser::new_ext(markdown_input.as_ref(), options);

    let mut header_index = None;
    let mut header_text = String::new();
    let mut events = vec![];
    for event in parser {
        events.push(event);
    }

    for i in 0..events.len() {
        match events[i] {
            Event::Start(Tag::Heading(_)) => {
                header_index = Some(i);
            }
            Event::Text(ref text) => {
                if header_index.is_some() {
                    let text = text.as_ref();
                    header_text.push_str(text);
                }
            }
            Event::End(Tag::Heading(level)) => {
                if let Some(index) = header_index {
                    let text = format!(
                        "<h{} id=\"toc-{}\">",
                        level,
                        slugify(header_text.clone())
                    );
                    events[index] = Event::Html(CowStr::from(text));
                    header_index = None;
                    header_text.clear();
                }
            }
            _ => {}
        };
    }
    let mut html_output = String::new();
    html::push_html(&mut html_output, events.into_iter());
    return Ok(html_output);
}
