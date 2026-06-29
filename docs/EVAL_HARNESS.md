# EVAL-HARNESS: Pre-Check Gate

**Purpose**: Evaluate harness readiness BEFORE any process starts. Block if critical info missing. Ask Product (user) to fill gaps.

## Core Principle

> "A harness that starts without readiness is theater, not engineering."

Every process MUST pass EVAL-HARNESS before intake. No exceptions.

---

## When EVAL-HARNESS Runs

```text
User Request
     │
     ▼
┌─────────────────────────────────┐
│  EVAL-HARNESS Pre-Check        │ ← YOU ARE HERE
│  • Is request clear enough?    │
│  • Do we have enough context?  │
│  • Are prerequisites met?      │
│  • What questions remain?      │
└────────────────┬────────────────┘
                 │
                 ├── BLOCKED ──► Ask questions → Wait for answers → Re-eval
                 │
                 ▼ READY
┌─────────────────────────────────┐
│  Feature Intake                │
└─────────────────────────────────┘
```

---

## Readiness Checklist

### Level 1: Request Clarity (MUST PASS)

| # | Check | Pass When | Fail Action |
|---|-------|-----------|-------------|
| 1.1 | **Goal stated** | User described what they want | Ask: "What is the goal of this change?" |
| 1.2 | **Scope bounded** | Clear what's in/out of scope | Ask: "What should this change NOT do?" |
| 1.3 | **Success defined** | Know when it's "done" | Ask: "How will we know this is complete?" |
| 1.4 | **Surface identified** | Know which code/docs change | Ask: "Which files/modules will be affected?" |

### Level 2: Context Availability (MUST PASS)

| # | Check | Pass When | Fail Action |
|---|-------|-----------|-------------|
| 2.1 | **Existing code read** | Agent has read relevant files | Agent reads files |
| 2.2 | **Patterns known** | Understand current conventions | Agent explores patterns |
| 2.3 | **Dependencies mapped** | Know what this touches | Ask: "What does this depend on?" |
| 2.4 | **Constraints listed** | Know limitations | Ask: "Any constraints I should know?" |

### Level 3: Harness State (MUST PASS)

| # | Check | Pass When | Fail Action |
|---|-------|-----------|-------------|
| 3.1 | **DB initialized** | `harness.db` exists | Run `harness-cli init` |
| 3.2 | **Stories checked** | No conflicting work | Run `harness-cli query stories` |
| 3.3 | **Decisions checked** | Know past choices | Run `harness-cli query decisions` |
| 3.4 | **Matrix current** | Validation expectations set | Run `harness-cli query matrix` |

### Level 4: Product Readiness (MUST PASS for normal/high-risk)

| # | Check | Pass When | Fail Action |
|---|-------|-----------|-------------|
| 4.1 | **Acceptance criteria** | Clear "done" conditions | Ask: "What must be true when this is done?" |
| 4.2 | **Edge cases listed** | Know failure modes | Ask: "What could go wrong?" |
| 4.3 | **User flow defined** | Know user's path | Ask: "Walk me through the user journey" |
| 4.4 | **Priority confirmed** | Know if urgent | Ask: "How urgent is this? (low/medium/high)" |

---

## Question Templates

### For Unclear Goals

```
I want to make sure I understand correctly.

**What I understood**: [Your interpretation]
**What I'm unsure about**: [Specific ambiguity]

**Questions**:
1. [Specific question about goal]
2. [Specific question about scope]
3. [Specific question about success criteria]

Please clarify before I proceed.
```

### For Missing Context

```
I need more context to proceed safely.

**What I know**: [Current understanding]
**What I'm missing**: [Gaps]

**Questions**:
1. [Question about existing behavior]
2. [Question about constraints]
3. [Question about dependencies]

This will help me avoid mistakes.
```

### For Harness State Issues

```
The harness isn't ready for this work.

**Issues found**:
- [ ] [Issue 1]
- [ ] [Issue 2]

**Questions**:
1. Should I fix these first?
2. Or should we proceed without full harness?

Note: Proceeding without harness increases risk.
```

### For Product Readiness

```
I need to understand the "done" state better.

**What I know**: [Current understanding]
**What I'm unsure about**: [Gaps]

**Questions**:
1. What are the acceptance criteria?
2. What edge cases should I handle?
3. What's the priority level?

This ensures I build the right thing.
```

---

## Readiness Score

After checklist, compute:

```text
Score = (Passed Checks / Total Checks) × 100

Readiness Levels:
  90-100%: READY — proceed to intake
  70-89%:  PARTIAL — ask questions, then proceed
  50-69%:  BLOCKED — must fill gaps first
  <50%:    NOT READY — too many unknowns
```

---

## Output Format

EVAL-HARNESS produces:

```markdown
## EVAL-HARNESS Result

**Readiness**: [READY/PARTIAL/BLOCKED/NOT_READY]
**Score**: [X]%

### Passed
- [x] [Check 1]
- [x] [Check 2]

### Failed
- [ ] [Check 3] — [Question to ask]
- [ ] [Check 4] — [Question to ask]

### Questions for Product
1. [Question 1]
2. [Question 2]
3. [Question 3]

### Recommendation
[What to do next]
```

---

## Integration with AGENTS.md

Add to agent entrypoint:

```markdown
## Before Any Work

1. Run EVAL-HARNESS checklist
2. If < 90% ready, ask questions
3. Wait for answers
4. Re-eval until ready
5. Only then proceed to intake
```

---

## CLI Integration (Future)

```bash
# Check readiness
harness-cli eval-harness --request "Add OAuth login"

# Output
Readiness: PARTIAL (75%)
Questions:
  - What OAuth providers? (Google, GitHub, etc.)
  - Session duration?
  - Refresh token strategy?
```

---

## Anti-Patterns

❌ **Starting without clarity**: "I'll figure it out as I go"
❌ **Assuming context**: "The user probably means X"
❌ **Skipping questions**: "I don't want to bother them"
❌ **Ignoring harness state**: "The DB doesn't matter"

✅ **Ask first**: Better to ask 3 questions than build wrong thing
✅ **Verify assumptions**: State what you think, ask if correct
✅ **Check harness**: Past decisions inform present work
✅ **Document gaps**: Record what's missing for future

---

## Summary

EVAL-HARNESS is the **quality gate before the quality gates**. It ensures:

1. **We understand the request** (clarity)
2. **We have enough context** (information)
3. **The harness is ready** (state)
4. **Product is ready** (criteria)

Only when all 4 are true do we proceed.

**Better to ask questions upfront than fix mistakes later.**
