use crate::minecraft_line::MinecraftLine;
use regex::Regex;

use super::SendRule;

#[derive(Clone)]
pub struct AdvancementRule;

impl SendRule for AdvancementRule {
    fn send(&self, line: &MinecraftLine) -> Option<String> {
        if !line.caused_at.contains("Server thread") || !line.level.eq("INFO") {
            return None;
        }

        let advancement_re =
            Regex::new(r"^(.*)\s\shas\smade\sthe\sadvancement\s\[(.*)\]$").unwrap();

        advancement_re.captures(&line.message).map(|cap| {
            let name = cap
                .get(1)
                .map(|name| name.as_str().to_string())
                .unwrap_or("".to_string());
            let kind = cap
                .get(2)
                .map(|m| m.as_str().to_string())
                .unwrap_or("".to_string());
            format!("**{name}** has made the advancement _**{kind}**_").to_string()
        })
    }
}
