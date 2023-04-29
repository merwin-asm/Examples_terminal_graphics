#![allow(unused)]



mod terminal_graphics;
use std;
use console::Term;
use colored::Colorize;




fn main() {
    // println!("Hello, world!");
    // terminal_graphics::clear_terminal();
    // println!("{:?}", terminal_graphics::get_terminal_size());
    // terminal_graphics::set_cursor(1, 1);
    // println!("Hello, world!");
    // terminal_graphics::print_pixel([250,250,250]);
    // terminal_graphics::new_row();
    
    
    // terminal_graphics::print_pixels([50,50,50], 50);
    // terminal_graphics::new_row();
    // terminal_graphics::print_pixels([180,50,50], 5);
    // terminal_graphics::new_row();
    // terminal_graphics::print_pixels([180,50,50], 5);
    // terminal_graphics::new_row();
    // terminal_graphics::print_pixels([180,50,50], 5);
    // terminal_graphics::new_row();
    // terminal_graphics::print_pixels([180,50,50], 5);
    // terminal_graphics::new_row();
    // terminal_graphics::print_pixels([50,50,50], 50);
    // terminal_graphics::new_row();
    

    // terminal_graphics::print_success(String::from("Started the program , :]"));
    // terminal_graphics::print_info(String::from("Ur CPU was brough from some fake chinesshe ventor"));
    // terminal_graphics::print_warning(String::from("Daym ur cpu too hot.. bruh"));
    // terminal_graphics::print_error(String::from("... bro fr we down rn"));


    // terminal_graphics::spin();
    // terminal_graphics::spinner(10);
    // terminal_graphics::animated_line(true, 5, [0,80,0]);
    // terminal_graphics::bar_animation(100, [30, 252, 136])


    // println!("Terminal Graphics v 1.0.0  - Test");

    // std::thread::sleep(std::time::Duration::from_millis(1800));


    // terminal_graphics::clear_terminal();
    
    // let mut e: terminal_graphics::RectangularObject = terminal_graphics::RectangularObject{color : [255,255,0], cords: [2,2], height :1, width: 2};
    // e.draw();


    // e.move_obj_w_delay(30,20, 20);
    

    // for ww in 0..10{
    // e.remove();
    // e.cords = [e.cords[0]+5, e.cords[1]];
    // e.draw();
    // std::thread::sleep(std::time::Duration::from_millis(500));
    // e.remove();
    // std::thread::sleep(std::time::Duration::from_millis(500));
    // e.cords = [e.cords[0]+5, e.cords[1]];
    // e.draw();
    // }

    // let mut e:terminal_graphics::DiamondObject = terminal_graphics::DiamondObject{color :[65,25,255], cords:[2,2], radius :51 };
    // e.draw();
    // e.remove();
    
    // Test game

    terminal_graphics::clear_terminal();
    
    let end_x:u16 = terminal_graphics::get_terminal_size().0;
    let end_y:u16 = terminal_graphics::get_terminal_size().1;


    terminal_graphics::set_cursor(end_y,0);

    terminal_graphics::print_color([59, 245, 136], String::from(" Loading : Mutable Box  "));

    terminal_graphics::set_cursor(0,0);

    let mut e: terminal_graphics::RectangularObject = terminal_graphics::RectangularObject{color : [252, 209, 106], cords: [2,2], height :7, width: 6};
    e.draw();


    e.move_obj_w_delay(20,20, 15);
    
    let z:u16 = ((terminal_graphics::get_terminal_size().0)-22)/11;

    for ww in 0..z{
    e.remove();
    e.cords = [e.cords[0]+5, e.cords[1]];
    e.draw();
    std::thread::sleep(std::time::Duration::from_millis(50));
    e.remove();
    std::thread::sleep(std::time::Duration::from_millis(50));
    e.cords = [e.cords[0]+5, e.cords[1]];
    e.draw();
    }


    terminal_graphics::clear_terminal();

    terminal_graphics::set_cursor(end_y,0);

    terminal_graphics::print_color([59, 245, 136], String::from(" `h` For Help  "));

    let mut body: terminal_graphics::RectangularObject = terminal_graphics::RectangularObject{color : [252, 209, 106], cords: [4,3], height :4, width: 8};
    body.draw();

    let stdout = Term::buffered_stdout();
    
    let mut pp:u8 = 0;

    loop{
    if let Ok(character) = stdout.read_char() {
        match character {
            'w' => {
                if body.cords[1] > 3{
                body.move_obj_w_delay(10, body.cords[0], body.cords[1]-1);
            }
            },
            'a' => {
                if body.cords[0] > 4{
                body.move_obj_w_delay(5, body.cords[0] - 1, body.cords[1]);
            }
            },
            's' => {
                if body.cords[1] < end_y-3{
                body.move_obj_w_delay(5, body.cords[0], body.cords[1]+1);
                }
            },
            'd' => {
                if body.cords[0] < end_x-10{
                body.move_obj_w_delay(2, body.cords[0] + 1, body.cords[1]);
                }
            },
            'q' => break,
            'h' =>{
                terminal_graphics::clear_terminal();


                let mut help_box: terminal_graphics::RectangularObject = terminal_graphics::RectangularObject
                {color : [105,100,205], cords: [(end_x/2)/2, (end_y/2)/2 ], height : (end_y/2) as u32, width: (end_x/2) as u32};
                
                help_box.draw();

                terminal_graphics::set_cursor(((end_y/2)/2) + 1, ((end_x/2)/2));
                terminal_graphics::print_color([50, 50, 50],String::from("Help Menu : "));
                

                terminal_graphics::set_cursor(((end_y/2)/2) + 3, ((end_x/2)/2));
                terminal_graphics::print_color([105,100,205],  String::from("\t w - > Up"));

                terminal_graphics::set_cursor(((end_y/2)/2) + 4, ((end_x/2)/2));
                terminal_graphics::print_color([105,100,205], String::from("\t a - > Left"));

                terminal_graphics::set_cursor(((end_y/2)/2) + 5, ((end_x/2)/2));
                terminal_graphics::print_color([105,100,205], String::from("\t s - > Down"));

                terminal_graphics::set_cursor(((end_y/2)/2) + 6, ((end_x/2)/2));
                terminal_graphics::print_color([105,100,205],String::from("\t d - > Right"));


                std::thread::sleep(std::time::Duration::from_millis(4000));
                terminal_graphics::clear_terminal();
                body.draw();

                terminal_graphics::set_cursor(end_y,0);

                terminal_graphics::print_color([59, 245, 136], String::from(" `h` For Help  "));
            
            },
            _ => pp = pp
        }
    }
}

    terminal_graphics::clear_terminal();

    terminal_graphics::print_color([59, 245, 136], String::from(" Ended Game !! \n"));
    
}



