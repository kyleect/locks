use anyhow::{Context, Result};
use crossterm::event::{KeyCode, KeyModifiers};
use nu_ansi_term::{Color, Style};
use reedline::{
    EditCommand, Emacs, FileBackedHistory, PromptEditMode, PromptHistorySearch, Reedline,
    ReedlineEvent, StyledText, ValidationResult,
};
use tree_sitter_highlight::{self, HighlightConfiguration, HighlightEvent};
use tree_sitter_lox::{self, HIGHLIGHTS_QUERY};

use std::borrow::Cow;

pub fn editor() -> Result<Reedline> {
    let mut keybindings = reedline::default_emacs_keybindings();
    keybindings.add_binding(
        KeyModifiers::ALT,
        KeyCode::Enter,
        ReedlineEvent::Edit(vec![EditCommand::InsertNewline]),
    );

    let highlighter = Box::new(Highlighter::new()?);

    let data_dir = dirs::data_dir().context("could not find data directory")?;
    let history_path = data_dir.join("lox/history.txt");
    let history = Box::new(
        FileBackedHistory::with_file(10000, history_path.clone())
            .with_context(|| format!("could not open history file: {}", history_path.display()))?,
    );

    let validator = Box::new(Validator);

    let editor = Reedline::create()
        .with_edit_mode(Box::new(Emacs::new(keybindings)))
        .with_highlighter(highlighter)
        .with_history(history)
        .with_validator(validator);
    Ok(editor)
}

struct PaletteItem<'a> {
    name: &'a str,
    fg: Color,
}

const PALETTE: &[PaletteItem] = &[
    PaletteItem { name: "", fg: Color::White },
    PaletteItem { name: "comment", fg: Color::DarkGray },
    PaletteItem { name: "conditional", fg: Color::LightPurple },
    PaletteItem { name: "constant", fg: Color::LightCyan },
    PaletteItem { name: "field", fg: Color::LightBlue },
    PaletteItem { name: "function", fg: Color::LightBlue },
    PaletteItem { name: "keyword.function", fg: Color::LightPurple },
    PaletteItem { name: "keyword.return", fg: Color::LightPurple },
    PaletteItem { name: "keyword", fg: Color::LightPurple },
    PaletteItem { name: "method", fg: Color::LightBlue },
    PaletteItem { name: "number", fg: Color::LightCyan },
    PaletteItem { name: "operator", fg: Color::White },
    PaletteItem { name: "parameter", fg: Color::LightRed },
    PaletteItem { name: "punctuation.bracket", fg: Color::White },
    PaletteItem { name: "punctuation.delimiter", fg: Color::White },
    PaletteItem { name: "repeat", fg: Color::LightPurple },
    PaletteItem { name: "string", fg: Color::LightGreen },
    PaletteItem { name: "type", fg: Color::LightYellow },
    PaletteItem { name: "variable", fg: Color::LightRed },
];

struct Highlighter {
    config: HighlightConfiguration,
}

impl Highlighter {
    pub fn new() -> Result<Self> {
        let highlight_names = PALETTE.iter().map(|item| item.name).collect::<Vec<_>>();
        let mut config =
            HighlightConfiguration::new(tree_sitter_lox::language(), HIGHLIGHTS_QUERY, "", "")
                .context("failed to create highlight configuration")?;
        config.configure(&highlight_names);
        Ok(Self { config })
    }
}

impl reedline::Highlighter for Highlighter {
    fn highlight(&self, line: &str, _: usize) -> StyledText {
        let mut highlighter = tree_sitter_highlight::Highlighter::new();
        let highlights =
            highlighter.highlight(&self.config, line.as_bytes(), None, |_| None).unwrap();

        let mut output = StyledText::new();
        let mut curr_fg = PALETTE[0].fg;
        let mut curr_end = 0;

        for event in highlights {
            match event {
                Ok(HighlightEvent::HighlightStart(highlight)) => {
                    curr_fg = PALETTE[highlight.0].fg;
                }
                Ok(HighlightEvent::Source { start, end }) => {
                    let style = Style::new().fg(curr_fg);
                    let text = line[start..end].to_string();
                    output.push((style, text));
                    curr_end = end;
                }
                Ok(HighlightEvent::HighlightEnd) => {
                    curr_fg = PALETTE[0].fg;
                }
                Err(_) => {
                    let style = Style::new().fg(PALETTE[0].fg);
                    let text = line.get(curr_end..).unwrap_or_default().to_string();
                    output.push((style, text));
                    break;
                }
            }
        }

        output
    }
}

struct Validator;

impl reedline::Validator for Validator {
    fn validate(&self, line: &str) -> ValidationResult {
        if lox_syntax::is_complete(line) {
            ValidationResult::Complete
        } else {
            ValidationResult::Incomplete
        }
    }
}

pub struct Prompt;

impl reedline::Prompt for Prompt {
    fn render_prompt_left(&self) -> Cow<str> {
        Cow::Borrowed(">>> ")
    }

    fn render_prompt_right(&self) -> Cow<str> {
        Cow::Borrowed("")
    }

    fn render_prompt_indicator(&self, _: PromptEditMode) -> Cow<str> {
        Cow::Borrowed("")
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<str> {
        Cow::Borrowed("... ")
    }

    fn render_prompt_history_search_indicator(&self, _: PromptHistorySearch) -> Cow<str> {
        Cow::Borrowed("")
    }
}