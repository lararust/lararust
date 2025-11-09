use regex::Regex;
use tera::{Context, Value};

pub fn render_larablade(template: &str, context: &Context) -> String {
    let mut output = template.to_string();

    let if_else_re =
        Regex::new(r"(?s)\{% if (.+?) %\}(.*?)\{% else %\}(.*?)\{% endif %\}").unwrap();
    output = replace_conditionals(output, &if_else_re, context);

    let if_only_re = Regex::new(r"(?s)\{% if (.+?) %\}(.*?)\{% endif %\}").unwrap();
    output = replace_conditionals(output, &if_only_re, context);

    let var_re = Regex::new(r"\{\{\s*([\w\.]+)\s*\}\}").unwrap();
    output = var_re
        .replace_all(&output, |caps: &regex::Captures| {
            let var_name = caps[1].trim();
            if let Some(value) = context.get(var_name) {
                match value {
                    Value::String(s) => s.to_string(),
                    Value::Bool(b) => b.to_string(),
                    Value::Number(n) => n.to_string(),
                    _ => format!("{:?}", value),
                }
            } else {
                format!("{{{{ {} }}}}", var_name)
            }
        })
        .to_string();

    output
}

fn replace_conditionals(mut template: String, regex: &Regex, context: &Context) -> String {
    while let Some(cap) = regex.captures(&template) {
        let full = cap.get(0).unwrap().as_str();
        let var = cap.get(1).unwrap().as_str().trim();
        let true_block = cap.get(2).unwrap().as_str();
        let false_block = cap.get(3).map(|m| m.as_str());

        let condition = context
            .get(var)
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let replacement = if condition {
            true_block
        } else {
            false_block.unwrap_or("")
        };

        template = template.replacen(full, replacement, 1);
    }

    template
}
