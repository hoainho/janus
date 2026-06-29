# Harness Workflow: Goal-Driven Continuous Improvement

## Core Principle

> "Harness không chỉ chạy eval — nó phân tích, đặt câu hỏi, và đề xuất cải thiện dựa trên goal của bạn."

---

## 1. Khi bắt đầu với Harness

### Step 1: Goal Clarification

Khi chạy `harness-cli eval run`, Harness PHẢI hỏi:

```
┌─────────────────────────────────────────────────────────┐
│  EVAL-HARNESS: Goal Clarification                        │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  Bạn muốn đạt gì với eval này?                          │
│                                                          │
│  1. Phát hiện regression trước khi push                  │
│  2. Đánh giá chất lượng skill sau refactor               │
│  3. So sánh hiệu suất giữa 2 version                    │
│  4. Kiểm tra compatibility với model mới                 │
│  5. Mục tiêu khác (mô tả)                               │
│                                                          │
│  Chọn hoặc mô tả goal của bạn: ___                      │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### Step 2: Goal-Specific Configuration

Dựa vào goal, Harness tự điều chỉnh:

| Goal | Config | Focus |
|------|--------|-------|
| Regression detection | `--mode=smoke --strict` | Nhanh, block on fail |
| Quality assessment | `--mode=full` | Chi tiết, all checks |
| Performance comparison | `--mode=full --benchmark` | Timing, cost |
| Model compatibility | `--mode=full --model=X` | Multi-model test |

---

## 2. Quy trình làm việc hàng ngày

### Daily Workflow

```
┌─────────────────────────────────────────────────────────┐
│  DAILY HARNESS WORKFLOW                                  │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  1. Morning: Chạy baseline mới                           │
│     $ harness-cli eval baseline --skill=<skill>          │
│                                                          │
│  2. Development: Code changes                            │
│     (bình thường)                                        │
│                                                          │
│  3. Pre-commit: Chạy eval                                │
│     $ harness-cli eval run --skill=<skill>               │
│                                                          │
│  4. Review: Phân tích kết quả                            │
│     $ harness-cli eval analyze --skill=<skill>           │
│                                                          │
│  5. Improve: Áp dụng đề xuất                             │
│     $ harness-cli eval suggest --skill=<skill>           │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### Analyze Command (NEW)

```bash
# Phân tích xu hướng
harness-cli eval analyze --skill=<skill>

# Output:
# ┌─────────────────────────────────────────────────────────┐
# │  EVAL ANALYSIS: my-skill                                │
# ├─────────────────────────────────────────────────────────┤
# │                                                          │
# │  📊 7-Day Trend:                                        │
# │    - Pass rate: 85% → 92% (↑7%)                         │
# │    - Avg duration: 2.3s → 1.8s (↓22%)                   │
# │    - Cost/run: $0.12 → $0.09 (↓25%)                     │
# │                                                          │
# │  🔍 Patterns Detected:                                  │
# │    - Case "smoke-003" flaky (3/7 passes)                │
# │    - Shell check timeout on slow days                   │
# │    - LLM judge inconsistent on edge cases               │
# │                                                          │
# │  💡 Suggestions:                                        │
# │    1. Fix flaky case: add retry or stabilize fixture    │
# │    2. Increase timeout for shell checks                 │
# │    3. Add more specific rubric for LLM judge            │
# │                                                          │
# │  🎯 Goal Alignment:                                     │
# │    Your goal: "Phát hiện regression"                    │
# │    Current: 92% pass rate (target: 95%)                 │
# │    Action: Focus on flaky cases first                   │
# │                                                          │
# └─────────────────────────────────────────────────────────┘
```

### Suggest Command (NEW)

```bash
# Đề xuất cải thiện
harness-cli eval suggest --skill=<skill>

# Output:
# ┌─────────────────────────────────────────────────────────┐
# │  IMPROVEMENT SUGGESTIONS                                │
# ├─────────────────────────────────────────────────────────┤
# │                                                          │
# │  Based on 30 days of eval data:                         │
# │                                                          │
# │  HIGH PRIORITY:                                         │
# │  1. [Fix] smoke-003: Add stability check                │
# │     Reason: 43% flaky rate, causes false alarms         │
# │     Impact: -15% false regressions                      │
# │                                                          │
# │  2. [Add] performance benchmark case                    │
# │     Reason: No timing regression detection              │
# │     Impact: Catch 20% more regressions                  │
# │                                                          │
# │  MEDIUM PRIORITY:                                       │
# │  3. [Optimize] shell checks: parallel execution         │
# │     Reason: Sequential = 3x slower                      │
# │     Impact: -40% eval time                              │
# │                                                          │
# │  4. [Remove] redundant output_contains checks           │
# │     Reason: 5 checks test same thing                    │
# │     Impact: -10% eval time, cleaner results             │
# │                                                          │
# │  LOW PRIORITY:                                          │
# │  5. [Upgrade] LLM judge model to claude-sonnet-4-6     │
# │     Reason: Better edge case detection                  │
# │     Impact: +5% accuracy, +$0.03/case                   │
# │                                                          │
# └─────────────────────────────────────────────────────────┘
```

---

## 3. Monthly Review Process

### Sau 1 tháng: Đánh giá toàn diện

```bash
# Generate monthly report
harness-cli eval report --skill=<skill> --period=30d

# Output:
# ┌─────────────────────────────────────────────────────────┐
# │  MONTHLY EVAL REPORT: my-skill                          │
# │  Period: 2026-06-01 → 2026-06-30                        │
# ├─────────────────────────────────────────────────────────┤
# │                                                          │
# │  📈 OVERALL METRICS                                     │
# │    Total runs: 45                                       │
# │    Pass rate: 88% (target: 95%)                         │
# │    Avg duration: 2.1s                                   │
# │    Total cost: $4.05                                    │
# │    Regressions caught: 12                               │
# │    False positives: 3                                   │
# │                                                          │
# │  🎯 GOAL PROGRESS                                       │
# │    Goal: "Phát hiện regression trước khi push"          │
# │    Status: 88% → 92% (↑4%)                              │
# │    Gap to target: 3%                                    │
# │    Projected: Target achievable in 2 weeks              │
# │                                                          │
# │  🔬 CASE ANALYSIS                                       │
# │    Most stable: smoke-001 (100%)                        │
# │    Most flaky: smoke-003 (57%)                          │
# │    Slowest: full-005 (4.2s)                             │
# │    Most expensive: llm-002 ($0.15)                      │
# │                                                          │
# │  📋 ACTION ITEMS                                        │
# │    [ ] Fix smoke-003 flakiness                          │
# │    [ ] Add timeout to full-005                          │
# │    [ ] Review llm-002 rubric specificity                │
# │    [ ] Consider removing redundant-001                  │
# │                                                          │
# │  🔄 NEXT MONTH FOCUS                                    │
# │    Priority: Stabilize flaky cases                      │
# │    Target: 95% pass rate                                │
# │    Budget: $5.00 (current: $4.05)                       │
# │                                                          │
# └─────────────────────────────────────────────────────────┘
```

---

## 4. SQLite Schema cho Goal Tracking

### Tables mới

```sql
-- Goal tracking
CREATE TABLE eval_goal (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    skill           TEXT NOT NULL,
    goal_type       TEXT NOT NULL
                    CHECK(goal_type IN (
                        'regression_detection',
                        'quality_assessment',
                        'performance_comparison',
                        'model_compatibility',
                        'custom'
                    )),
    description     TEXT,
    target_metric   TEXT,  -- e.g., "pass_rate >= 95%"
    current_value   REAL,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Goal progress tracking
CREATE TABLE eval_goal_progress (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    goal_id         INTEGER NOT NULL REFERENCES eval_goal(id),
    run_id          TEXT NOT NULL,
    metric_value    REAL,
    notes           TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Improvement suggestions
CREATE TABLE eval_suggestion (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    skill           TEXT NOT NULL,
    suggestion_type TEXT NOT NULL
                    CHECK(suggestion_type IN ('fix', 'add', 'remove', 'optimize', 'upgrade')),
    priority        TEXT NOT NULL
                    CHECK(priority IN ('high', 'medium', 'low')),
    title           TEXT NOT NULL,
    description     TEXT,
    expected_impact TEXT,
    status          TEXT NOT NULL DEFAULT 'pending'
                    CHECK(status IN ('pending', 'accepted', 'rejected', 'implemented')),
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    implemented_at  TEXT
);
```

---

## 5. Question-Asking Protocol

### Khi goal chưa rõ ràng

Harness PHẢI hỏi những câu sau:

```
┌─────────────────────────────────────────────────────────┐
│  GOAL CLARIFICATION QUESTIONS                           │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  1. Bạn muốn eval này phục vụ mục đích gì?              │
│     □ Phát hiện regression                               │
│     □ Đánh giá chất lượng                                │
│     □ So sánh performance                                │
│     □ Kiểm tra compatibility                             │
│     □ Khác: ___                                          │
│                                                          │
│  2. Metric nào quan trọng nhất?                          │
│     □ Pass rate (target: __%)                            │
│     □ Speed (target: <__s)                               │
│     □ Cost (target: $__/run)                             │
│     □ Coverage (target: __% cases)                       │
│                                                          │
│  3. Ngân sách cho eval?                                  │
│     □ $__/tháng                                           │
│     □ $__/run                                            │
│     □ Không giới hạn                                     │
│                                                          │
│  4. Tần suất chạy eval?                                  │
│     □ Mỗi commit                                         │
│     □ Mỗi push                                           │
│     □ Hàng ngày                                          │
│     □ Theo yêu cầu                                       │
│                                                          │
│  5. Điều gì sẽ xảy ra nếu eval fail?                    │
│     □ Block push                                         │
│     □ Warning only                                       │
│     □ Log for review                                     │
│     □ Tự động fix                                        │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### Khi kết quả không như kỳ vọng

Harness PHẢI hỏi:

```
┌─────────────────────────────────────────────────────────┐
│  RESULT ANALYSIS QUESTIONS                               │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  Pass rate 85% (target 95%) - Gap 10%                    │
│                                                          │
│  Bạn muốn:                                               │
│  1. Phân tích nguyên nhân fail                           │
│  2. Điều chỉnh target (85% chấp nhận được?)              │
│  3. Thêm cases để tăng coverage                          │
│  4. Fix flaky cases trước                                │
│  5. Bỏ qua và tiếp tục                                  │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

---

## 6. Implementation Plan

### Phase 1: SQLite Integration (Week 1)

- [ ] Chuyển storage từ JSON sang SQLite
- [ ] Thêm `eval_goal` table
- [ ] Thêm `eval_goal_progress` table
- [ ] Thêm `eval_suggestion` table

### Phase 2: Goal-Driven Commands (Week 2)

- [ ] Implement `eval analyze` command
- [ ] Implement `eval suggest` command
- [ ] Implement `eval report` command
- [ ] Thêm goal clarification flow

### Phase 3: Question-Asking Protocol (Week 3)

- [ ] Implement goal clarification questions
- [ ] Implement result analysis questions
- [ ] Thêm adaptive questioning dựa trên context

### Phase 4: Monthly Review Automation (Week 4)

- [ ] Auto-generate monthly reports
- [ ] Auto-suggest improvements
- [ ] Track goal progress over time

---

## 7. Example Workflow

### Scenario: Bạn muốn phát hiện regression

```bash
# 1. Start với goal clarification
$ harness-cli eval run --skill=my-skill

# Harness asks:
# "Bạn muốn đạt gì với eval này?"
# You answer: "Phát hiện regression trước khi push"

# 2. Harness tự cấu hình
# --mode=smoke --strict --trigger=pre-push

# 3. Chạy eval
# Nếu pass → push được
# Nếu fail → block push, show diff

# 4. Sau 1 tuần, phân tích
$ harness-cli eval analyze --skill=my-skill

# Harness shows:
# - Pass rate: 92% (target: 95%)
# - 2 flaky cases detected
# - Suggest: Fix flaky cases

# 5. Áp dụng đề xuất
$ harness-cli eval suggest --skill=my-skill

# Harness shows:
# - HIGH: Fix smoke-003 flakiness
# - MEDIUM: Add performance benchmark

# 6. Sau 1 tháng, review
$ harness-cli eval report --skill=my-skill --period=30d

# Harness shows:
# - Goal achieved: 95% pass rate
# - 12 regressions caught
# - $4.05 total cost
# - Next month: Focus on performance
```

---

## Summary

Harness không chỉ là tool chạy eval — nó là **partner**帮你:

1. **Hỏi** khi goal chưa rõ
2. **Phân tích** kết quả theo goal
3. **Đề xuất** cải thiện dựa trên data
4. **Theo dõi** progress qua thời gian
5. **Đánh giá** lại sau 1 tháng

**Mục tiêu**: Harness giúp bạn cải thiện skill quality liên tục, không chỉ phát hiện regression.
