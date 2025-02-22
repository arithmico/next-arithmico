use std::collections::HashMap;

use leptos::prelude::expect_context;

use crate::TranslateContext;

pub fn use_translate()
-> impl Fn(&str, Option<HashMap<String, String>>) -> Result<String, &'static str>
{
    |id: &str, keys: Option<HashMap<String, String>>| {
        let context = expect_context::<TranslateContext>();
        let template = context
            .get_template_provider()
            .get_template(&id)
            .ok_or("TranslationError")?;
        let keys = keys.unwrap_or_default();

        match template.translate_with(context.get_current_language(), &keys) {
            Ok(ok) => Ok(ok),
            Err(_) => match template
                .translate_with(context.get_fallback_language(), &keys)
            {
                Ok(ok) => Ok(ok),
                Err(_) => Err("TranslationError"),
            },
        }
    }
}
