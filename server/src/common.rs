use owo_colors::{Style, OwoColorize};

use crate::LUNA_VER;

pub fn clear_console_and_print_banner() {
    print!("{esc}c", esc = 27 as char);

    let banner_style = Style::new().magenta().bold();
    const BANNER: &str = r#"
             _                                      .-""""-                                   
            | |                                    F   .-'                                    
            | |       _   _   _ __     __ _       F   J                                       
            | |      | | | | | '_ \   / _` |     I    I                                       
            | |____  | |_| | | | | | | (_| |      L   `.                                      
            |______|  \__,_| |_| |_|  \__,_|       L    `-._,        
                                                    `-.__.-'                                                      
    "#;
    println!("\n\n{}", BANNER.style(banner_style));
    println!("\n        =[ luna {}            ]", LUNA_VER.bold());
    println!("+ -- ---=[ Created by gokimax         ]\n");
}