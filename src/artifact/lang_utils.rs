use std::fmt::Display;

use super::Language;
use crate::html_utils::render_text_tag;

impl PartialOrd for Language {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Language {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        format!("{:?}", self).cmp(&format!("{:?}", other))
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Language {
    pub fn icon(&self) -> String {
        // https://devicon.dev/
        let colored = match self {
            Language::Rust => "",
            Language::Lua => "text-blue-500",
            Language::Bash => "text-gray-500",
            Language::Ruby => "colored scale-75",
            Language::Zig => "colored scale-90",
            Language::Elixir => "text-purple-500",
            Language::Perl => "text-indigo-600",
            _ => "colored",
        };
        let lang_lower = format!("{:?}", self).to_lowercase();

        format!(
            r#"<i class="devicon-{}-plain {}" style="font-size: 3rem"></i>"#,
            lang_lower, colored
        )
    }

    pub fn to_tag(&self) -> String {
        render_text_tag(&self.to_string())
    }
}

// derive deserialize for Language
