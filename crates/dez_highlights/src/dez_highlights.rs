//! dez-opinionated special-comment highlighting.
//!
//! Scans syntax-highlighted comment chunks for conventional attention keywords
//! (`TODO`, `FIXME`, `HACK`, …) and renders them bold in the theme's `hint`
//! color. Lives in its own crate so the divergence from upstream stays in a
//! `dez_*` crate and only requires the `HighlightKey::SpecialComment` variant.

use std::{ops::Range, sync::OnceLock, time::Duration};

use aho_corasick::{AhoCorasick, MatchKind};
use editor::{Addon, Editor, HighlightKey, RangeToAnchorExt as _};
use gpui::{App, AppContext as _, Context, FontWeight, HighlightStyle, Subscription, Task};
use language::{Anchor, LanguageAwareStyling};
use multi_buffer::{Event as MultiBufferEvent, MultiBufferOffset, MultiBufferSnapshot};
use theme::{ActiveTheme, SyntaxTheme};

/// Conventional comment markers worth pulling out of a wall of prose. Order
/// only matters for reporting; the matcher uses leftmost-first matching.
const SPECIAL_COMMENT_KEYWORDS: &[&str] = &[
    "TODO", "FIXME", "HACK", "XXX", "NOTE", "WIP", "BUG", "OPTIMIZE", "REVIEW",
];

const REFRESH_DEBOUNCE: Duration = Duration::from_millis(120);

fn keyword_matcher() -> &'static AhoCorasick {
    static MATCHER: OnceLock<AhoCorasick> = OnceLock::new();
    MATCHER.get_or_init(|| {
        AhoCorasick::builder()
            .match_kind(MatchKind::LeftmostFirst)
            .build(SPECIAL_COMMENT_KEYWORDS)
            .expect("special comment keyword automaton is static and valid")
    })
}

struct SpecialCommentAddon {
    _subscription: Subscription,
    _task: Task<()>,
}

impl Addon for SpecialCommentAddon {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }
}

pub fn init(cx: &mut App) {
    cx.observe_new::<Editor>(|editor, _window, cx| {
        let buffer = editor.buffer().clone();
        let subscription = cx.subscribe(&buffer, |editor, event: &MultiBufferEvent, cx| {
            if matches!(
                event,
                MultiBufferEvent::Edited { .. }
                    | MultiBufferEvent::BuffersEdited { .. }
                    | MultiBufferEvent::Reparsed(_)
            ) {
                refresh(editor, cx);
            }
        });

        editor.register_addon(SpecialCommentAddon {
            _subscription: subscription,
            _task: Task::ready(()),
        });

        refresh(editor, cx);
    })
    .detach();
}

fn refresh(editor: &mut Editor, cx: &mut Context<Editor>) {
    let syntax_theme = cx.theme().syntax().clone();
    let mut style = syntax_theme
        .style_for_name("hint")
        .or_else(|| syntax_theme.style_for_name("emphasis"))
        .unwrap_or_else(|| HighlightStyle {
            color: Some(cx.theme().colors().text_accent),
            ..Default::default()
        });
    style.font_weight = Some(FontWeight::BOLD);
    style.font_style = None;
    style.background_color = None;

    let snapshot = editor.buffer().read(cx).snapshot(cx);
    let task = cx.spawn(async move |editor, cx| {
        cx.background_executor().timer(REFRESH_DEBOUNCE).await;
        let ranges = cx
            .background_spawn({
                let snapshot = snapshot.clone();
                let syntax_theme = syntax_theme.clone();
                async move { find_special_comment_ranges(&snapshot, &syntax_theme) }
            })
            .await;

        editor
            .update(cx, |editor, cx| {
                editor.clear_highlights(HighlightKey::SpecialComment, cx);
                if !ranges.is_empty() {
                    editor.highlight_text(HighlightKey::SpecialComment, ranges, style, cx);
                }
            })
            .ok();
    });

    if let Some(addon) = editor.addon_mut::<SpecialCommentAddon>() {
        addon._task = task;
    }
}

fn find_special_comment_ranges(
    snapshot: &MultiBufferSnapshot,
    syntax_theme: &SyntaxTheme,
) -> Vec<Range<Anchor>> {
    let matcher = keyword_matcher();
    let mut ranges = Vec::new();
    let mut offset = 0usize;

    for chunk in snapshot.chunks(
        MultiBufferOffset::ZERO..snapshot.len(),
        LanguageAwareStyling {
            tree_sitter: true,
            diagnostics: false,
        },
    ) {
        let chunk_start = offset;
        offset += chunk.text.len();

        let Some(highlight_id) = chunk.syntax_highlight_id else {
            continue;
        };
        let Some(capture_name) = syntax_theme.get_capture_name(usize::from(highlight_id)) else {
            continue;
        };
        if !capture_name.starts_with("comment") {
            continue;
        }

        let text = chunk.text;
        for matched in matcher.find_iter(text) {
            if !is_word_boundary(text, matched.start(), matched.end()) {
                continue;
            }
            ranges.push(
                (chunk_start + matched.start()..chunk_start + matched.end()).to_anchors(snapshot),
            );
        }
    }

    ranges
}

fn is_word_boundary(text: &str, start: usize, end: usize) -> bool {
    let bytes = text.as_bytes();
    let before_ok = start == 0 || !is_identifier_byte(bytes[start - 1]);
    let after_ok = end >= bytes.len() || !is_identifier_byte(bytes[end]);
    before_ok && after_ok
}

fn is_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}
