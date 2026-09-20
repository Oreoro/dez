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
pub enum dezSyntaxToken {
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

impl std::fmt::Display for dezSyntaxToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                dezSyntaxToken::Attribute => "attribute",
                dezSyntaxToken::Boolean => "boolean",
                dezSyntaxToken::Comment => "comment",
                dezSyntaxToken::CommentDoc => "comment.doc",
                dezSyntaxToken::Constant => "constant",
                dezSyntaxToken::Constructor => "constructor",
                dezSyntaxToken::Embedded => "embedded",
                dezSyntaxToken::Emphasis => "emphasis",
                dezSyntaxToken::EmphasisStrong => "emphasis.strong",
                dezSyntaxToken::Enum => "enum",
                dezSyntaxToken::Function => "function",
                dezSyntaxToken::Hint => "hint",
                dezSyntaxToken::Keyword => "keyword",
                dezSyntaxToken::Label => "label",
                dezSyntaxToken::LinkText => "link_text",
                dezSyntaxToken::LinkUri => "link_uri",
                dezSyntaxToken::Number => "number",
                dezSyntaxToken::Operator => "operator",
                dezSyntaxToken::Predictive => "predictive",
                dezSyntaxToken::Preproc => "preproc",
                dezSyntaxToken::Primary => "primary",
                dezSyntaxToken::Property => "property",
                dezSyntaxToken::Punctuation => "punctuation",
                dezSyntaxToken::PunctuationBracket => "punctuation.bracket",
                dezSyntaxToken::PunctuationDelimiter => "punctuation.delimiter",
                dezSyntaxToken::PunctuationListMarker => "punctuation.list_marker",
                dezSyntaxToken::PunctuationSpecial => "punctuation.special",
                dezSyntaxToken::String => "string",
                dezSyntaxToken::StringEscape => "string.escape",
                dezSyntaxToken::StringRegex => "string.regex",
                dezSyntaxToken::StringSpecial => "string.special",
                dezSyntaxToken::StringSpecialSymbol => "string.special.symbol",
                dezSyntaxToken::Tag => "tag",
                dezSyntaxToken::TextLiteral => "text.literal",
                dezSyntaxToken::Title => "title",
                dezSyntaxToken::Type => "type",
                dezSyntaxToken::Variable => "variable",
                dezSyntaxToken::VariableSpecial => "variable.special",
                dezSyntaxToken::Variant => "variant",
            }
        )
    }
}

impl dezSyntaxToken {
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
            dezSyntaxToken::CommentDoc => &[dezSyntaxToken::Comment],
            dezSyntaxToken::Number => &[dezSyntaxToken::Constant],
            dezSyntaxToken::VariableSpecial => &[dezSyntaxToken::Variable],
            dezSyntaxToken::PunctuationBracket
            | dezSyntaxToken::PunctuationDelimiter
            | dezSyntaxToken::PunctuationListMarker
            | dezSyntaxToken::PunctuationSpecial => &[dezSyntaxToken::Punctuation],
            dezSyntaxToken::StringEscape
            | dezSyntaxToken::StringRegex
            | dezSyntaxToken::StringSpecial
            | dezSyntaxToken::StringSpecialSymbol => &[dezSyntaxToken::String],
            _ => &[],
        }
    }

    fn to_vscode(self) -> Vec<&'static str> {
        match self {
            dezSyntaxToken::Attribute => vec!["entity.other.attribute-name"],
            dezSyntaxToken::Boolean => vec!["constant.language"],
            dezSyntaxToken::Comment => vec!["comment"],
            dezSyntaxToken::CommentDoc => vec!["comment.block.documentation"],
            dezSyntaxToken::Constant => {
                vec!["constant", "constant.language", "constant.character"]
            }
            dezSyntaxToken::Constructor => {
                vec![
                    "entity.name.tag",
                    "entity.name.function.definition.special.constructor",
                ]
            }
            dezSyntaxToken::Embedded => vec!["meta.embedded"],
            dezSyntaxToken::Emphasis => vec!["markup.italic"],
            dezSyntaxToken::EmphasisStrong => vec![
                "markup.bold",
                "markup.italic markup.bold",
                "markup.bold markup.italic",
            ],
            dezSyntaxToken::Enum => vec!["support.type.enum"],
            dezSyntaxToken::Function => vec![
                "entity.function",
                "entity.name.function",
                "variable.function",
            ],
            dezSyntaxToken::Hint => vec![],
            dezSyntaxToken::Keyword => vec![
                "keyword",
                "keyword.other.fn.rust",
                "keyword.control",
                "keyword.control.fun",
                "keyword.control.class",
                "punctuation.accessor",
                "entity.name.tag",
            ],
            dezSyntaxToken::Label => vec![
                "label",
                "entity.name",
                "entity.name.import",
                "entity.name.package",
            ],
            dezSyntaxToken::LinkText => vec!["markup.underline.link", "string.other.link"],
            dezSyntaxToken::LinkUri => vec!["markup.underline.link", "string.other.link"],
            dezSyntaxToken::Number => vec!["constant.numeric", "number"],
            dezSyntaxToken::Operator => vec!["operator", "keyword.operator"],
            dezSyntaxToken::Predictive => vec![],
            dezSyntaxToken::Preproc => vec![
                "preproc",
                "meta.preprocessor",
                "punctuation.definition.preprocessor",
            ],
            dezSyntaxToken::Primary => vec![],
            dezSyntaxToken::Property => vec![
                "variable.member",
                "support.type.property-name",
                "variable.object.property",
                "variable.other.field",
            ],
            dezSyntaxToken::Punctuation => vec![
                "punctuation",
                "punctuation.section",
                "punctuation.accessor",
                "punctuation.separator",
                "punctuation.definition.tag",
            ],
            dezSyntaxToken::PunctuationBracket => vec![
                "punctuation.bracket",
                "punctuation.definition.tag.begin",
                "punctuation.definition.tag.end",
            ],
            dezSyntaxToken::PunctuationDelimiter => vec![
                "punctuation.delimiter",
                "punctuation.separator",
                "punctuation.terminator",
            ],
            dezSyntaxToken::PunctuationListMarker => {
                vec!["markup.list punctuation.definition.list.begin"]
            }
            dezSyntaxToken::PunctuationSpecial => vec!["punctuation.special"],
            dezSyntaxToken::String => vec!["string"],
            dezSyntaxToken::StringEscape => {
                vec!["string.escape", "constant.character", "constant.other"]
            }
            dezSyntaxToken::StringRegex => vec!["string.regex"],
            dezSyntaxToken::StringSpecial => vec!["string.special", "constant.other.symbol"],
            dezSyntaxToken::StringSpecialSymbol => {
                vec!["string.special.symbol", "constant.other.symbol"]
            }
            dezSyntaxToken::Tag => vec!["tag", "entity.name.tag", "meta.tag.sgml"],
            dezSyntaxToken::TextLiteral => vec!["text.literal", "string"],
            dezSyntaxToken::Title => vec!["title", "entity.name"],
            dezSyntaxToken::Type => vec![
                "entity.name.type",
                "entity.name.type.primitive",
                "entity.name.type.numeric",
                "keyword.type",
                "support.type",
                "support.type.primitive",
                "support.class",
            ],
            dezSyntaxToken::Variable => vec![
                "variable",
                "variable.language",
                "variable.member",
                "variable.parameter",
                "variable.parameter.function-call",
            ],
            dezSyntaxToken::VariableSpecial => vec![
                "variable.special",
                "variable.member",
                "variable.annotation",
                "variable.language",
            ],
            dezSyntaxToken::Variant => vec!["variant"],
        }
    }
}
