//! The rail's session search.
//!
//! Ported in behavior from dez v0.6's sidebar filter (`fuzzy_match_positions`
//! in `crates/sidebar/src/sidebar.rs`): a query is read out of a session's
//! text in order, case-insensitively, and the matched characters are
//! highlighted where they sit. v0.6 matched a contiguous substring; this
//! matches a subsequence, so `fps` finds "Fix bug in panel sidebar", which is
//! what a type-to-filter rail is expected to do.
//!
//! Matching is deliberately synchronous. The rail rebuilds its rows on every
//! render, so an async matcher would put a background pass in front of every
//! keystroke and let a slow pass land after a fast one; the rail holds tens of
//! rows, not thousands, so one forward scan per field is cheaper than the
//! machinery to cancel and reorder one.

/// The query to filter the rail by, or `None` when there is nothing to filter
/// by.
///
/// Whitespace-only counts as blank. A trailing space is what a user types while
/// they are still forming the next word, and matching on it would empty the
/// rail out from under them.
pub fn normalize_query(query: &str) -> Option<&str> {
    let query = query.trim();
    (!query.is_empty()).then_some(query)
}

/// The byte offsets in `candidate` that `query` matches in order, or `None`
/// when `query` cannot be read out of `candidate`.
///
/// Offsets ascend, are one per query character, and always land on UTF-8
/// character boundaries -- the shape `ui::HighlightedLabel` expects, and the
/// reason a multi-byte title can be highlighted without tripping its debug
/// assertion.
///
/// An empty query matches everything and highlights nothing, so callers that
/// treat "no query" as "no filter" can pass one straight through.
pub fn match_positions(query: &str, candidate: &str) -> Option<Vec<usize>> {
    let query: Vec<char> = query.chars().collect();
    if query.is_empty() {
        return Some(Vec::new());
    }

    let candidate: Vec<(usize, char)> = candidate.char_indices().collect();
    let mut positions = Vec::with_capacity(query.len());
    // Where the previous query character landed, and the first index still open
    // to this one. Both are char indices, not byte offsets.
    let mut previous: Option<usize> = None;
    let mut search_from = 0;

    for query_char in query {
        let equal = |index: usize| chars_equal(candidate[index].1, query_char);
        // Extend the run already being matched first, then take a word start,
        // then take whatever is left. Without that order `ab` would pair the
        // `a` of "alpha beta" with nothing sensible, and the highlight would
        // read as scattered noise rather than as the word the user typed.
        let index = previous
            .and_then(|previous| previous.checked_add(1))
            .filter(|&index| index < candidate.len() && equal(index))
            .or_else(|| {
                (search_from..candidate.len())
                    .find(|&index| is_word_start(&candidate, index) && equal(index))
            })
            .or_else(|| (search_from..candidate.len()).find(|&index| equal(index)))?;
        positions.push(candidate[index].0);
        previous = Some(index);
        search_from = index + 1;
    }

    Some(positions)
}

/// The offsets to highlight in `title` for a query that matched the title or
/// any of `also_matches` -- the fields a session row does not spell out but
/// which still admit the row when they match, such as an agent kind or a
/// working directory.
///
/// `None` when nothing matched. `Some(Vec::new())` when only a field the row
/// does not display matched: the row survives the filter with nothing
/// highlighted, which is what v0.6 did for a worktree-name match.
///
/// A blank query matches nothing rather than everything. Callers skip the
/// filter entirely in that case, so returning `None` here keeps "blank" from
/// quietly reading as "match every row" if one of them forgets.
pub fn match_title(query: &str, title: &str, also_matches: &[&str]) -> Option<Vec<usize>> {
    if let Some(positions) = non_empty_match(query, title) {
        return Some(positions);
    }
    also_matches
        .iter()
        .find_map(|field| non_empty_match(query, field))
        .map(|_| Vec::new())
}

fn non_empty_match(query: &str, candidate: &str) -> Option<Vec<usize>> {
    match match_positions(query, candidate)? {
        positions if positions.is_empty() => None,
        positions => Some(positions),
    }
}

fn chars_equal(candidate: char, query: char) -> bool {
    candidate == query || candidate.to_lowercase().eq(query.to_lowercase())
}

/// Whether `index` starts a word: either the first character, or one whose
/// predecessor is not alphanumeric. This is what makes `-` and `_` and `/`
/// count as separators inside a title or a working directory.
fn is_word_start(candidate: &[(usize, char)], index: usize) -> bool {
    if index == 0 {
        return true;
    }
    match candidate.get(index - 1) {
        Some((_, previous)) => !previous.is_alphanumeric(),
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::{match_positions, match_title, normalize_query};

    #[test]
    fn an_empty_query_matches_everything_and_highlights_nothing() {
        assert_eq!(match_positions("", "alpha"), Some(Vec::new()));
    }

    #[test]
    fn a_contiguous_substring_matches_and_highlights_every_character() {
        assert_eq!(
            match_positions("bug", "Fix bug in sidebar"),
            Some(vec![4, 5, 6])
        );
    }

    #[test]
    fn matching_ignores_case_in_both_directions() {
        assert_eq!(
            match_positions("SIDEBAR", "Fix bug in sidebar"),
            Some(vec![11, 12, 13, 14, 15, 16, 17])
        );
        assert_eq!(
            match_positions("fix", "Fix bug in sidebar"),
            Some(vec![0, 1, 2])
        );
        assert_eq!(
            match_positions("BUG", "Fix bug in sidebar"),
            Some(vec![4, 5, 6])
        );
    }

    #[test]
    fn a_subsequence_matches_across_word_boundaries() {
        assert_eq!(
            match_positions("fps", "Fix bug in panel sidebar"),
            Some(vec![0, 11, 17])
        );
    }

    #[test]
    fn a_run_is_preferred_over_an_earlier_loose_match() {
        assert_eq!(match_positions("ab", "alpha beta"), Some(vec![0, 6]));
    }

    #[test]
    fn a_word_start_is_preferred_over_an_earlier_match_inside_a_word() {
        // "axb b": the `b` inside "axb" comes first but is mid-word, so the
        // word-start preference skips it for the `b` that opens a word. Without
        // that preference this would be [0, 2].
        assert_eq!(match_positions("ab", "axb b"), Some(vec![0, 4]));
    }

    #[test]
    fn a_run_is_preferred_over_the_word_start_preference() {
        // "at" is the front of "atlas", so continuing the run wins outright and
        // the later word-start `a` in "afar" is never considered.
        assert_eq!(match_positions("at", "atlas afar"), Some(vec![0, 1]));
    }

    #[test]
    fn a_separator_breaks_a_run() {
        // `-` is not alphanumeric, so it both ends a run and opens a word.
        assert_eq!(match_positions("ab", "a-b"), Some(vec![0, 2]));
    }

    #[test]
    fn a_query_that_cannot_be_read_in_order_does_not_match() {
        assert_eq!(match_positions("bar alpha", "alpha beta"), None);
        assert_eq!(match_positions("toolong", "short"), None);
        assert_eq!(match_positions("a", ""), None);
    }

    #[test]
    fn a_repeated_character_consumes_one_candidate_character_each() {
        assert_eq!(match_positions("aa", "alpha"), Some(vec![0, 4]));
    }

    #[test]
    fn offsets_land_on_character_boundaries_for_non_ascii_titles() {
        let title = "Reparer café";
        let positions = match_positions("café", title).expect("query should match");
        // One offset per query character, so a two-byte character contributes
        // one offset and the byte span it covers is two wide.
        assert_eq!(positions.len(), "café".chars().count());
        assert!(
            positions.iter().all(|&index| title.is_char_boundary(index)),
            "every offset must be a character boundary: {positions:?}"
        );
        assert_eq!(&title[positions[0]..positions[3] + 'é'.len_utf8()], "café");
    }

    #[test]
    fn matching_ignores_case_outside_ascii_too() {
        let title = "Reparer café";
        assert_eq!(
            match_positions("CAFÉ", title),
            match_positions("café", title)
        );
    }

    #[test]
    fn match_title_admits_a_row_that_only_matches_a_hidden_field() {
        assert_eq!(
            match_title("codex", "Fix bug in sidebar", &["codex"]),
            Some(Vec::new())
        );
    }

    #[test]
    fn match_title_prefers_the_title_so_the_highlight_is_visible() {
        assert_eq!(
            match_title("fix", "Fix bug in sidebar", &["codex"]),
            Some(vec![0, 1, 2])
        );
    }

    #[test]
    fn match_title_reports_no_match_for_an_unrelated_query() {
        assert_eq!(match_title("zzz", "Fix bug in sidebar", &["codex"]), None);
    }

    #[test]
    fn match_title_treats_a_blank_query_as_no_match() {
        assert_eq!(match_title("", "Fix bug in sidebar", &["codex"]), None);
    }

    #[test]
    fn a_whitespace_only_query_normalizes_to_no_filter() {
        assert_eq!(normalize_query(""), None);
        assert_eq!(normalize_query("   "), None);
        assert_eq!(normalize_query("  fix  "), Some("fix"));
    }
}
