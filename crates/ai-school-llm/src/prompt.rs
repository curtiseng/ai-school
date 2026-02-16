use minijinja::Environment;
use std::path::Path;
use tracing::debug;

use ai_school_core::error::LlmError;

/// Prompt 模板引擎
///
/// 基于 minijinja 加载和渲染 .j2 模板文件。
pub struct PromptEngine {
    env: Environment<'static>,
}

impl PromptEngine {
    /// 从指定目录加载所有 .j2 模板
    pub fn from_directory(prompts_dir: &Path) -> Result<Self, LlmError> {
        let mut env = Environment::new();

        // Recursively load all .j2 files
        Self::load_templates_recursive(&mut env, prompts_dir, prompts_dir)?;

        debug!(dir = ?prompts_dir, "Prompt templates loaded");

        Ok(Self { env })
    }

    fn load_templates_recursive(
        env: &mut Environment<'static>,
        base_dir: &Path,
        current_dir: &Path,
    ) -> Result<(), LlmError> {
        if !current_dir.exists() {
            return Ok(());
        }

        let entries = std::fs::read_dir(current_dir)
            .map_err(|e| LlmError::PromptError(format!("Failed to read directory: {e}")))?;

        for entry in entries {
            let entry =
                entry.map_err(|e| LlmError::PromptError(format!("Failed to read entry: {e}")))?;
            let path = entry.path();

            if path.is_dir() {
                Self::load_templates_recursive(env, base_dir, &path)?;
            } else if path.extension().is_some_and(|ext| ext == "j2") {
                let relative = path
                    .strip_prefix(base_dir)
                    .map_err(|e| LlmError::PromptError(e.to_string()))?;

                let template_name = relative
                    .to_str()
                    .ok_or_else(|| LlmError::PromptError("Invalid path".to_string()))?
                    .replace('\\', "/")
                    .trim_end_matches(".j2")
                    .to_string();

                let content = std::fs::read_to_string(&path).map_err(|e| {
                    LlmError::PromptError(format!("Failed to read template {}: {e}", path.display()))
                })?;

                env.add_template_owned(template_name.clone(), content)
                    .map_err(|e| {
                        LlmError::PromptError(format!(
                            "Failed to compile template {template_name}: {e}"
                        ))
                    })?;

                debug!(template = %template_name, "Loaded prompt template");
            }
        }

        Ok(())
    }

    /// 渲染指定模板
    pub fn render(
        &self,
        template_name: &str,
        context: &serde_json::Value,
    ) -> Result<String, LlmError> {
        let template = self
            .env
            .get_template(template_name)
            .map_err(|e| LlmError::PromptError(format!("Template '{template_name}' not found: {e}")))?;

        template
            .render(context)
            .map_err(|e| LlmError::PromptError(format!("Failed to render '{template_name}': {e}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_prompt_engine() {
        let dir = tempdir().unwrap();
        let agent_dir = dir.path().join("agent");
        fs::create_dir_all(&agent_dir).unwrap();
        fs::write(
            agent_dir.join("decision.j2"),
            "你是{{ name }}，性格类型是{{ mbti }}。\n当前情境：{{ situation }}",
        )
        .unwrap();

        let engine = PromptEngine::from_directory(dir.path()).unwrap();
        let context = serde_json::json!({
            "name": "小明",
            "mbti": "INTJ",
            "situation": "数学课上被分组"
        });

        let result = engine.render("agent/decision", &context).unwrap();
        assert!(result.contains("小明"));
        assert!(result.contains("INTJ"));
    }
}
