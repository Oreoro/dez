use indexmap::IndexMap;
use serde::Deserialize;
use strum::EnumIter;

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum VsCodeTokenScope {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct VsCodeTokenColor {
    pub name: Option<String>,
    pub scope: Option<VsCodeTokenScope>,
    pub settings: VsCodeTokenColorSettings,
}

#[derive(Debug, Deserialize)]
pub struct VsCodeTokenColorSettings {
    pub foreground: Option<String>,
    pub background: Option<String>,
    #[serde(rename = "fontStyle")]
    pub font_style: Option<String>,
}

#[derive(Debug, PartialEq, Copy, Clone, EnumIter)]
pub enum DezSyntaxToken {
    Attribute,
    Boolean,
    Comment,
    CommentDoc,
    Constant,
    Constructor,
    Embedded,
    Emphasis,
    EmphasisStrong,
    Enum,
    Function,
    Hint,
    Keyword,
    Label,
    LinkText,
    LinkUri,
    Number,
    Operator,
    Predictive,
    Preproc,
    Primary,
    Property,
    Punctuation,
    PunctuationBracket,
    PunctuationDelimiter,
    PunctuationListMarker,
    PunctuationSpecial,
    String,
    StringEscape,
    StringRegex,
    StringSpecial,
    StringSpecialSymbol,
    Tag,
    TextLiteral,
    Title,
    Type,
    Variable,
    VariableSpecial,
    Variant,
}

impl std::fmt::Display for DezSyntaxToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DezSyntaxToken::Attribute => "attribute",
                DezSyntaxToken::Boolean => "boolean",
                DezSyntaxToken::Comment => "comment",
                DezSyntaxToken::CommentDoc => "comment.doc",
                DezSyntaxToken::Constant => "constant",
                DezSyntaxToken::Constructor => "constructor",
                DezSyntaxToken::Embedded => "embedded",
                DezSyntaxToken::Emphasis => "emphasis",
                DezSyntaxToken::EmphasisStrong => "emphasis.strong",
                DezSyntaxToken::Enum => "enum",
                DezSyntaxToken::Function => "function",
                DezSyntaxToken::Hint => "hint",
                DezSyntaxToken::Keyword => "keyword",
                DezSyntaxToken::Label => "label",
                DezSyntaxToken::LinkText => "link_text",
                DezSyntaxToken::LinkUri => "link_uri",
                DezSyntaxToken::Number => "number",
                DezSyntaxToken::Operator => "operator",
                DezSyntaxToken::Predictive => "predictive",
                DezSyntaxToken::Preproc => "preproc",
                DezSyntaxToken::Primary => "primary",
                DezSyntaxToken::Property => "property",
                DezSyntaxToken::Punctuation => "punctuation",
                DezSyntaxToken::PunctuationBracket => "punctuation.bracket",
                DezSyntaxToken::PunctuationDelimiter => "punctuation.delimiter",
                DezSyntaxToken::PunctuationListMarker => "punctuation.list_marker",
                DezSyntaxToken::PunctuationSpecial => "punctuation.special",
                DezSyntaxToken::String => "string",
                DezSyntaxToken::StringEscape => "string.escape",
                DezSyntaxToken::StringRegex => "string.regex",
                DezSyntaxToken::StringSpecial => "string.special",
                DezSyntaxToken::StringSpecialSymbol => "string.special.symbol",
                DezSyntaxToken::Tag => "tag",
                DezSyntaxToken::TextLiteral => "text.literal",
                DezSyntaxToken::Title => "title",
                DezSyntaxToken::Type => "type",
                DezSyntaxToken::Variable => "variable",
                DezSyntaxToken::VariableSpecial => "variable.special",
                DezSyntaxToken::Variant => "variant",
            }
        )
    }
}

impl DezSyntaxToken {
    pub fn find_best_token_color_match<'a>(
        &self,
        token_colors: &'a [VsCodeTokenColor],
    ) -> Option<&'a VsCodeTokenColor> {
        let mut ranked_matches = IndexMap::new();

        for (ix, token_color) in token_colors.iter().enumerate() {
            if token_color.settings.foreground.is_none() {
                continue;
            }

            let Some(rank) = self.rank_match(token_color) else {
                continue;
            };

            if rank > 0 {
                ranked_matches.insert(ix, rank);
            }
        }

        ranked_matches
            .into_iter()
            .max_by_key(|(_, rank)| *rank)
            .map(|(ix, _)| &token_colors[ix])
    }

    fn rank_match(&self, token_color: &VsCodeTokenColor) -> Option<u32> {
        let candidate_scopes = match token_color.scope.as_ref()? {
            VsCodeTokenScope::One(scope) => vec![scope],
            VsCodeTokenScope::Many(scopes) => scopes.iter().collect(),
        }
        .iter()
        .flat_map(|scope| scope.split(',').map(|s| s.trim()))
        .collect::<Vec<_>>();

        let scopes_to_match = self.to_vscode();
        let number_of_scopes_to_match = scopes_to_match.len();

        let mut matches = 0;

        for (ix, scope) in scopes_to_match.into_iter().enumerate() {
            // Assign each entry a weight that is inversely proportional to its
            // position in the list.
            //
            // Entries towards the front are weighted higher than those towards the end.
            let weight = (number_of_scopes_to_match - ix) as u32;

            if candidate_scopes.contains(&scope) {
                matches += 1 + weight;
            }
        }

        Some(matches)
    }

    pub fn fallbacks(&self) -> &[Self] {
        match self {
            DezSyntaxToken::CommentDoc => &[DezSyntaxToken::Comment],
            DezSyntaxToken::Number => &[DezSyntaxToken::Constant],
            DezSyntaxToken::VariableSpecial => &[DezSyntaxToken::Variable],
            DezSyntaxToken::PunctuationBracket
            | DezSyntaxToken::PunctuationDelimiter
            | DezSyntaxToken::PunctuationListMarker
            | DezSyntaxToken::PunctuationSpecial => &[DezSyntaxToken::Punctuation],
            DezSyntaxToken::StringEscape
            | DezSyntaxToken::StringRegex
            | DezSyntaxToken::StringSpecial
            | DezSyntaxToken::StringSpecialSymbol => &[DezSyntaxToken::String],
            _ => &[],
        }
    }

    fn to_vscode(self) -> Vec<&'static str> {
        match self {
            DezSyntaxToken::Attribute => vec!["entity.other.attribute-name"],
            DezSyntaxToken::Boolean => vec!["constant.language"],
            DezSyntaxToken::Comment => vec!["comment"],
            DezSyntaxToken::CommentDoc => vec!["comment.block.documentation"],
            DezSyntaxToken::Constant => {
                vec!["constant", "constant.language", "constant.character"]
            }
            DezSyntaxToken::Constructor => {
                vec![
                    "entity.name.tag",
                    "entity.name.function.definition.special.constructor",
                ]
            }
            DezSyntaxToken::Embedded => vec!["meta.embedded"],
            DezSyntaxToken::Emphasis => vec!["markup.italic"],
            DezSyntaxToken::EmphasisStrong => vec![
                "markup.bold",
                "markup.italic markup.bold",
                "markup.bold markup.italic",
            ],
            DezSyntaxToken::Enum => vec!["support.type.enum"],
            DezSyntaxToken::Function => vec![
                "entity.function",
                "entity.name.function",
                "variable.function",
            ],
            DezSyntaxToken::Hint => vec![],
            DezSyntaxToken::Keyword => vec![
                "keyword",
                "keyword.other.fn.rust",
                "keyword.control",
                "keyword.control.fun",
                "keyword.control.class",
                "punctuation.accessor",
                "entity.name.tag",
            ],
            DezSyntaxToken::Label => vec![
                "label",
                "entity.name",
                "entity.name.import",
                "entity.name.package",
            ],
            DezSyntaxToken::LinkText => vec!["markup.underline.link", "string.other.link"],
            DezSyntaxToken::LinkUri => vec!["markup.underline.link", "string.other.link"],
            DezSyntaxToken::Number => vec!["constant.numeric", "number"],
            DezSyntaxToken::Operator => vec!["operator", "keyword.operator"],
            DezSyntaxToken::Predictive => vec![],
            DezSyntaxToken::Preproc => vec![
                "preproc",
                "meta.preprocessor",
                "punctuation.definition.preprocessor",
            ],
            DezSyntaxToken::Primary => vec![],
            DezSyntaxToken::Property => vec![
                "variable.member",
                "support.type.property-name",
                "variable.object.property",
                "variable.other.field",
            ],
            DezSyntaxToken::Punctuation => vec![
                "punctuation",
                "punctuation.section",
                "punctuation.accessor",
                "punctuation.separator",
                "punctuation.definition.tag",
            ],
            DezSyntaxToken::PunctuationBracket => vec![
                "punctuation.bracket",
                "punctuation.definition.tag.begin",
                "punctuation.definition.tag.end",
            ],
            DezSyntaxToken::PunctuationDelimiter => vec![
                "punctuation.delimiter",
                "punctuation.separator",
                "punctuation.terminator",
            ],
            DezSyntaxToken::PunctuationListMarker => {
                vec!["markup.list punctuation.definition.list.begin"]
            }
            DezSyntaxToken::PunctuationSpecial => vec!["punctuation.special"],
            DezSyntaxToken::String => vec!["string"],
            DezSyntaxToken::StringEscape => {
                vec!["string.escape", "constant.character", "constant.other"]
            }
            DezSyntaxToken::StringRegex => vec!["string.regex"],
            DezSyntaxToken::StringSpecial => vec!["string.special", "constant.other.symbol"],
            DezSyntaxToken::StringSpecialSymbol => {
                vec!["string.special.symbol", "constant.other.symbol"]
            }
            DezSyntaxToken::Tag => vec!["tag", "entity.name.tag", "meta.tag.sgml"],
            DezSyntaxToken::TextLiteral => vec!["text.literal", "string"],
            DezSyntaxToken::Title => vec!["title", "entity.name"],
            DezSyntaxToken::Type => vec![
                "entity.name.type",
                "entity.name.type.primitive",
                "entity.name.type.numeric",
                "keyword.type",
                "support.type",
                "support.type.primitive",
                "support.class",
            ],
            DezSyntaxToken::Variable => vec![
                "variable",
                "variable.language",
                "variable.member",
                "variable.parameter",
                "variable.parameter.function-call",
            ],
            DezSyntaxToken::VariableSpecial => vec![
                "variable.special",
                "variable.member",
                "variable.annotation",
                "variable.language",
            ],
            DezSyntaxToken::Variant => vec!["variant"],
        }
    }
}
