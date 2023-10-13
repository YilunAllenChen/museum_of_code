pub fn make_tag(text: &str, color: &str) -> String {
    let color_class = format!("inline-flex items-center rounded-md mr-2 px-2 py-1 text-xs font-medium ring-1 ring-inset bg-{}-500/20 text-{}-400 ring-{}-500/80", color, color, color);
    format!(r#"<span class="{}">{}</span>"#, color_class, text)
}
