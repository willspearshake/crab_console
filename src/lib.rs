pub mod console_controls {
    const CSI : &str = "\x1b[";

    pub fn ring_bell() {
        print!("{}{}",CSI,'\x07');
    }
    
    pub fn clear_screen() {
        print!("{}2J", CSI);
    }
    
    pub fn move_cursor(row: i8,col: i8) {
        print!("{}{};{}H", CSI, row, col);
    }

    pub fn hide_cursor() {
        print!("{}?25l", CSI);
    }
}