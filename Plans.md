---
_harness_template: "Plans.md.template"
_harness_version: "4.10.0"
---

# Plans.md - Task Tracking

> **Project**: tolvex
> **Last updated**: 2026-07-07
> **Updated by**: Claude Code

---

## In Progress

<!-- Add tasks with cc:wip here. -->

(none)

---

## Not Started

<!-- Add tasks with cc:todo or pm:requested here. -->

(none)

---

## Completed

<!-- Add tasks with cc:done or pm:confirmed here. -->

(none)

---

## Archive

<!-- Move older completed tasks here. -->

---

## Status Marker Legend

These markers are protocol values used by Harness tooling. Keep them unchanged
unless the project has tested parser aliases. No Japanese characters in status
markers per this project's CLAUDE.md.

| Marker | Meaning |
|--------|---------|
| `pm:requested` | PM requested work |
| `cc:todo` | Not started by Claude Code |
| `cc:wip` | Claude Code is working |
| `cc:done` | Claude Code completed the task and is awaiting confirmation |
| `pm:confirmed` | PM confirmed completion |
| `blocked` | Blocked; include the reason next to the task |

---

## Optional Extended Syntax

For larger plans, you may add task IDs, dependencies, and parallel markers.

### Task ID / Dependency / Parallel Marker

```markdown
- [ ] T001: Authentication `cc:todo`
- [ ] T002: User API `cc:todo` depends:T001
- [ ] T003: Product API `cc:todo` [P]
- [ ] T004: Order API `cc:todo` depends:T001,T003
```

| Syntax | Meaning | Example |
|--------|---------|---------|
| `T001:` | Optional task ID | Used for references and dependencies |
| `depends:ID` | Dependency task | `depends:T001,T002` |
| `[P]` | Parallelizable | Can run at the same time as other ready tasks |

**Note**: Extended syntax is optional. The plain checklist format still works.

---

## Last Update

- **Updated at**: 2026-07-07
- **Last session owner**: Claude Code
- **Branch**: main
