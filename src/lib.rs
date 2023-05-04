pub mod console_controls {
    const CSI : &str = "\x1b[";

    pub fn ring_bell() {
        print!("{}{}",CSI,'\x07');
    }
    
    pub fn clear_screen() {
        print!("{}2J", CSI);
    }
    
    pub fn move_cursor(row_col: (i8, i8)) {
        let (row, col) = row_col;
        print!("{}{};{}H", CSI, row, col);
    }
}