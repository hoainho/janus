use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JUnitTestSuite {
    pub name: String,
    pub tests: usize,
    pub failures: usize,
    pub errors: usize,
    pub time: f64,
    pub testcases: Vec<JUnitTestCase>,
}

#[derive(Debug, Serialize)]
pub struct JUnitTestCase {
    pub name: String,
    pub classname: String,
    pub time: f64,
    pub failure: Option<JUnitFailure>,
}

#[derive(Debug, Serialize)]
pub struct JUnitFailure {
    pub message: String,
    #[serde(rename = "type")]
    pub failure_type: String,
}

pub fn render_junit(summary: &crate::eval::stats::RunSummary) -> String {
    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str(&format!(
        "<testsuite name=\"{}\" tests=\"{}\" failures=\"{}\" errors=\"0\" time=\"0\">\n",
        summary.skill, summary.total, summary.regression_count
    ));

    for regression in &summary.regressions {
        xml.push_str(&format!(
            "  <testcase name=\"{}\" classname=\"{}\" time=\"0\">\n",
            regression, summary.skill
        ));
        xml.push_str(&format!(
            "    <failure message=\"Regression detected\" type=\"REGRESSION\">{}</failure>\n",
            regression
        ));
        xml.push_str("  </testcase>\n");
    }

    xml.push_str("</testsuite>\n");
    xml
}

pub fn render_sarif(summary: &crate::eval::stats::RunSummary, version: &str) -> String {
    let sarif = serde_json::json!({
        "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
        "version": "2.1.0",
        "runs": [{
            "tool": {
                "driver": {
                    "name": "eval-harness",
                    "version": version,
                    "informationUri": "https://github.com/nano-step/eval-harness"
                }
            },
            "results": summary.regressions.iter().map(|regression| {
                serde_json::json!({
                    "ruleId": "eval-regression",
                    "level": "error",
                    "message": {
                        "text": format!("Regression detected in case: {}", regression)
                    },
                    "locations": [{
                        "physicalLocation": {
                            "artifactLocation": {
                                "uri": format!("evals/cases/{}.yaml", regression)
                            }
                        }
                    }]
                })
            }).collect::<Vec<_>>(),
            "invocations": [{
                "executionSuccessful": summary.verdict == "PASS",
                "toolExecutionNotifications": []
            }]
        }]
    });

    serde_json::to_string_pretty(&sarif).unwrap_or_default()
}
