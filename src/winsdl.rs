use sdl2::{video::{gl_attr, GLContext, SwapInterval, Window}, EventPump, Sdl};

pub struct Winsdl {
    pub sdl_context: Sdl,
    pub window: Window,
    pub gl_context: GLContext,
    pub gl: (),
    pub event_pump: EventPump,
}

impl Winsdl {
    pub fn new(width: usize, height: usize) -> Result<Self, &'static str> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = video_subsystem
            .window("Rust!", width as u32, height as u32)
            .resizable()
            .opengl()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap(); // create opengl context
        let gl = gl::load_with(|s| { // load opengl functions
            video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        });

        window
            .subsystem()
            .gl_set_swap_interval(SwapInterval::VSync) // limit framerate to screen refresh rate
            .unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        Ok(Winsdl {
            sdl_context,
            window,
            gl_context,
            gl,
            event_pump,
        })
    }
}