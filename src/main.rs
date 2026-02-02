pub use chip8_emu::Machine;
use ncurses::*;
use crossterm::event;
use std::{time::Duration, io};

const POLLING_DURATION: Duration = Duration::from_millis(100);

fn get_key(polling_duration: Duration) -> io::Result<char> {
    if event::poll(polling_duration)? {
        // It's guaranteed that the `read()` won't block when the `poll()`
        // function returns `true`
        match event::read()? {
            event::Event::Key(event) => {
                if let event::KeyCode::Char(c) = event.code {
                    return Ok(c);
                }
            },
            _ => (),
        }
    }
    Ok('\0') // Return null character if no key was pressed
}

fn main() -> Result<()> {
    println!("Hello, world!");
    let mut machine = Machine::new();
    machine.reset();

    let mut screen = Screen::init()?;
    screen.keypad(true);  // Enable special keys
    screen.noecho()?;     // Don't echo input
    screen.cbreak()?;     // Disable line buffering

    // Create a window (height, width, y, x)
    let mut win = screen.newwin(10, 40, 5, 10)?;

    // Draw a border
    win.box_(0, 0)?;
    // Add content
    win.mvaddstr(1, 2, "Window Title")?;
    win.mvaddstr(3, 2, "This is a window!")?;


    loop {
        let msg = get_key(POLLING_DURATION)?;
        // let ch = screen.getch()?;
        if msg == 'q' {
            break;
        }


        win.mvaddstr(5, 2, &format!("You pressed: {}   ", msg))?;

        // screen.clear()?;

        screen.refresh()?;
        screen.wrefresh(&mut win)?;

    }

    Ok(())
}
