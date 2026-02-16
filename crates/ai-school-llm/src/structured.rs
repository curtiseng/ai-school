use ai_school_core::error::LlmError;

/// 从 LLM 响应中提取 JSON 内容
///
/// 处理可能的 markdown code block 包裹
pub fn extract_json(content: &str) -> Result<String, LlmError> {
    let trimmed = content.trim();

    // Try to extract from markdown code block
    if let Some(start) = trimmed.find("```json") {
        let after_marker = &trimmed[start + 7..];
        if let Some(end) = after_marker.find("```") {
            return Ok(after_marker[..end].trim().to_string());
        }
    }

    // Try to extract from generic code block
    if let Some(start) = trimmed.find("```") {
        let after_marker = &trimmed[start + 3..];
        // Skip language identifier on the same line
        let content_start = after_marker.find('\n').unwrap_or(0);
        let after_newline = &after_marker[content_start..];
        if let Some(end) = after_newline.find("```") {
            return Ok(after_newline[..end].trim().to_string());
        }
    }

    // Try to find raw JSON object or array
    if let Some(start) = trimmed.find('{') {
        if let Some(end) = trimmed.rfind('}') {
            if end > start {
                return Ok(trimmed[start..=end].to_string());
            }
        }
    }

    if let Some(start) = trimmed.find('[') {
        if let Some(end) = trimmed.rfind(']') {
            if end > start {
                return Ok(trimmed[start..=end].to_string());
            }
        }
    }

    Err(LlmError::ParseError(format!(
        "Could not extract JSON from response: {}",
        &trimmed[..trimmed.len().min(200)]
    )))
}

/// 验证 JSON 字符串是否符合 JSON Schema
pub fn validate_json(json_str: &str, schema: &serde_json::Value) -> Result<(), LlmError> {
    let instance: serde_json::Value =
        serde_json::from_str(json_str).map_err(|e| LlmError::ParseError(e.to_string()))?;

    let compiled = jsonschema::validator_for(schema)
        .map_err(|e| LlmError::SchemaValidation(format!("Invalid schema: {e}")))?;

    if let Err(error) = compiled.validate(&instance) {
        return Err(LlmError::SchemaValidation(error.to_string()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_from_code_block() {
        let input = r#"```json
{"key": "value"}
```"#;
        let result = extract_json(input).unwrap();
        assert_eq!(result, r#"{"key": "value"}"#);
    }

    #[test]
    fn test_extract_json_raw() {
        let input = r#"Here is the result: {"key": "value"}"#;
        let result = extract_json(input).unwrap();
        assert_eq!(result, r#"{"key": "value"}"#);
    }

    #[test]
    fn test_extract_json_pure() {
        let input = r#"{"key": "value"}"#;
        let result = extract_json(input).unwrap();
        assert_eq!(result, r#"{"key": "value"}"#);
    }
}
