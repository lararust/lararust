use regex::Regex;

/// Compiles Blade-like syntax into interpretable tokens.
/// Handles `@if($var)`, `@elseif($var)`, `@foreach($list as $item)` and normalizes `{{ $var }}` placeholders.
pub fn compile_larablade(content: &str) -> String {
    let mut result = content.to_string();

    // Handle @if(...) → {% if var %}
    let if_re = Regex::new(r"@if\s*\((.+?)\)").unwrap();
    result = if_re
        .replace_all(&result, |caps: &regex::Captures| {
            format!(
                "{{% if {} %}}",
                normalize_identifier(caps.get(1).unwrap().as_str())
            )
        })
        .to_string();

    // Handle @elseif(...) → {% else if var %}
    let elseif_re = Regex::new(r"@elseif\s*\((.+?)\)").unwrap();
    result = elseif_re
        .replace_all(&result, |caps: &regex::Captures| {
            format!(
                "{{% else if {} %}}",
                normalize_identifier(caps.get(1).unwrap().as_str())
            )
        })
        .to_string();

    result = result.replace("@else", "{% else %}");
    result = result.replace("@endif", "{% endif %}");

    // Handle @foreach($items as $item) → {% for item in items %}
    let for_re =
        Regex::new(r"@foreach\s*\(\s*(.+?)\s+as\s+(.+?)\s*\)").unwrap();
    result = for_re
        .replace_all(&result, |caps: &regex::Captures| {
            let collection =
                normalize_identifier(caps.get(1).unwrap().as_str());
            let item = normalize_identifier(caps.get(2).unwrap().as_str());
            format!("{{% for {} in {} %}}", item, collection)
        })
        .to_string();
    result = result.replace("@endforeach", "{% endfor %}");

    // Normalize {{ $var }} placeholders to {{ var }}
    let placeholder_re = Regex::new(r"\{\{\s*\$?(\w+)\s*\}\}").unwrap();
    result = placeholder_re
        .replace_all(&result, "{{ $1 }}")
        .to_string();

    result
}

fn normalize_identifier(raw: &str) -> String {
    raw.trim()
        .trim_matches(|c| c == '(' || c == ')')
        .trim_start_matches('$')
        .to_string()
}
