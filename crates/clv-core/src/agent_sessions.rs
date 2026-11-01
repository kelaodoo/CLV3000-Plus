use crate::category::CleanupCategory;
use crate::messages::RuleDescription;
use crate::models::{RiskLevel, TechStack};
use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct AgentSessionTarget {
    pub path: PathBuf,
    pub stack: TechStack,
    pub risk: RiskLevel,
    pub category: CleanupCategory,
    pub description: RuleDescription,
}

/// Known coding-agent session / cache locations (inspired by ccusage path resolution).
pub fn discover_agent_session_targets() -> Vec<AgentSessionTarget> {
    let Some(home) = directories::UserDirs::new().map(|d| d.home_dir().to_path_buf()) else {
        return Vec::new();
    };

    let mut targets = Vec::new();

    for codex_home in codex_home_paths(&home) {
        push_dir(
            &mut targets,
            codex_home.join("sessions"),
            RiskLevel::Caution,
            CleanupCategory::AgentSession,
            RuleDescription::R106,
        );
        push_dir(
            &mut targets,
            codex_home.join("archived_sessions"),
            RiskLevel::Caution,
            CleanupCategory::AgentSession,
            RuleDescription::R107,
        );
    }

    for claude_root in claude_config_paths(&home) {
        push_dir(
            &mut targets,
            claude_root.join("projects"),
            RiskLevel::Caution,
            CleanupCategory::AgentSession,
            RuleDescription::R108,
        );
    }

    for cursor_root in cursor_data_roots(&home) {
        push_dir(
            &mut targets,
            cursor_root.join("CachedData"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R109,
        );
        push_dir(
            &mut targets,
            cursor_root.join("GPUCache"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R110,
        );
        push_dir(
            &mut targets,
            cursor_root.join("Code Cache"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R111,
        );
        push_dir(
            &mut targets,
            cursor_root.join("CachedExtensionVSIXs"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R112,
        );
        push_dir(
            &mut targets,
            cursor_root.join("logs"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R088,
        );
    }

    for windsurf_root in windsurf_data_roots(&home) {
        push_dir(
            &mut targets,
            windsurf_root.join("CachedData"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R113,
        );
        push_dir(
            &mut targets,
            windsurf_root.join("GPUCache"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R114,
        );
        push_dir(
            &mut targets,
            windsurf_root.join("Code Cache"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R115,
        );
        push_dir(
            &mut targets,
            windsurf_root.join("logs"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R116,
        );
    }

    push_dir(
        &mut targets,
        home.join(".cursor/projects"),
        RiskLevel::Caution,
        CleanupCategory::AgentSession,
        RuleDescription::R117,
    );

    for workbuddy_root in workbuddy_data_roots(&home) {
        push_dir(
            &mut targets,
            workbuddy_root.join("sessions"),
            RiskLevel::Caution,
            CleanupCategory::AgentSession,
            RuleDescription::R118,
        );
        push_dir(
            &mut targets,
            workbuddy_root.join("logs"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R119,
        );
        push_dir(
            &mut targets,
            workbuddy_root.join("traces"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R120,
        );
        push_dir(
            &mut targets,
            workbuddy_root.join("file-history"),
            RiskLevel::Caution,
            CleanupCategory::AgentSession,
            RuleDescription::R121,
        );
        push_dir(
            &mut targets,
            workbuddy_root.join("shell-snapshots"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R122,
        );
    }

    for trae_root in trae_app_data_roots(&home) {
        push_dir(
            &mut targets,
            trae_root.join("CachedData"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R123,
        );
        push_dir(
            &mut targets,
            trae_root.join("GPUCache"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R124,
        );
        push_dir(
            &mut targets,
            trae_root.join("Code Cache"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R125,
        );
        push_dir(
            &mut targets,
            trae_root.join("Cache"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R123,
        );
        push_dir(
            &mut targets,
            trae_root.join("logs"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R093,
        );
        push_dir(
            &mut targets,
            trae_root.join("ModularData/ai-agent/snapshot"),
            RiskLevel::Caution,
            CleanupCategory::AgentSession,
            RuleDescription::R126,
        );
        push_dir(
            &mut targets,
            trae_root.join("User/workspaceStorage"),
            RiskLevel::Caution,
            CleanupCategory::AgentSession,
            RuleDescription::R127,
        );
        push_dir(
            &mut targets,
            trae_root.join("User/globalStorage"),
            RiskLevel::Caution,
            CleanupCategory::AgentSession,
            RuleDescription::R128,
        );
    }

    for traex_root in traex_session_roots(&home) {
        push_dir(
            &mut targets,
            traex_root.join("sessions"),
            RiskLevel::Caution,
            CleanupCategory::AgentSession,
            RuleDescription::R129,
        );
        push_dir(
            &mut targets,
            traex_root.join("archived_sessions"),
            RiskLevel::Caution,
            CleanupCategory::AgentSession,
            RuleDescription::R130,
        );
    }

    for opencode_root in opencode_data_roots(&home) {
        push_dir(
            &mut targets,
            opencode_root.join("log"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R131,
        );
        push_dir(
            &mut targets,
            opencode_root.join("project"),
            RiskLevel::Caution,
            CleanupCategory::AgentSession,
            RuleDescription::R132,
        );
        push_dir(
            &mut targets,
            opencode_root.join("storage"),
            RiskLevel::Caution,
            CleanupCategory::AgentSession,
            RuleDescription::R133,
        );
    }

    for agy_cli_root in antigravity_cli_roots(&home) {
        push_dir(
            &mut targets,
            agy_cli_root.join("brain"),
            RiskLevel::Caution,
            CleanupCategory::AgentSession,
            RuleDescription::R141,
        );
        push_dir(
            &mut targets,
            agy_cli_root.join(".system_generated/tasks"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R142,
        );
        push_dir(
            &mut targets,
            agy_cli_root.join(".system_generated/logs"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R142,
        );
    }

    for agy_desktop_root in antigravity_desktop_roots(&home) {
        push_dir(
            &mut targets,
            agy_desktop_root.join("CachedData"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R143,
        );
        push_dir(
            &mut targets,
            agy_desktop_root.join("GPUCache"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R143,
        );
        push_dir(
            &mut targets,
            agy_desktop_root.join("Code Cache"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R143,
        );
        push_dir(
            &mut targets,
            agy_desktop_root.join("logs"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R143,
        );
        push_dir(
            &mut targets,
            agy_desktop_root.join("User/workspaceStorage"),
            RiskLevel::Caution,
            CleanupCategory::AgentSession,
            RuleDescription::R144,
        );
        push_dir(
            &mut targets,
            agy_desktop_root.join("User/globalStorage"),
            RiskLevel::Caution,
            CleanupCategory::AgentSession,
            RuleDescription::R144,
        );
    }

    for dsh_root in dsh_home_paths(&home) {
        push_dir(
            &mut targets,
            dsh_root.join("sessions"),
            RiskLevel::Caution,
            CleanupCategory::AgentSession,
            RuleDescription::R145,
        );
        push_dir(
            &mut targets,
            dsh_root.join("subagent-gc-backup"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R146,
        );
        push_dir(
            &mut targets,
            dsh_root.join("storages"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R147,
        );
        push_dir(
            &mut targets,
            dsh_root.join("attachments/v1/tmp"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R147,
        );
    }

    for dsh_desktop_root in dsh_desktop_data_roots(&home) {
        push_dir(
            &mut targets,
            dsh_desktop_root.join("Cache"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R148,
        );
        push_dir(
            &mut targets,
            dsh_desktop_root.join("Code Cache"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R148,
        );
        push_dir(
            &mut targets,
            dsh_desktop_root.join("GPUCache"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R148,
        );
        push_dir(
            &mut targets,
            dsh_desktop_root.join("ShaderCache"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R148,
        );
        push_dir(
            &mut targets,
            dsh_desktop_root.join("logs"),
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R148,
        );
    }

    for updater_root in dsh_updater_roots(&home) {
        push_dir(
            &mut targets,
            updater_root,
            RiskLevel::Safe,
            CleanupCategory::AgentCache,
            RuleDescription::R149,
        );
    }

    targets
}

fn push_dir(
    targets: &mut Vec<AgentSessionTarget>,
    path: PathBuf,
    risk: RiskLevel,
    category: CleanupCategory,
    description: RuleDescription,
) {
    if !path.exists() {
        return;
    }
    targets.push(AgentSessionTarget {
        path,
        stack: TechStack::Agent,
        risk,
        category,
        description,
    });
}

fn parse_env_paths(var: &str) -> Vec<PathBuf> {
    env::var(var)
        .ok()
        .map(|raw| {
            raw.split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(PathBuf::from)
                .collect()
        })
        .unwrap_or_default()
}

fn codex_home_paths(home: &Path) -> Vec<PathBuf> {
    let mut paths = parse_env_paths("CODEX_HOME");
    if paths.is_empty() {
        paths.push(home.join(".codex"));
    }
    paths
}

fn claude_config_paths(home: &Path) -> Vec<PathBuf> {
    let mut paths = parse_env_paths("CLAUDE_CONFIG_DIR");
    if paths.is_empty() {
        paths.push(home.join(".claude"));
        #[cfg(not(target_os = "windows"))]
        {
            paths.push(home.join(".config/claude"));
        }
        #[cfg(target_os = "windows")]
        {
            paths.push(home.join("AppData/Roaming/Claude"));
        }
    }
    paths
}

fn cursor_data_roots(home: &Path) -> Vec<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        vec![home.join("Library/Application Support/Cursor")]
    }
    #[cfg(target_os = "windows")]
    {
        vec![
            home.join("AppData/Roaming/Cursor"),
            home.join("AppData/Local/Programs/cursor"),
        ]
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        vec![home.join(".config/Cursor")]
    }
}

fn windsurf_data_roots(home: &Path) -> Vec<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        vec![home.join("Library/Application Support/Windsurf")]
    }
    #[cfg(target_os = "windows")]
    {
        vec![home.join("AppData/Roaming/Windsurf")]
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        vec![home.join(".config/Windsurf")]
    }
}

fn workbuddy_data_roots(home: &Path) -> Vec<PathBuf> {
    let mut roots = parse_env_paths("CODEBUDDY_HOME");
    if roots.is_empty() {
        roots.push(home.join(".codebuddy"));
    }
    roots
}

const TRAE_APP_NAMES: &[&str] = &["Trae", "Trae CN", "TRAE SOLO CN"];

fn trae_app_data_roots(home: &Path) -> Vec<PathBuf> {
    let roots = parse_env_paths("TRAE_DIR");
    if !roots.is_empty() {
        return roots
            .into_iter()
            .map(|user_dir| {
                user_dir
                    .parent()
                    .map(Path::to_path_buf)
                    .unwrap_or(user_dir)
            })
            .collect();
    }

    TRAE_APP_NAMES
        .iter()
        .map(|name| trae_app_data_root(home, name))
        .collect()
}

fn trae_app_data_root(home: &Path, name: &str) -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        home.join("Library/Application Support").join(name)
    }
    #[cfg(target_os = "windows")]
    {
        home.join("AppData/Roaming").join(name)
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        home.join(".config").join(name)
    }
}

fn traex_session_roots(home: &Path) -> Vec<PathBuf> {
    let mut roots = parse_env_paths("TRAEX_SESSIONS_DIR");
    if roots.is_empty() {
        roots.push(home.join(".trae/cli"));
    } else {
        roots = roots
            .into_iter()
            .map(|path| {
                if path.ends_with("sessions") || path.ends_with("archived_sessions") {
                    path.parent()
                        .map(Path::to_path_buf)
                        .unwrap_or(path)
                } else {
                    path
                }
            })
            .collect();
    }
    roots
}

fn opencode_data_roots(home: &Path) -> Vec<PathBuf> {
    let mut paths = parse_env_paths("OPENCODE_DIR");
    if paths.is_empty() {
        paths.push(home.join(".local/share/opencode"));
        #[cfg(not(target_os = "windows"))]
        if let Ok(xdg) = env::var("XDG_DATA_HOME") {
            paths.push(PathBuf::from(xdg).join("opencode"));
        }
        #[cfg(target_os = "windows")]
        {
            if let Ok(local) = env::var("LOCALAPPDATA") {
                paths.push(PathBuf::from(local).join("opencode"));
            }
            if let Ok(appdata) = env::var("APPDATA") {
                paths.push(PathBuf::from(appdata).join("opencode"));
            }
        }
    }
    paths
}

fn antigravity_cli_roots(home: &Path) -> Vec<PathBuf> {
    let mut paths = parse_env_paths("ANTIGRAVITY_APP_DATA_DIR");
    if paths.is_empty() {
        paths.push(home.join(".gemini/antigravity-cli"));
        #[cfg(target_os = "windows")]
        {
            if let Ok(appdata) = env::var("APPDATA") {
                paths.push(PathBuf::from(appdata).join(".gemini/antigravity-cli"));
            }
        }
    }
    paths
}

fn antigravity_desktop_roots(_home: &Path) -> Vec<PathBuf> {
    let mut roots = parse_env_paths("ANTIGRAVITY_DESKTOP_DIR");
    if roots.is_empty() {
        #[cfg(target_os = "windows")]
        {
            if let Ok(appdata) = env::var("APPDATA") {
                roots.push(PathBuf::from(&appdata).join("Antigravity"));
                roots.push(PathBuf::from(&appdata).join("Google/Antigravity"));
            }
            if let Ok(local) = env::var("LOCALAPPDATA") {
                roots.push(PathBuf::from(&local).join("Programs/Antigravity"));
            }
        }
        #[cfg(target_os = "macos")]
        {
            roots.push(home.join("Library/Application Support/Antigravity"));
            roots.push(home.join("Library/Application Support/Google/Antigravity"));
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            roots.push(home.join(".config/Antigravity"));
            roots.push(home.join(".config/antigravity"));
        }
    }
    roots
}

fn dsh_home_paths(home: &Path) -> Vec<PathBuf> {
    let mut paths = parse_env_paths("DSH_HOME");
    if paths.is_empty() {
        paths.push(home.join(".dsh"));
    }
    paths
}

fn dsh_desktop_data_roots(_home: &Path) -> Vec<PathBuf> {
    let mut roots = parse_env_paths("DSH_DESKTOP_DIR");
    if roots.is_empty() {
        #[cfg(target_os = "windows")]
        {
            if let Ok(appdata) = env::var("APPDATA") {
                roots.push(PathBuf::from(&appdata).join("@deepseek-ai/dsh-desktop"));
                roots.push(PathBuf::from(&appdata).join("deepseek-harness"));
            }
        }
        #[cfg(target_os = "macos")]
        {
            roots.push(_home.join("Library/Application Support/@deepseek-ai/dsh-desktop"));
            roots.push(_home.join("Library/Application Support/deepseek-harness"));
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            roots.push(_home.join(".config/@deepseek-ai/dsh-desktop"));
            roots.push(_home.join(".config/deepseek-harness"));
        }
    }
    roots
}

fn dsh_updater_roots(_home: &Path) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    #[cfg(target_os = "windows")]
    {
        if let Ok(local) = env::var("LOCALAPPDATA") {
            roots.push(PathBuf::from(&local).join("@deepseek-aidsh-desktop-updater"));
            roots.push(PathBuf::from(&local).join("deepseek-harness-updater"));
        }
    }
    #[cfg(target_os = "macos")]
    {
        roots.push(_home.join("Library/Caches/@deepseek-ai/dsh-desktop-updater"));
    }
    roots
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codex_home_respects_env() {
        let temp = tempfile::tempdir().unwrap();
        let custom = temp.path().join("custom-codex");
        std::fs::create_dir_all(&custom).unwrap();
        unsafe {
            env::set_var("CODEX_HOME", &custom);
        }
        let paths = codex_home_paths(temp.path());
        assert_eq!(paths, vec![custom]);
        unsafe {
            env::remove_var("CODEX_HOME");
        }
    }

    #[test]
    fn claude_defaults_include_home_dot_claude() {
        let temp = tempfile::tempdir().unwrap();
        let paths = claude_config_paths(temp.path());
        assert!(paths.iter().any(|p| p.ends_with(".claude")));
    }

    #[test]
    fn trae_dir_respects_env() {
        let temp = tempfile::tempdir().unwrap();
        let custom_user = temp.path().join("custom-trae/User");
        std::fs::create_dir_all(&custom_user).unwrap();
        unsafe {
            env::set_var("TRAE_DIR", &custom_user);
        }
        let paths = trae_app_data_roots(temp.path());
        assert_eq!(paths, vec![custom_user.parent().unwrap().to_path_buf()]);
        unsafe {
            env::remove_var("TRAE_DIR");
        }
    }

    #[test]
    fn opencode_dir_respects_env() {
        let temp = tempfile::tempdir().unwrap();
        let custom = temp.path().join("custom-opencode");
        std::fs::create_dir_all(&custom).unwrap();
        unsafe {
            env::set_var("OPENCODE_DIR", &custom);
        }
        let paths = opencode_data_roots(temp.path());
        assert_eq!(paths, vec![custom]);
        unsafe {
            env::remove_var("OPENCODE_DIR");
        }
    }

    #[test]
    fn antigravity_cli_respects_env() {
        let temp = tempfile::tempdir().unwrap();
        let custom = temp.path().join("custom-agy");
        std::fs::create_dir_all(&custom).unwrap();
        unsafe {
            env::set_var("ANTIGRAVITY_APP_DATA_DIR", &custom);
        }
        let paths = antigravity_cli_roots(temp.path());
        assert_eq!(paths, vec![custom]);
        unsafe {
            env::remove_var("ANTIGRAVITY_APP_DATA_DIR");
        }
    }

    #[test]
    fn antigravity_cli_defaults_to_gemini_home() {
        let temp = tempfile::tempdir().unwrap();
        let paths = antigravity_cli_roots(temp.path());
        assert!(paths.iter().any(|p| p.ends_with(".gemini/antigravity-cli")));
    }
}
