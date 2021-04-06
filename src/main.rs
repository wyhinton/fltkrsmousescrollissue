use fltk::{
    app,
    enums::Event,
    prelude::{WidgetBase, WidgetExt, GroupExt, WindowExt},
    window::{GlutWindow, Window},
};
use speedy2d::GLRenderer;

fn main() {
    let app = app::App::default();
    let wind_size =  500;
    let mut main_win = Window::new(450,450,wind_size, wind_size, "");
    let mut win = GlutWindow::default().with_size(wind_size, wind_size).with_pos(0,0);
    win.end();
    main_win.end();
    main_win.show();
    win.make_current();

    
    gl::load_with(|s| win.get_proc_address(s));
    let mut renderer = unsafe { GLRenderer::new_for_current_context((wind_size as u32, wind_size as u32)) }.unwrap();

    let mut win_cl = win.clone();
    win.handle(Box::new(move |ev| match ev {
        Event::Push => {
            dbg!(app::event_x().clone(), app::event_y().clone());
            win_cl.redraw();
            println!("Pushed");
            true
        },
        Event::MouseWheel=>{
            dbg!(app::event_x().clone(), app::event_y().clone());
            win_cl.redraw();
            false
        }
        _ => false,
    }));
    app.run().unwrap();
}
