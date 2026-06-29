# EVAL-HARNESS: Pre-Check Gate

**Purpose**: Prevent failures BEFORE they happen. Based on research from:
- Columbia University: 9 Critical Failure Patterns of Coding Agents
- Microsoft: Taxonomy of Failure Mode in Agentic AI Systems
- Industry: CI/CD Quality Gates best practices
- Senior Engineer patterns: What experts ask before coding

---

## Core Principle

> "Quality gates are binary. A gate either passes or blocks. Warning states undermine the system." — Industry standard

> "Most agent failures are scaffold failures: things the agent never sees, never replays, or never bounds." — Research finding

---

## When EVAL-HARNESS Runs

```text
User Request
     │
     ▼
┌─────────────────────────────────┐
│  EVAL-HARNESS Pre-Check        │ ← BLOCK if not ready
│  • Failure Mode Prevention     │
│  • Risk Assessment             │
│  • Context Verification        │
│  • Success Criteria            │
└────────────────┬────────────────┘
                 │
                 ├── BLOCKED → Ask questions → Wait → Re-eval
                 │
                 ▼ READY
┌─────────────────────────────────┐
│  Feature Intake                │
└─────────────────────────────────┘
```

---

## Risk Tiers

| Tier | When | Questions Required | Time Budget |
|------|------|-------------------|-------------|
| **Tiny** | 0-1 risk flags | 4 MUST | < 1 min |
| **Normal** | 2-3 risk flags | 8 MUST | < 3 min |
| **High-Risk** | 4+ flags or hard gate | 12 MUST + context check | < 5 min |

---

## MUST Ask Questions (Based on Failure Modes)

### Tier 1: Request Clarity (ALL tiers)

These prevent **Business Logic Mismatch** (Failure Mode #3 from research):

| # | Question | Why It Matters | Pass When |
|---|----------|----------------|-----------|
| 1.1 | **What problem are we solving?** | Prevents building wrong thing | Clear problem statement |
| 1.2 | **Who is impacted?** | Prevents wrong assumptions about users | Specific persona/role identified |
| 1.3 | **What does success look like?** | Prevents "done" ambiguity | Measurable criteria defined |
| 1.4 | **What is explicitly OUT of scope?** | Prevents scope creep | Boundaries stated |

**Fail Action**: Ask these 4 questions. Do NOT proceed without answers.

---

### Tier 2: Context Verification (Normal + High-Risk)

These prevent **Codebase Awareness Issues** (Failure Mode #8 from research):

| # | Question | Why It Matters | Pass When |
|---|----------|----------------|-----------|
| 2.1 | **Have we done this before?** | Prevents reinventing wheel | Past work checked |
| 2.2 | **What existing patterns apply?** | Prevents architecture drift | Patterns identified |
| 2.3 | **What depends on this?** | Prevents breaking downstream | Dependencies mapped |
| 2.4 | **What constraints exist?** | Prevents impossible solutions | Constraints listed |

**Fail Action**: Agent must explore codebase before answering.

---

### Tier 3: Risk Assessment (High-Risk only)

These prevent **Security Vulnerabilities** (Failure Mode #6 from research):

| # | Question | Why It Matters | Pass When |
|---|----------|----------------|-----------|
| 3.1 | **What could go wrong?** | Prevents blind spots | Failure modes listed |
| 3.2 | **What's the blast radius?** | Prevents cascading failures | Impact bounded |
| 3.3 | **How do we rollback?** | Prevents stuck states | Rollback path exists |
| 3.4 | **What data is affected?** | Prevents data corruption | Data impact assessed |

**Fail Action**: Must have rollback plan before proceeding.

---

### Tier 4: Success Criteria (ALL tiers)

These prevent **Testing Illusion** (Failure Mode #10 from research):

| # | Question | Why It Matters | Pass When |
|---|----------|----------------|-----------|
| 4.1 | **How will we verify this works?** | Prevents "looks right" trap | Test plan exists |
| 4.2 | **What are the edge cases?** | Prevents hidden bugs | Edge cases listed |
| 4.3 | **What's the acceptance criteria?** | Prevents partial delivery | Criteria defined |
| 4.4 | **How will we know it's done?** | Prevents endless iteration | Done state clear |

**Fail Action**: Define verification method before coding.

---

## Failure Mode Prevention Matrix

Based on research from Columbia University and Microsoft:

| Failure Mode | Prevention Question | Gate |
|--------------|---------------------|------|
| **Business Logic Mismatch** | "What problem are we solving?" | 1.1 |
| **State Management Failures** | "What state changes occur?" | 2.3 |
| **Security Vulnerabilities** | "What data is affected?" | 3.4 |
| **Codebase Awareness Issues** | "Have we done this before?" | 2.1 |
| **API Integration Failures** | "What depends on this?" | 2.3 |
| **Data Management Errors** | "What data is affected?" | 3.4 |
| **Exception Handling** | "What could go wrong?" | 3.1 |
| **Repeated Code** | "What existing patterns apply?" | 2.2 |
| **Presentation Mismatch** | "What does success look like?" | 1.3 |

---

## Question Templates

### For Unclear Requests (Gate 1.x fails)

```
I need clarity before proceeding.

**What I understood**: [Your interpretation]
**What I'm unsure about**: [Specific gap]

**Please answer**:
1. What problem are we solving?
2. Who is impacted?
3. What does success look like?
4. What is out of scope?

Reply with answers or "proceed with assumptions" (I'll list them).
```

### For Missing Context (Gate 2.x fails)

```
I need to understand the codebase better.

**What I found**: [Current understanding]
**What I'm missing**: [Gaps]

**I'll check**:
- [ ] Existing patterns
- [ ] Past decisions
- [ ] Dependencies
- [ ] Constraints

Give me a moment to explore, or tell me if you have this context.
```

### For Risk Concerns (Gate 3.x fails)

```
This has risk factors that need attention.

**Risk flags**: [List]
**Blast radius**: [Assessment]

**Questions**:
1. What could go wrong?
2. How do we rollback?
3. What data is affected?

I need answers before proceeding with high-risk work.
```

### For Undefined Success (Gate 4.x fails)

```
I need to know when we're "done."

**What I know**: [Current understanding]
**What I'm unsure about**: [Gaps]

**Please define**:
1. How will we verify this works?
2. What are the edge cases?
3. What's the acceptance criteria?

Without this, I can't guarantee quality.
```

---

## Readiness Score

```text
Score = (Passed MUST checks / Total MUST checks) × 100

Readiness Levels:
  100%:    READY — proceed to intake
  75-99%:  PARTIAL — ask missing questions
  50-74%:  BLOCKED — must fill gaps
  <50%:    NOT READY — too many unknowns
```

**Rule**: NEVER proceed below 75% readiness for normal/high-risk work.

---

## Output Format

```markdown
## EVAL-HARNESS Result

**Tier**: [tiny/normal/high-risk]
**Readiness**: [READY/PARTIAL/BLOCKED]
**Score**: [X]%

### Passed
- [x] 1.1 Problem defined: "Add OAuth login"
- [x] 1.2 Users identified: "End users"
- [x] 1.3 Success criteria: "Users can login with Google"

### Failed
- [ ] 1.4 Out of scope: Not defined
- [ ] 4.1 Verification: No test plan

### Questions for Product
1. What is explicitly out of scope?
2. How will we verify this works?

### Risk Assessment
- Risk flags: 2 (Auth + Public Contracts)
- Lane: normal
- Gates required: All (1-9)

### Recommendation
Answer the 2 questions above, then proceed to intake.
```

---

## Integration with Workflow

```text
1. User request arrives
2. Run EVAL-HARNESS checklist
3. If score < 75%:
   - Ask missing questions
   - Wait for answers
   - Re-eval
4. If score ≥ 75%:
   - Proceed to Feature Intake
   - Record readiness in harness-cli
```

---

## CLI Integration

```bash
# Check readiness
harness-cli eval-harness --request "Add OAuth login"

# Output
Tier: normal
Readiness: PARTIAL (75%)
Questions:
  - What is out of scope?
  - How will we verify this works?

# Record readiness
harness-cli intake --type feature --summary "OAuth" --lane normal --readiness 75
```

---

## Anti-Patterns

❌ **Starting without clarity**: "I'll figure it out as I go"
❌ **Assuming context**: "The user probably means X"
❌ **Skipping questions**: "I don't want to bother them"
❌ **Soft gates**: "Warning" states that let code through

✅ **Ask first**: Better to ask 3 questions than build wrong thing
✅ **Binary gates**: Pass or fail, no warnings
✅ **Risk-tiered**: More questions for higher risk
✅ **Failure-mode based**: Questions prevent known failures

---

## Sources

- Columbia University: "9 Critical Failure Patterns of Coding Agents" (2026)
- Microsoft: "Taxonomy of Failure Mode in Agentic AI Systems"
- Industry: CI/CD Quality Gates best practices (InfoQ, Keploy, JetBrains)
- Senior Engineer patterns: Requirements gathering best practices
- Production Readiness: Checklist standards (Cortex, penguinboi/preflight-checks)

---

## Summary

EVAL-HARNESS is based on **real failure modes** from research, not assumptions.

Each question prevents a **specific, documented failure pattern**.

**Better to ask questions upfront than debug failures later.**
