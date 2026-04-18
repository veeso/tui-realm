use pretty_assertions::assert_eq;
use tui_realm_textarea::{
    TEXTAREA_CMD_DEL_LINE_BY_END, TEXTAREA_CMD_DEL_LINE_BY_HEAD, TEXTAREA_CMD_DEL_NEXT_WORD,
    TEXTAREA_CMD_DEL_WORD, TEXTAREA_CMD_MOVE_BOTTOM, TEXTAREA_CMD_MOVE_PARAGRAPH_BACK,
    TEXTAREA_CMD_MOVE_PARAGRAPH_FORWARD, TEXTAREA_CMD_MOVE_TOP, TEXTAREA_CMD_MOVE_WORD_BACK,
    TEXTAREA_CMD_MOVE_WORD_FORWARD, TEXTAREA_CMD_NEWLINE, TEXTAREA_CMD_REDO, TEXTAREA_CMD_UNDO,
    TextArea,
};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::Component;
use tuirealm::props::{Borders, Color, Style, Title};
use tuirealm::ratatui::layout::Size;
use tuirealm::state::{State, StateValue};
use tuirealm::testing::render_to_string;

#[test]
fn test_textarea_initial_state_empty() {
    let component = TextArea::default();
    let state = component.state();
    assert!(matches!(state, State::Vec(_)));
}

#[test]
fn test_textarea_type_char() {
    let mut component = TextArea::default();
    let result = component.perform(Cmd::Type('H'));
    assert!(matches!(result, CmdResult::Changed(_)));
    let state = component.state();
    if let State::Vec(lines) = state {
        assert_eq!(lines[0], StateValue::String("H".to_string()));
    } else {
        panic!("Expected State::Vec");
    }
}

#[test]
fn test_textarea_type_multiple_chars() {
    let mut component = TextArea::default();
    component.perform(Cmd::Type('H'));
    component.perform(Cmd::Type('i'));
    let state = component.state();
    if let State::Vec(lines) = state {
        assert_eq!(lines[0], StateValue::String("Hi".to_string()));
    } else {
        panic!("Expected State::Vec");
    }
}

#[test]
fn test_textarea_paste() {
    let mut component = TextArea::default();
    component.paste("Hello, World!");
    let state = component.state();
    if let State::Vec(lines) = state {
        assert_eq!(lines[0], StateValue::String("Hello, World!".to_string()));
    } else {
        panic!("Expected State::Vec");
    }
}

#[test]
fn test_textarea_paste_multiline() {
    let mut component = TextArea::default();
    component.paste("Line 1\nLine 2\nLine 3");
    let state = component.state();
    if let State::Vec(lines) = state {
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], StateValue::String("Line 1".to_string()));
        assert_eq!(lines[1], StateValue::String("Line 2".to_string()));
        assert_eq!(lines[2], StateValue::String("Line 3".to_string()));
    } else {
        panic!("Expected State::Vec");
    }
}

#[test]
fn test_textarea_newline() {
    let mut component = TextArea::default();
    component.perform(Cmd::Type('A'));
    component.perform(Cmd::Custom(TEXTAREA_CMD_NEWLINE));
    component.perform(Cmd::Type('B'));
    let state = component.state();
    if let State::Vec(lines) = state {
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], StateValue::String("A".to_string()));
        assert_eq!(lines[1], StateValue::String("B".to_string()));
    } else {
        panic!("Expected State::Vec");
    }
}

#[test]
fn test_textarea_delete() {
    let mut component = TextArea::default();
    component.perform(Cmd::Type('A'));
    component.perform(Cmd::Type('B'));
    let result = component.perform(Cmd::Delete);
    assert!(matches!(result, CmdResult::Changed(_)));
    let state = component.state();
    if let State::Vec(lines) = state {
        assert_eq!(lines[0], StateValue::String("A".to_string()));
    } else {
        panic!("Expected State::Vec");
    }
}

#[test]
fn test_textarea_cancel_deletes_forward() {
    let mut component = TextArea::default();
    component.perform(Cmd::Type('A'));
    component.perform(Cmd::Type('B'));
    component.perform(Cmd::GoTo(Position::Begin));
    let result = component.perform(Cmd::Cancel);
    assert!(matches!(result, CmdResult::Changed(_)));
}

#[test]
fn test_textarea_cursor_movement() {
    let mut component = TextArea::default();
    component.perform(Cmd::Type('A'));
    component.perform(Cmd::Type('B'));
    assert_eq!(
        component.perform(Cmd::Move(Direction::Left)),
        CmdResult::Visual
    );
    assert_eq!(
        component.perform(Cmd::Move(Direction::Right)),
        CmdResult::Visual
    );
    assert_eq!(
        component.perform(Cmd::GoTo(Position::Begin)),
        CmdResult::Visual
    );
    assert_eq!(
        component.perform(Cmd::GoTo(Position::End)),
        CmdResult::Visual
    );
}

#[test]
fn test_textarea_vertical_movement() {
    let mut component = TextArea::default();
    component.perform(Cmd::Type('A'));
    component.perform(Cmd::Custom(TEXTAREA_CMD_NEWLINE));
    component.perform(Cmd::Type('B'));
    assert_eq!(
        component.perform(Cmd::Move(Direction::Up)),
        CmdResult::Visual
    );
    assert_eq!(
        component.perform(Cmd::Move(Direction::Down)),
        CmdResult::Visual
    );
}

#[test]
fn test_textarea_undo_redo() {
    let mut component = TextArea::default();
    component.perform(Cmd::Type('A'));
    let undo_result = component.perform(Cmd::Custom(TEXTAREA_CMD_UNDO));
    assert!(matches!(undo_result, CmdResult::Changed(_)));
    let redo_result = component.perform(Cmd::Custom(TEXTAREA_CMD_REDO));
    assert!(matches!(redo_result, CmdResult::Changed(_)));
}

#[test]
fn test_textarea_word_movement() {
    let mut component = TextArea::default();
    for ch in "hello world".chars() {
        component.perform(Cmd::Type(ch));
    }
    assert_eq!(
        component.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_WORD_BACK)),
        CmdResult::Visual
    );
    assert_eq!(
        component.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_WORD_FORWARD)),
        CmdResult::Visual
    );
}

#[test]
fn test_textarea_paragraph_movement() {
    let mut component = TextArea::default();
    component.paste("line 1\nline 2\nline 3");

    assert_eq!(
        component.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_PARAGRAPH_BACK)),
        CmdResult::Visual
    );
    assert_eq!(
        component.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_PARAGRAPH_FORWARD)),
        CmdResult::Visual
    );
}

#[test]
fn test_textarea_delete_word() {
    let mut component = TextArea::default();
    for ch in "hello".chars() {
        component.perform(Cmd::Type(ch));
    }
    let result = component.perform(Cmd::Custom(TEXTAREA_CMD_DEL_WORD));
    assert!(matches!(result, CmdResult::Changed(_)));
}

#[test]
fn test_textarea_del_next_word() {
    let mut component = TextArea::default();
    for ch in "hello world".chars() {
        component.perform(Cmd::Type(ch));
    }
    component.perform(Cmd::GoTo(Position::Begin));
    let result = component.perform(Cmd::Custom(TEXTAREA_CMD_DEL_NEXT_WORD));
    assert!(matches!(result, CmdResult::Changed(_)));
}

#[test]
fn test_textarea_del_line_by_end() {
    let mut component = TextArea::default();
    component.perform(Cmd::Type('A'));
    component.perform(Cmd::Type('B'));
    component.perform(Cmd::GoTo(Position::Begin));
    let result = component.perform(Cmd::Custom(TEXTAREA_CMD_DEL_LINE_BY_END));
    assert!(matches!(result, CmdResult::Changed(_)));
}

#[test]
fn test_textarea_del_line_by_head() {
    let mut component = TextArea::default();
    component.perform(Cmd::Type('A'));
    component.perform(Cmd::Type('B'));
    let result = component.perform(Cmd::Custom(TEXTAREA_CMD_DEL_LINE_BY_HEAD));
    assert!(matches!(result, CmdResult::Changed(_)));
}

#[test]
fn test_textarea_move_top_bottom() {
    let mut component = TextArea::default();
    component.perform(Cmd::Type('A'));
    component.perform(Cmd::Custom(TEXTAREA_CMD_NEWLINE));
    component.perform(Cmd::Type('B'));
    component.perform(Cmd::Custom(TEXTAREA_CMD_NEWLINE));
    component.perform(Cmd::Type('C'));
    assert_eq!(
        component.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_TOP)),
        CmdResult::Visual
    );
    assert_eq!(
        component.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_BOTTOM)),
        CmdResult::Visual
    );
}

#[test]
fn test_textarea_submit() {
    let mut component = TextArea::default();
    component.perform(Cmd::Type('A'));
    let result = component.perform(Cmd::Submit);
    assert!(matches!(result, CmdResult::Submit(_)));
}

#[test]
fn test_textarea_scroll() {
    let mut component = TextArea::default().scroll_step(2);
    component.perform(Cmd::Type('A'));
    component.perform(Cmd::Custom(TEXTAREA_CMD_NEWLINE));
    component.perform(Cmd::Type('B'));
    component.perform(Cmd::Custom(TEXTAREA_CMD_NEWLINE));
    component.perform(Cmd::Type('C'));
    assert_eq!(
        component.perform(Cmd::Scroll(Direction::Up)),
        CmdResult::Visual
    );
    assert_eq!(
        component.perform(Cmd::Scroll(Direction::Down)),
        CmdResult::Visual
    );
}

#[test]
fn test_textarea_unhandled_cmd() {
    let mut component = TextArea::default();
    assert_eq!(
        component.perform(Cmd::Toggle),
        CmdResult::Invalid(Cmd::Toggle)
    );
}

// Snapshot tests

#[test]
fn test_textarea_snapshot_singleline() {
    let mut component = TextArea::default()
        .borders(Borders::default())
        .title(Title::from("Editor"))
        .style(Style::default().fg(Color::White));
    for ch in "Hello, World!".chars() {
        component.perform(Cmd::Type(ch));
    }
    let rendered = render_to_string(&mut component, Size::new(40, 8));
    insta::assert_snapshot!("textarea_crate_singleline", rendered);
}

#[test]
fn test_textarea_snapshot_multiline() {
    let mut component = TextArea::default()
        .borders(Borders::default())
        .title(Title::from("Editor"));
    for ch in "Line 1".chars() {
        component.perform(Cmd::Type(ch));
    }
    component.perform(Cmd::Custom(TEXTAREA_CMD_NEWLINE));
    for ch in "Line 2".chars() {
        component.perform(Cmd::Type(ch));
    }
    component.perform(Cmd::Custom(TEXTAREA_CMD_NEWLINE));
    for ch in "Line 3".chars() {
        component.perform(Cmd::Type(ch));
    }
    let rendered = render_to_string(&mut component, Size::new(40, 8));
    insta::assert_snapshot!("textarea_crate_multiline", rendered);
}

#[test]
fn test_textarea_snapshot_empty() {
    let mut component = TextArea::default()
        .borders(Borders::default())
        .title(Title::from("Empty"));
    let rendered = render_to_string(&mut component, Size::new(40, 8));
    insta::assert_snapshot!("textarea_crate_empty", rendered);
}
