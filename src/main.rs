mod txtbuffer;


use std::{
    io::{self, Write},
    process
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
    Frame,
    layout,
    widgets,
    text,
    style
};
use crossterm::{
    terminal,
    event::{self, KeyEvent},
    execute
};

use txtbuffer::TextBuffer;


struct AppParams
{
    scroll: (u16, u16),
    txt_buf: TextBuffer,
    term_buf: *mut tui::buffer::Buffer,
    terminal_output: String
}


impl AppParams
{
    fn new() -> AppParams
    {
        AppParams { 
            scroll: (0, 0), 
            txt_buf: TextBuffer::new(),
            term_buf: std::ptr::null_mut(),
            terminal_output: String::new()
        }
    }

    fn set_terminal_buffer(&mut self, buffer: &mut tui::buffer::Buffer)
    {
        self.term_buf = buffer;
    }


}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let mut app = AppParams::new();

    terminal::enable_raw_mode().expect("can run in raw mode");

    let mut stdout = io::stdout();

    execute!(stdout, terminal::EnterAlternateScreen)?;


    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    app.set_terminal_buffer(terminal.current_buffer_mut());

    

    // create app and run it
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        terminal::LeaveAlternateScreen
    )?;

    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut AppParams) -> io::Result<()> {
    let mut ui_p: fn(&mut Frame<B>, &mut AppParams, rx: &mut tokio::sync::broadcast::Receiver<String>) = home;
    //let mut txt_buffer = TextBuffer::new();

    let rt = &tokio::runtime::Runtime::new().unwrap();
    let tx: tokio::sync::broadcast::Sender<String>;
    let mut rx: tokio::sync::broadcast::Receiver<String>;

    (tx, rx) = tokio::sync::broadcast::channel(5);


    loop {
        
        terminal.draw(|f| ui_p(f, app, &mut rx))?;
            

        if event::poll(std::time::Duration::from_millis(800)).unwrap()
        {
            if let event::Event::Key(key) = event::read()?
            {

                match key {
                    KeyEvent{code: event::KeyCode::Char('q'), modifiers: event::KeyModifiers::CONTROL} =>{
                        return Ok(());
                    }
    
                    KeyEvent{code: event::KeyCode::Char('n'), modifiers: event::KeyModifiers::CONTROL} =>{
                        ui_p = new_blank;
                    }
    
                    KeyEvent{code: event::KeyCode::Char('h'), modifiers: event::KeyModifiers::CONTROL} =>{
                        ui_p = home;
                    }
    
                    KeyEvent{code: event::KeyCode::Char('t'), modifiers: event::KeyModifiers::CONTROL} =>{
                        rt.spawn(terminal_block(tx.clone()));
                        
                    }
                    
                    KeyEvent{code: event::KeyCode::Down, modifiers: event::KeyModifiers::CONTROL} =>{
                        app.txt_buf.scroll_down();
                    }
    
                    KeyEvent{code: event::KeyCode::Up, modifiers: event::KeyModifiers::CONTROL} =>{
                        app.txt_buf.scroll_up();
                    }
    
                    KeyEvent{code: event::KeyCode::Left, modifiers: event::KeyModifiers::CONTROL} =>{
                        app.txt_buf.scroll_left();
                    }
    
                    KeyEvent{code: event::KeyCode::Right, modifiers: event::KeyModifiers::CONTROL} =>{
                        app.txt_buf.scroll_right();
                    }

                    // This is the last match of CONTROL keys, any event with the CONTROL modifier after this will not be registered
                    KeyEvent{code: event::KeyCode::Char(_), modifiers: event::KeyModifiers::CONTROL} =>{}
    
    
                    KeyEvent{code: event::KeyCode::Down, modifiers: event::KeyModifiers::NONE} =>{
                        app.txt_buf.mov_cursor_down();
                    }
    
                    KeyEvent{code: event::KeyCode::Up, modifiers: event::KeyModifiers::NONE} =>{
                        app.txt_buf.mov_cursor_up();
                    }
    
                    KeyEvent{code: event::KeyCode::Left, modifiers: event::KeyModifiers::NONE} =>{
                        app.txt_buf.mov_cursor_left();
                    }
    
                    KeyEvent{code: event::KeyCode::Right, modifiers: event::KeyModifiers::NONE} =>{
                        app.txt_buf.mov_cursor_right();
                    }
    
                   
                    KeyEvent{code: event::KeyCode::Char(a), modifiers: _} =>{
                        app.txt_buf.cursor_insert(a);
                    }
    
                    KeyEvent{code: event::KeyCode::Tab, modifiers: _} =>{
                        app.txt_buf.cursor_tab();
                    }
    
                    KeyEvent{code: event::KeyCode::Enter, modifiers: _} =>{
                        app.txt_buf.new_line();
                    }
    
                    KeyEvent{code: event::KeyCode::Backspace, modifiers: _} =>{
                        app.txt_buf.cursor_back_del();
                        
                    }
    
                    KeyEvent{code: event::KeyCode::Delete, modifiers: _} =>{
                        app.txt_buf.cursor_del();
                        
                    }
    
                    KeyEvent{code: event::KeyCode::Home, modifiers: _} =>{
                        app.txt_buf.mov_cursor_sol();
                        
                    }
    
                    KeyEvent{code: event::KeyCode::End, modifiers: _} =>{
                        app.txt_buf.mov_cursor_eol();
                        
                    }
                    _ => () 
                } 
            } 
        }
    }
}


fn home<B: Backend>(f: &mut Frame<B>, _app: &mut AppParams, _rx: &mut tokio::sync::broadcast::Receiver<String>) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let size = f.size();

    // Surrounding block
    let block = widgets::Block::default()
        .borders(widgets::Borders::ALL)
        .title("Main block with round corners")
        .title_alignment(layout::Alignment::Center)
        .border_type(widgets::BorderType::Rounded);
    f.render_widget(block, size);

    let chunks = layout::Layout::default()
        .direction(layout::Direction::Vertical)
        .margin(4)
        .constraints([layout::Constraint::Percentage(50), layout::Constraint::Percentage(50)].as_ref())
        .split(f.size());

    // Top two inner blocks
    let top_chunks = layout::Layout::default()
        .direction(layout::Direction::Horizontal)
        .constraints([layout::Constraint::Percentage(50), layout::Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    // Top left inner block with green background
    let block = widgets::Block::default()
        .title(vec![
            text::Span::styled("With", style::Style::default().fg(style::Color::Yellow)),
            text::Span::from(" background"),
        ])
        .style(style::Style::default().bg(style::Color::Green));
    f.render_widget(block, top_chunks[0]);

    // Top right inner block with styled title aligned to the right
    let block = widgets::Block::default()
        .title(text::Span::styled(
            "Styled title",
            style::Style::default()
                .fg(style::Color::White)
                .bg(style::Color::Red)
                .add_modifier(style::Modifier::BOLD),
        ))
        .title_alignment(layout::Alignment::Right);
    f.render_widget(block, top_chunks[1]);

    // Bottom two inner blocks
    let bottom_chunks = layout::Layout::default()
        .direction(layout::Direction::Horizontal)
        .constraints([layout::Constraint::Percentage(50), layout::Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // Bottom left block with all default borders
    let block = widgets::Block::default().title("With borders").borders(widgets::Borders::ALL);
    f.render_widget(block, bottom_chunks[0]);

    // Bottom right block with styled left and right border
    let block = widgets::Block::default()
        .title("With styled borders and doubled borders")
        .border_style(style::Style::default().fg(style::Color::Cyan))
        .borders(widgets::Borders::LEFT | widgets::Borders::RIGHT)
        .border_type(widgets::BorderType::Double);
    f.render_widget(block, bottom_chunks[1]);
}

fn new_blank<B: Backend>(f: &mut Frame<B>, app: &mut AppParams, rx: &mut tokio::sync::broadcast::Receiver<String>) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let size = f.size();

    let body: layout::Rect;
    let header: layout::Rect;

    let chunks = layout::Layout::default()
    .direction(layout::Direction::Vertical)
    .margin(0)
    .constraints([layout::Constraint::Percentage(80), layout::Constraint::Percentage(20)].as_ref())
    .split(size);

    body = chunks[0]; header = chunks[1];
    
    
    // Surrounding block
    let body_block = widgets::Block::default()
        .borders(widgets::Borders::ALL)
        .title("New File")
        .title_alignment(layout::Alignment::Center)
        .border_type(widgets::BorderType::Rounded);
    f.render_widget(body_block.clone(), body);

    let header_block = widgets::Block::default()
        .borders(widgets::Borders::ALL)
        .title("Terminal")
        .title_alignment(layout::Alignment::Center)
        .border_type(widgets::BorderType::Rounded);
    f.render_widget(header_block, header);
    
    // TODO: change for a .checked_log10() once the feature is stable
    let width = app.txt_buf.lines().to_string().len();

    let body_text = app.txt_buf.to_string();
    
    let chunks = layout::Layout::default()
        .direction(layout::Direction::Horizontal)
        .margin(0)
        .constraints([layout::Constraint::Length(width as u16 + 5), layout::Constraint::Percentage(100)].as_ref())
        .split(body);
    
    let (number_column, body_column) = (chunks[0], chunks[1]);

    
    let body_paragraph = widgets::Paragraph::new(body_text)
        .block(widgets::Block::default())
        .scroll(app.txt_buf.get_scroll());
    
    
    app.txt_buf.set_text_area(body_column.inner(&layout::Margin{vertical: 1, horizontal: 1}));

    let mut number_text: String = String::new();

    for x in 1..app.txt_buf.lines()+1
    {
        number_text.push_str(format!("{:>width$}  :", x).as_str());
        number_text.push('\n');
    }

    let number_paragraph = widgets::Paragraph::new(number_text)
        .block(widgets::Block::default())
        .scroll((app.txt_buf.get_scroll().0, 0))
        .style(style::Style::default().add_modifier(style::Modifier::DIM));


    
    app.terminal_output = rx.try_recv().unwrap_or(app.terminal_output.clone()).into();
    let header_text = app.terminal_output.clone();
    let header_paragraph = widgets::Paragraph::new(header_text)
        .block(widgets::Block::default());

    f.render_widget(body_paragraph, body_column.inner(&layout::Margin{vertical: 1, horizontal: 1}));
    f.render_widget(number_paragraph, number_column.inner(&layout::Margin{vertical: 1, horizontal: 1}));
    f.render_widget(header_paragraph, header.inner(&layout::Margin{vertical: 1, horizontal: 2}));

    let (cursor_x, cursor_y) = app.txt_buf.get_cursor_pos();

    f.set_cursor(cursor_x, cursor_y);

}

async fn terminal_block(tx: tokio::sync::broadcast::Sender<String>)
{
    let out: process::Output = process::Command::new("cmd")
        .args(["/C", "ping www.google.com"])
        .output()
        .expect("failed to execute process");
    let msg =  String::from_utf8(out.stdout).unwrap();
    
    tx.send(msg).unwrap();
}
