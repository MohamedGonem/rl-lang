//! Converts the [`OutputLine`] buffer into ratatui [`Line`]s for rendering.
//!
//! `ValidInput` lines are intentionally filtered out - they exist only for
//! `:save` and are never shown in the output area.
//!
//! `Result` lines run through [`highlight`] first; if the highlighter returns
//! only error or plain-text spans (i.e. it didn't actually recognize any
//! code tokens), the line is rendered as plain success-colored text instead.

use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};

use crate::{lines_types::OutputLine, syntax_highlighting::highlight, theme};

/// Converts the output buffer into a list of styled ratatui lines ready to render.
pub fn render_output(output: &[OutputLine]) -> Vec<Line<'static>> {
    output
        .iter()
        .filter_map(|line| match line {
            OutputLine::Input(s) => {
                // strips ".. " (the stored continuation marker - independent
                // of the glyphs actually shown below)
                let (prefix, prefix_color, code) = if let Some(stripped) = s.strip_prefix(".. ") {
                    ("· ", theme::ACCENT2, stripped)
                } else {
                    ("❯ ", theme::ACCENT, s.as_str())
                };
                let mut spans = vec![Span::styled(
                    prefix,
                    Style::default()
                        .fg(prefix_color)
                        .add_modifier(Modifier::BOLD),
                )];
                spans.extend(highlight(code));
                Some(Line::from(spans))
            }
            OutputLine::ValidInput(_) => None,
            OutputLine::Result(s) => {
                // try to highlight if it is correct
                let spans = highlight(s);
                // highlight() falls back to a single error-colored span on
                // lex failure, and un-recognized chars fall through to
                // theme::TEXT - if every span is one of those two, treat
                // the line as plain text rather than "code we recognized".
                let is_code = spans.iter().any(|sp| {
                    sp.style
                        .fg
                        .map(|c| c != theme::ERROR && c != theme::TEXT)
                        .unwrap_or(false)
                });
                if is_code {
                    Some(Line::from(spans))
                } else {
                    Some(Line::from(Span::styled(
                        s.clone(),
                        Style::default().fg(theme::SUCCESS),
                    )))
                }
            }
            OutputLine::Error(s) => Some(Line::from(vec![
                Span::styled(
                    "✗ ",
                    Style::default()
                        .fg(theme::ERROR)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(s.clone(), Style::default().fg(theme::ERROR)),
            ])),
            OutputLine::Info(s) => Some(Line::from(Span::styled(
                s.clone(),
                Style::default().fg(theme::INFO),
            ))),
            OutputLine::Separator => Some(Line::from(Span::styled(
                "─".repeat(40),
                Style::default().fg(theme::TEXT_MUTED),
            ))),
            OutputLine::Styled(parts) => Some(Line::from(
                parts
                    .iter()
                    .map(|(text, style)| Span::styled(text.clone(), *style))
                    .collect::<Vec<_>>(),
            )),
        })
        .collect()
}
