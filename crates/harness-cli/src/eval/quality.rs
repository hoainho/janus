pub struct QualityScore {
    pub prompt_score: f64,
    pub response_score: f64,
    pub context_score: f64,
    pub overall_score: f64,
    pub feedback: String,
}

pub fn score_prompt_quality(prompt: &str) -> f64 {
    let word_count = prompt.split_whitespace().count();
    let has_question = prompt.contains('?');
    let has_context = prompt.len() > 100;
    let has_specificity =
        prompt.contains("specific") || prompt.contains("exact") || prompt.contains("particular");

    let mut score: f64 = 0.0;

    if word_count >= 10 {
        score += 25.0;
    }
    if word_count >= 25 {
        score += 15.0;
    }
    if has_question {
        score += 20.0;
    }
    if has_context {
        score += 25.0;
    }
    if has_specificity {
        score += 15.0;
    }

    score.min(100.0)
}

pub fn score_response_quality(response: &str) -> f64 {
    let word_count = response.split_whitespace().count();
    let has_code = response.contains("```");
    let has_examples = response.contains("example") || response.contains("e.g.");
    let has_structure =
        response.contains("\n-") || response.contains("\n*") || response.contains("\n1.");
    let has_conclusion = response.contains("summary")
        || response.contains("conclusion")
        || response.contains("in summary");

    let mut score: f64 = 0.0;

    if word_count >= 50 {
        score += 20.0;
    }
    if word_count >= 100 {
        score += 15.0;
    }
    if word_count >= 200 {
        score += 10.0;
    }
    if has_code {
        score += 15.0;
    }
    if has_examples {
        score += 15.0;
    }
    if has_structure {
        score += 15.0;
    }
    if has_conclusion {
        score += 10.0;
    }

    score.min(100.0)
}

pub fn score_context_relevance(response: &str, context: &str) -> f64 {
    let context_words: Vec<&str> = context.split_whitespace().collect();
    let response_words: Vec<&str> = response.split_whitespace().collect();

    let mut match_count = 0;
    for word in &context_words {
        if word.len() > 3 && response_words.contains(word) {
            match_count += 1;
        }
    }

    let relevance = if context_words.is_empty() {
        0.0
    } else {
        (match_count as f64 / context_words.len() as f64) * 100.0
    };

    relevance.min(100.0)
}

pub fn compute_quality_score(prompt: &str, response: &str, context: &str) -> QualityScore {
    let prompt_score = score_prompt_quality(prompt);
    let response_score = score_response_quality(response);
    let context_score = score_context_relevance(response, context);

    let overall_score = (prompt_score * 0.2) + (response_score * 0.4) + (context_score * 0.4);

    let feedback = generate_feedback(prompt_score, response_score, context_score);

    QualityScore {
        prompt_score,
        response_score,
        context_score,
        overall_score,
        feedback,
    }
}

fn generate_feedback(prompt_score: f64, response_score: f64, context_score: f64) -> String {
    let mut feedback = Vec::new();

    if prompt_score < 50.0 {
        feedback.push("Prompt is too vague. Add more specific questions and context.");
    }
    if response_score < 50.0 {
        feedback.push("Response lacks detail. Provide more examples and structure.");
    }
    if context_score < 50.0 {
        feedback.push("Response is not relevant to the context. Stay on topic.");
    }

    if feedback.is_empty() {
        "Quality is good. No major issues found.".to_string()
    } else {
        feedback.join(" ")
    }
}
