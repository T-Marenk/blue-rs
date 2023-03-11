use tui::{
    backend::Backend,
    Frame,
    layout::{Layout, Constraint, Direction, Rect},
    widgets::{Block, Borders, Table, Row, Cell, TableState},
    style::{Style, Color, Modifier},
    text::{Span, Spans}
};

pub fn draw<B: Backend>(f: &mut Frame<B>) {
    let constraints: Vec<Constraint> = vec![Constraint::Percentage(10), Constraint::Percentage(90)];
    let parts = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(f.size());
    
    let status = "On";
    draw_state_block(f, status, &parts);
}

fn draw_state_block<B: Backend>(f: &mut Frame<B>, status: &str, parts: &Vec<Rect>) {
    let rows = vec![
        Row::new(vec![""]),
        Row::new(vec!["Bluetooth status", status])
    ];
    let mut state = TableState::default();
    let table = Table::new(rows)
        .widths(&[Constraint::Length(16), Constraint::Length(5), Constraint::Length(5)])
        .block(Block::default().title("Blue-rs").borders(Borders::ALL));
    f.render_stateful_widget(table, parts[0], &mut state);
}