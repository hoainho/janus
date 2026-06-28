# Feature Intake

Every implementation prompt enters the intake gate before code changes. A new
project spec also enters through this gate before it becomes product docs,
stories, or implementation work.

The human does not need to classify risk. The harness does.

## Intake Flow

```text
User prompt
    |
    v
Classify input type
    |
    v
Restate as work item
    |
    v
Find affected product docs and stories
    |
    v
Run risk checklist
    |
    v
Choose lane: tiny, normal, or high-risk
    |
    v
Record with: harness-cli intake
```

## Input Types

Use the input type to decide where the work should land before choosing the risk
lane.

| Type | Use when | Typical artifact |
| --- | --- | --- |
| New spec | Turning a user-provided project spec into harness-ready docs | Product docs, candidate epics, decisions |
| Spec slice | Implementing selected behavior from an accepted spec | Story packet |
| Change request | Changing, fixing, or refining accepted behavior | Story packet or direct patch |
| New initiative | Adding a larger product area that needs multiple stories | Initiative notes plus story packets |
| Maintenance request | Changing technical, operational, or dependency behavior | Story packet, validation report, or decision |
| Harness improvement | Improving how humans and agents collaborate | Direct docs update or `harness-cli backlog add` |

Do not create or extend a monolithic spec by default after intake. Use product
docs, stories, decisions, and initiative notes as the living surface.

## Lanes

### Tiny

Use for low-risk docs, copy, names, or narrow edits.

Also use for initial project setup when the work is limited to installing
declared dependencies, wiring a server entrypoint, adding a health/smoke
endpoint, or opening a local development database connection without creating
domain schema, CRUD behavior, auth, authorization, provider integration, or
data migration. A health endpoint in a new benchmark or scaffolded project is
smoke proof, not a public contract escalation by itself.

Requirements:

- Record the intake row before implementation:
  ```bash
  scripts/bin/harness-cli intake --type <type> --summary "<text>" --lane tiny
  ```
- Patch directly.
- Keep affected docs current.
- Run available quick checks.
- Update the harness only if friction was found.
- PR Bot Review still applies if pushing remotely.

---

### Normal

Use for story-sized behavior with bounded blast radius (1-3 risk flags).

Steps:

1. **Record intake**
   ```bash
   scripts/bin/harness-cli intake --type <type> --summary "<text>" --lane normal
   ```

2. **Propose**
   ```bash
   openspec new change "<kebab-name>"
   ```
   Write `proposal.md` and `design.md`.

3. **Deep-design gap analysis** *(if available)*
   ```
   /deep-design
   ```
   - If gaps found → revise artifacts → re-run deep-design.
   - Proceed only on clean pass.

4. **Generate specs + story packet**
   ```bash
   openspec instructions specs --change "<name>"
   openspec validate "<name>" --strict
   ```
   Create `docs/stories/<name>.md` from `docs/templates/story.md`.
   
   Record story:
   ```bash
   scripts/bin/harness-cli story add --id <id> --title "<text>" --lane normal
   ```
   
   Update `docs/TEST_MATRIX.md`.

5. **Implement**
   ```
   /opsx-apply
   ```
   Keep `make validate-quick` green on every commit.

6. **Validate**
   Run `validate:quick` + `test:integration`. Paste output in story Evidence.

7. **User-flow test** (skip if change type = infra/refactor/docs)
   Run at least 1 test through the user's entry point matching the changed
   surface (see HARNESS.md § User-Flow Testing). Paste command + output in
   story Evidence section.

8. **Review Gate** (skip if change type = infra/refactor/docs)
   Spawn a fresh review agent to verify each acceptance criterion against
   evidence. Reviewer ≠ implementer. Paste Review Verdict in story Evidence.
   Proceed only on PASS.
   
   Record review:
   ```bash
   scripts/bin/harness-cli intervention add --type review --description "Review verdict: PASS" --source agent
   ```

9. **PR + Bot Review Loop**
   Push branch, open PR. Address bot review comments (fix or
   reasoned reply). Re-run validate + user-flow + Review Gate if implementation
   changes. Loop until bot approves (max 3 push cycles → escalate to human).

10. **Close**
    ```bash
    openspec archive "<name>"
    ```
    
    Update story status:
    ```bash
    scripts/bin/harness-cli story update --id <id> --status done
    ```
    
    Update `docs/TEST_MATRIX.md` with evidence + Review Verdict.

---

### High-Risk

Use when the work can affect security, data, scope, contracts, or multiple
roles/platforms (4+ risk flags, or any hard gate).

Steps:

1. **Record intake**
   ```bash
   scripts/bin/harness-cli intake --type <type> --summary "<text>" --lane high-risk
   ```

2. **Propose** — same as Normal, plus fill design.md in full detail.

3. **Deep-design gap analysis** — mandatory; do not skip.
   - All blocking gaps must be resolved before proceeding.
   - Record architecture decisions in `docs/decisions/`.

4. **Human confirmation** — present synthesis to human; get explicit go-ahead
   before writing any spec.

5. **Generate specs + story folder**
   ```bash
   openspec instructions specs --change "<name>"
   openspec validate "<name>" --strict
   ```
   Create story folder from `docs/templates/high-risk-story/`.
   Fill `overview.md`, `design.md`, `execplan.md`, `validation.md`.
   
   Record story:
   ```bash
   scripts/bin/harness-cli story add --id <id> --title "<text>" --lane high-risk
   ```

6. **Implement** — same as Normal.

7. **Validate**
   Run `validate:quick` + `test:integration` + `test:e2e`. Paste output in
   story Evidence.

8. **User-flow test + evidence artifacts**
   Run user-flow tests covering **primary path + at least 1 error/edge path**.
   For web changes: capture screenshots to `docs/evidence/<name>/`.
   For bot/chat: paste simulator output showing each user step.
   Paste all command outputs in story Evidence section.

9. **Review Gate (full)**
   Run full review-work skill (5 parallel sub-agents). All must pass.
   Reviewer ≠ implementer. Paste Review Verdict + per-criterion evidence
   table in story Evidence section. Proceed only on PASS.
   
   Record review:
   ```bash
   scripts/bin/harness-cli intervention add --type review --description "Full review verdict: PASS" --source agent
   ```

10. **PR + Bot Review Loop**
    Push branch, open PR. Address every bot review comment
    substantively. Re-run validate + user-flow + Review Gate on each substantive
    push. Loop until bot approves (max 3 cycles → escalate to human).

11. **Close**
    ```bash
    openspec archive "<name>"
    ```
    
    Record decision:
    ```bash
    scripts/bin/harness-cli decision add --id <id> --title "<text>" --doc docs/decisions/<file>.md
    ```
    
    Update story status:
    ```bash
    scripts/bin/harness-cli story update --id <id> --status done
    ```
    
    Update `docs/TEST_MATRIX.md` with evidence + Review Verdict.

## Risk Checklist

Mark one flag for each item that applies:

| Risk flag | Applies when the work touches |
| --- | --- |
| Auth | login, logout, sessions, JWT, password, refresh token |
| Authorization | roles, permissions, tenant or company scope |
| Data model | schema, migrations, uniqueness, deletion, retention |
| Audit/security | audit logs, privacy, sensitive data, access logs |
| External systems | email, payments, cloud services, provider SDKs, queues, webhooks |
| Public contracts | API shape, response envelope, client-visible behavior |
| Cross-platform | desktop/mobile/browser split, native shell behavior, deep links |
| Existing behavior | already implemented or test-covered behavior changes |
| Weak proof | unclear or missing tests around the affected area |
| Multi-domain | more than one product domain changes at once |

## Classification

```text
0-1 flags:
  tiny or normal, based on code impact

2-3 flags:
  normal with stronger validation

4+ flags:
  high-risk

Any hard gate:
  high-risk unless the human explicitly narrows scope
```

Hard gates:

- Auth.
- Authorization.
- Data loss or migration.
- Audit/security.
- External provider behavior.
- Removing or weakening validation requirements.

## Output

At the end of intake, the agent must state lane + change type + planned gates:

```text
Lane: normal
Change type: user-feature
Reason: touches API contract and existing behavior (2 flags).
Proposal: <link>
Validation: validate:quick + test:integration
User-flow test: matching changed surface
Review Gate: single Oracle review
PR Bot Review: required (max 3 push cycles)
```

For infrastructure/migrations (no user surface):
```text
Lane: normal
Change type: infrastructure
Reason: data model touched (1 flag), no user-visible behavior.
Proposal: <link>
Validation: validate:quick + test:integration
User-flow test: not applicable — change type exempt
Review Gate: self-verify
PR Bot Review: required (max 3 push cycles)
```

For tiny lane:
```text
Lane: tiny
Change type: docs
Reason: single-file change, 0 risk flags.
Action: patch directly, run validate:quick.
No proposal, no Review Gate, no user-flow test required.
PR Bot Review: still required if pushing to remote.
```
