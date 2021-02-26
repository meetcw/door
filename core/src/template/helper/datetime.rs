use chrono::prelude::*;
use handlebars::*;

pub struct DatetimeHelper;

impl HelperDef for DatetimeHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let datetime = h.param(0).and_then(|v| v.value().as_str()).ok_or(
            RenderError::new(
                "Missing parameter `datetime` for helper `datetime`",
            ),
        )?;
        let format = h
            .param(1)
            .and_then(|v| v.value().as_str())
            .unwrap_or("%b %e, %Y");
        if let Ok(datetime) = datetime.parse::<DateTime<Utc>>() {
            let result = datetime.format(format).to_string();
            out.write(&result)?;
        }
        Ok(())
    }
}
