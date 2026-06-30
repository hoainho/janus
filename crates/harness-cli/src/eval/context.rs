use std::collections::HashMap;

use super::case::{ContextFile, EvalCase};

pub fn inject_context(case: &EvalCase) -> String {
    let context = match &case.context {
        Some(ctx) => ctx,
        None => return case.prompt.clone(),
    };

    let mut prompt = String::new();

    if let Some(window) = &context.window {
        prompt.push_str("## Context\n\n");
        prompt.push_str(window);
        prompt.push_str("\n\n");
    }

    if let Some(files) = &context.files {
        prompt.push_str("## Files\n\n");
        for file in files {
            prompt.push_str(&format!("### {}\n", file.path));
            prompt.push_str("```\n");
            prompt.push_str(&file.content);
            prompt.push_str("\n```\n\n");
        }
    }

    prompt.push_str("## Task\n\n");
    prompt.push_str(&case.prompt);

    if let Some(variables) = &context.variables {
        prompt = replace_variables(&prompt, variables);
    }

    prompt
}

fn replace_variables(template: &str, variables: &HashMap<String, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in variables {
        let placeholder = format!("{{{{{}}}}}", key);
        result = result.replace(&placeholder, value);
    }
    result
}

pub fn build_context_window(window: &str, files: &[ContextFile]) -> String {
    let mut context = String::new();

    context.push_str(window);
    context.push_str("\n\n");

    for file in files {
        context.push_str(&format!("--- {} ---\n", file.path));
        context.push_str(&file.content);
        context.push_str("\n---\n\n");
    }

    context
}

pub fn validate_context(case: &EvalCase) -> Result<(), String> {
    let context = match &case.context {
        Some(ctx) => ctx,
        None => return Ok(()),
    };

    if let Some(files) = &context.files {
        for file in files {
            if file.path.is_empty() {
                return Err("Context file path cannot be empty".to_string());
            }
            if file.content.is_empty() {
                return Err(format!(
                    "Context file '{}' content cannot be empty",
                    file.path
                ));
            }
        }
    }

    Ok(())
}
