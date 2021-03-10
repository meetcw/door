use handlebars::*;

pub struct CountHelper;

impl HelperDef for CountHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).ok_or(RenderError::new(
            "Missing parameter `array` for helper `count`",
        ))?;
        let array= param.value()
            .as_array()
            .ok_or_else(||
                        param.relative_path()
                        .map_or_else(|| RenderError::new(format!(
                            "The parameter for helper `count` is not a valid array `{:?}`",
                            ""
                        )), |v| RenderError::new(format!(
                            "The parameter for helper is not a valid array `{}`",
                            v
                        ))))?;
        Ok(out.write(&array.len().to_string())?)
        // match h.param(0) {
        //     Some(param) => {
        //         let data = param.value();
        //         match data.as_array() {
        //             Some(array) => Ok(out.write(&array.len().to_string())?),
        //             None => {
        //                 match param.relative_path(){
        //                     Some(relative_path) =>Err(RenderError::new(format!(
        //                         "The parameter for helper is not a valid array `{}`",
        //                         relative_path
        //                     ))),
        //                     None =>Err(RenderError::new(format!(
        //                         "The parameter for helper `count` is not a valid array `{:?}`",
        //                         data
        //                     )))
        //                 }
        //             }
        //         }
        //     }
        //     None => Err(RenderError::new(
        //         "Missing parameter `array` for helper `count`",
        //     )),
        // }
    }
}
