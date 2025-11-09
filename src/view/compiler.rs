use regex::Regex;

pub fn compile_larablade(content: &str) -> String {
    let mut html = content.to_string();

    // @if → {% if %}
    let if_re = Regex::new(r"@if (.+)").unwrap();
    html = if_re
        .replace_all(&html, "{% if $1 %}")
        .to_string();

    // @elseif → {% elif %}
    let elseif_re = Regex::new(r"@elseif (.+)").unwrap();
    html = elseif_re
        .replace_all(&html, "{% elif $1 %}")
        .to_string();

    // @else → {% else %}
    html = html.replace("@else", "{% else %}");

    // @endif → {% endif %}
    html = html.replace("@endif", "{% endif %}");

    // @foreach → {% for %}
    let for_re = Regex::new(r"@foreach (.+) as (.+)").unwrap();
    html = for_re
        .replace_all(&html, "{% for $2 in $1 %}")
        .to_string();

    // @endforeach → {% endfor %}
    html = html.replace("@endforeach", "{% endfor %}");

    // {{ var }} → {{ var }}
    // (sem alteração, só garantir que não há @{{ ou algo escapado)
    html
}
