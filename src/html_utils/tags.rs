/// Make a tag with the given text and color.
pub fn make_tag(text: &str, color: &str) -> String {
    let color_class = format!("inline-flex items-center rounded-md mr-2 px-2 py-1 text-xs font-medium ring-1 ring-inset bg-{}-500/20 text-{}-400 ring-{}-500/80", color, color, color);
    format!(r#"<span class="{}">{}</span>"#, color_class, text)
}

/// Render a tag with the given text.
/// The color is determined by the text.
pub fn render_text_tag(tag: &str) -> String {
    make_tag(
        tag,
        match tag {
            "Rust" => "orange",
            "C" => "red",
            "Python" => "yellow",
            "Haskell" => "purple",
            "OCaml" => "blue",
            "Bash" => "green",
            "Clojure" => "purple",
            "Go" => "cyan",

            "ADT" => "green",
            "Recursion" => "yellow",
            "Sorting" => "blue",
            "Graph" => "purple",
            "Math" => "sky",
            "Concurrency" => "cyan",
            "OS" | "Dangerous" | "Bare Metal" => "red",
            _ => "gray",
        },
    )
}
