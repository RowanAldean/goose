use include_dir::{include_dir, Dir};

static BUILTIN_SKILLS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/skills/builtins");

pub fn get_all() -> Vec<&'static str> {
    BUILTIN_SKILLS_DIR
        .files()
        .filter(|f| f.path().extension().is_some_and(|ext| ext == "md"))
        .filter_map(|f| f.contents_utf8())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn web_search_skill_includes_parallel_as_an_opt_in_provider() {
        let skill = get_all()
            .into_iter()
            .find(|content| content.contains("name: web-search"))
            .expect("web-search builtin skill");

        assert!(skill.contains("PARALLEL_API_KEY"));
        assert!(skill.contains("uvx --from parallel-web"));
        assert!(skill.contains("Parallel if `PARALLEL_API_KEY` is set"));
        assert!(skill.contains("otherwise DuckDuckGo"));
    }
}
