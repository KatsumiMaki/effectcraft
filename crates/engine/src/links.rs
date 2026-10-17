//! Community and project links (shown in Help ▸ and the header's community buttons).

pub const APP_NAME: &str = "EffectCraft";
pub const APP_SLUG: &str = "effectcraft";
pub const DISCORD: &str = "https://discord.gg/artcraft";
pub const WEBSITE: &str = "https://getartcraft.com";
pub const APP_PAGE: &str = "https://getartcraft.com/apps/effectcraft";
pub const GITHUB: &str = "https://github.com/storytold/effectcraft";
pub const ISSUES: &str = "https://github.com/storytold/effectcraft/issues";

pub fn github_of(slug: &str) -> String {
    format!("https://github.com/storytold/{slug}")
}
pub fn page_of(slug: &str) -> String {
    format!("https://getartcraft.com/apps/{slug}")
}
