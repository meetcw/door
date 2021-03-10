use handlebars::*;
use pulldown_cmark::*;

#[derive(Serialize, Debug)]
struct MarkdownTOCItem {
    pub name: String,
    pub anchor: String,
    pub level: u32,
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
            let toc_list = markdown_toc(markdown_input)?;
            let template = h.template();
            let block_context = BlockContext::new();
            rc.push_block(block_context);
            if let Some(ref mut block) = rc.block_mut() {
                block.set_base_value(to_json(toc_list));
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


fn markdown_toc<S: AsRef<str>>(
    markdown_input: S,
) -> Result<Vec<MarkdownTOCItem>, RenderError> {
    let options = Options::all();
    let parser = Parser::new_ext(markdown_input.as_ref(), options);

    let mut in_header = false;
    let mut header_text = String::new();
    let mut header_index = 0;
    let mut toc_list = vec![];
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
                header_index += 1;
                let anchor = format!(
                    "toc-{}",
                    roman::to(header_index).unwrap().to_lowercase()
                );
                let toc = MarkdownTOCItem {
                    name: header_text.clone(),
                    anchor: anchor,
                    level: level,
                };
                toc_list.push(toc);
                in_header = false;
                header_text.clear();
            }
            _ => {}
        }
    }
    return Ok(toc_list);
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

    let mut header_index = 0;
    let parser = parser.map(|event| match event {
        Event::Start(Tag::Heading(level)) => {
            header_index += 1;
            let header = format!(
                "<h{} id=\"toc-{}\">",
                level,
                roman::to(header_index).unwrap().to_lowercase()
            );
            Event::Html(CowStr::from(header))
        }
        _ => event,
    });
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    return Ok(html_output);
}
