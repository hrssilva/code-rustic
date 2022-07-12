use tui::{
    layout
};

pub struct TextBuffer
{
    area: layout::Rect,
    char_matrix: Vec<String>,
    cursor_pos: (u16, u16),
    scroll: (u16, u16),
    selected_start: (u16, u16),
    selected_end: (u16, u16)
}

impl TextBuffer
{
    pub fn new() -> TextBuffer
    {
        TextBuffer { 
            area: layout::Rect::new(0, 0, 0, 0), 
            char_matrix: Vec::new(), 
            cursor_pos: (0, 0),
            scroll: (0, 0),
            selected_start: (0, 0),
            selected_end: (0, 0)
        }
    }

    pub fn set_text_area(&mut self, rect: layout::Rect)
    {
            self.area = rect;

            if self.char_matrix.len() < 1 {self.push_new_line();}
    }

    pub fn mov_cursor_right(&mut self)
    {
        if usize::from(self.cursor_pos.0) < self.char_matrix[usize::from(self.cursor_pos.1)].chars().count()
        {
            self.cursor_pos.0 += 1;
        }
        if self.get_cursor_pos().0 >= self.area.right()
        {
            self.scroll_right();
        }
    }

    pub fn mov_cursor_left(&mut self)
    {
        if self.get_cursor_pos().0 <= self.area.left()
        {
            self.scroll_left();
        }
        self.cursor_pos.0 = self.cursor_pos.0.saturating_sub(1);
    }

    pub fn mov_cursor_up(&mut self)
    {
        self.cursor_pos.1 = self.cursor_pos.1.saturating_sub(1);

        if self.get_cursor_pos().1 < self.area.top()
        {
            self.scroll_up()
        }

        if usize::from(self.cursor_pos.0) > self.char_matrix[usize::from(self.cursor_pos.1)].len()
        {
            self.mov_cursor_eol()
        }
    }

    pub fn mov_cursor_down(&mut self)
    {
        if usize::from(self.cursor_pos.1) < self.char_matrix.len() -1
        {
            self.cursor_pos.1 += 1;
        }

        if self.get_cursor_pos().1 >= self.area.bottom()
        {
            self.scroll_down()
        }

        if usize::from(self.cursor_pos.0) > self.char_matrix[usize::from(self.cursor_pos.1)].len()
        {
            self.mov_cursor_eol()
        }
    }

    pub fn mov_cursor_eol(&mut self)
    {
        self.cursor_pos.0 = self.char_matrix[usize::from(self.cursor_pos.1)].len() as u16;
        self.scroll.1 = self.get_line(self.cursor_pos.1.into()).len().saturating_sub(usize::from(self.area.width) - 1/* -1 is compensation for 0 starting index */) as u16;
    }

    pub fn mov_cursor_sol(&mut self)
    {
        self.cursor_pos.0 = 0;
        self.scroll.1 = 0;
    }

    pub fn get_cursor_pos(&self) -> (u16, u16)
    {
        (self.cursor_pos.0 + self.area.left() - self.scroll.1, self.cursor_pos.1 + self.area.top() - self.scroll.0)
    }

    pub fn scroll_right(&mut self)
    {
        if self.char_matrix[usize::from(self.cursor_pos.1)].len() >= (self.area.width + self.scroll.1).into()
        {
            self.scroll.1 += 1;
        }

        if self.get_cursor_pos().0 < self.area.left() // compensation for scroll reliant final cursor position
        {
            self.mov_cursor_right();
        }
    }

    pub fn scroll_left(&mut self)
    {
        self.scroll.1 = self.scroll.1.saturating_sub(1);

        if self.get_cursor_pos().0 >= self.area.right() // compensation for scroll reliant final cursor position
        {
            self.mov_cursor_left();
        }
    }

    pub fn scroll_up(&mut self)
    {
        self.scroll.0 = self.scroll.0.saturating_sub(1);

        if self.get_cursor_pos().1 >= self.area.bottom() // compensation for scroll reliant final cursor position
        {
            self.mov_cursor_up();
        }
    }

    pub fn scroll_down(&mut self)
    {
        if self.char_matrix.len()-1 > self.scroll.0.into()
        {
            self.scroll.0 += 1;
        }
        if self.get_cursor_pos().1 < self.area.top() // compensation for scroll reliant final cursor position
        {
            self.mov_cursor_down();
        }
    }

    pub fn get_scroll(&self) -> (u16, u16)
    {
        self.scroll.clone()
    }

    pub fn get_line(&self, nth: usize) -> &String
    {
        &self.char_matrix[nth]
    }

    pub fn to_string(&self) -> String
    {
        let mut s = String::new();
        for i in 0..self.char_matrix.len()
        {
            s.push_str(self.get_line(i).as_str());
            s.push('\n');
        }

        return s;
    }

    pub fn cursor_insert(&mut self, ch: char)
    {
        let (x16, y16) = self.cursor_pos.into();
        let x: usize = x16.into();
        let y: usize = y16.into();
        self.char_matrix[y].insert(x, ch);

        self.mov_cursor_right();
    }

    pub fn cursor_tab(&mut self)
    {
        let (x16, y16) = self.cursor_pos.into();
        let x: usize = x16.into();
        let y: usize = y16.into();
        self.char_matrix[y].insert_str(x, "    ");

        self.mov_cursor_right();
        self.mov_cursor_right();
        self.mov_cursor_right();
        self.mov_cursor_right();
    }

    pub fn new_line(&mut self)
    {
        self.break_line();
        self.mov_cursor_down();
        self.cursor_pos.0 = 0;
    }

    pub fn break_line(&mut self)
    {
        let (u_posx, u_posy) = (usize::from(self.cursor_pos.0), usize::from(self.cursor_pos.1));

        let s = self.char_matrix[u_posy].clone();

        let (s_old, s_new) = s.split_at(u_posx);

        let (mut s_old, mut s_new) = (s_old.to_string(), s_new.to_string());

        s_old.reserve(usize::from(self.area.right()).saturating_sub(s_old.len()));
        s_new.reserve(usize::from(self.area.right()).saturating_sub(s_new.len()));

        self.char_matrix[u_posy] = s_old.to_string();

        self.char_matrix.insert(u_posy+1, s_new.to_string());
    }

    pub fn push_line(&mut self, mut s: String)
    {
        s.reserve(usize::from(self.area.right()).saturating_sub(s.len()));

        self.char_matrix.push(s);
    }

    pub fn push_new_line(&mut self)
    {
        self.char_matrix.push(String::with_capacity(self.area.right().into()));
    }

    pub fn cursor_del(&mut self)
    {
        let len = self.char_matrix[usize::from(self.cursor_pos.1)].len();

        if len < 1 && usize::from(self.cursor_pos.1) + 1 < self.char_matrix.len()
        {
            self.char_matrix.remove(self.cursor_pos.1.into());
        }
        else if usize::from(self.cursor_pos.0) < len
        {   
            let (x16, y16) = self.cursor_pos.into();
            let x: usize = x16.into();
            let y: usize = y16.into();
            
            self.char_matrix[y].remove(x);
        }
    }

    pub fn cursor_back_del(&mut self)
    {   
        let u_posy = usize::from(self.cursor_pos.1);
        let s = self.char_matrix[u_posy].clone();
        
        if self.cursor_pos.0 == 0
        {
            if self.cursor_pos.1 != 0
            {
                self.mov_cursor_up();
                self.mov_cursor_eol();
                
                self.char_matrix[u_posy-1].push_str(s.as_str());
                
                self.char_matrix.remove(u_posy);
            }
        }
        else
        {
            self.mov_cursor_left();
            
            self.cursor_del();
        }

    }

    pub fn lines(&self) -> usize
    {
        self.char_matrix.len()
    }

    // TODO: decide if this method is usefull, if yes, actually implement it
    pub fn _from_str(&mut self, s: &str)
    {
        let mut lines = s.lines();
        let mut temp_matrix: Vec<String> = Vec::new();

        loop {
            let line = lines.next();
            if line == None
            {
                break;
            }

            temp_matrix.push(line.unwrap().into());
        }
    }

    pub fn clear(&mut self)
    {
        self.area = layout::Rect::new(0, 0, 0, 0);
        self.char_matrix = Vec::new();
        self.cursor_pos = (0, 0);
    }
}

