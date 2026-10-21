# Workflow State Matrix

Use this matrix before wireframing or implementing a Dez workflow. Remove rows
that do not apply, but never omit failure and return states merely because the
happy path is simple.

| State | User question | Required surface response | Forbidden response |
| --- | --- | --- | --- |
| Home | Where do I begin? | one primary Workspace action, recent work, native tool launch after a Workspace exists | hero art, setup overlay, pathless agent terminal |
| Focused work | Where am I working? | active Workspace, focused pane text, native tabs, adjacent `+`, durable status | duplicate tab manager, hidden focus |
| Start work | Where will this run? | named target Workspace and native Terminal or task tab | automatic mystery split |
| Running | Is work still active? | bounded Activity with owner and text state | transcript scraping, permanent dashboard |
| Completed | Is there anything left to do? | keep review-ready or unread work in Activity; send inactive completed Agent Sessions to History | unbounded historical rows in Workspaces |
| Idle tab | How do I return to open content? | keep it in Layout and the native tab strip | keeping idle terminals in Activity merely because a record exists |
| Waiting | What needs me? | actionable row, native attention, exact permission/review action | color-only badge, floating mascot |
| Return | What changed while away? | one-time recap from trusted events with Review/Dismiss | invented summary of PTY output |
| Review | How do I inspect changes? | existing Files/Git/Diff surface in the Main Work Area | custom review overlay |
| Workspace access | Why is everything failing? | one root-scoped Access required notice and Grant Access action | repeated background prompts and log flood |
| Install required | Can durable work start? | native Home installation state and relaunch | restoring helpers from a temporary DMG |
| Terminal launch failure | Did a shell start? | inline cause, settings route, retry/new shell as appropriate | generic toast claiming success |
| tmux/Herdr stale attach | Is the external session alive? | last-known state, refresh/retry, preserved output, honest ownership | duplicate auto-attach or process-death claim |
| cmux handoff | Which app owns this? | explicit Open in cmux action and ownership copy | embedded imitation of cmux |
| Search | What matches? | replace only Workspaces list body, clear action, Main Work Area unchanged | modal/global overlay |
| Narrow window | What remains essential? | Workspace, focused pane, actionable Activity, status recovery route | horizontal scrolling, crushed Main Work Area |
| Keyboard-only | Can I operate every route? | logical focus order, labels, Escape/back path, native actions | hover-only controls |
| Screen reader | What is selected and why? | role, position, focused/visible text, owner and action labels | unlabeled icons or color-only state |
| Quit/restart | Will work be safe? | bounded cancellation, durable owner state, honest legacy visibility | silent process kill or fake migration |

## Review questions

For every row in scope, answer:

1. What exact native entity owns the state?
2. Which existing action opens or activates it?
3. What source proves the state is authoritative?
4. What copy names cause and next action?
5. What remains visible after the action fails?
6. What is source-verified, remote-verified, and runtime-verified?
