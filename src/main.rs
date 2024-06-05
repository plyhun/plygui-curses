extern crate pancurses;

use pancurses::*;

fn main() {
    let mut window = initscr();

    window.keypad(true); // Set keypad mode
    mousemask(ALL_MOUSE_EVENTS, None); // Listen to all mouse events

    window.printw("Click in the terminal, press q to exit\n");
    window.refresh();
    window.draw_box(0, 0);

    loop {
        match window.getch() {
            Some(Input::KeyMouse) => {
                if let Ok(mouse_event) = getmouse() {
                    window.mvprintw(1, 1,
                                    &format!("Mouse at {},{}", mouse_event.x, mouse_event.y),
                    );
                    if (mouse_event.bstate & BUTTON4_PRESSED) > 0 {
                        window.mvprintw(1, 1,
                                        &format!("Scrolled up {},{}", mouse_event.x, mouse_event.y),
                        );
                    } else if (mouse_event.bstate & BUTTON5_PRESSED) > 0 {
                        window.mvprintw(1, 1,
                                        &format!("Scrolled down {},{}", mouse_event.x, mouse_event.y),
                        );
                    }
                };
            }
            Some(Input::KeyResize) => {
                window.draw_box(0, 0);
                let size = window.get_max_yx();
                window.mvprintw(1, 1,
                    &format!("Resized to {},{}", size.0, size.1),    );
            }
            Some(Input::Character(x)) if x == 'q' => break,
            _ => (),
        }
    }
    endwin();
}