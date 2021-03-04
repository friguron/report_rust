
mod ui_interface
{

    #[derive(Debug, Default)]
    pub struct Dimensions {
        pub x_pos: u16,
        pub y_pos: u16,
        pub width: u16,
        pub height: u16,
    }

    #[derive(Debug, Default)]
    pub struct RGBAColor
    {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub a: u8,
    }

    #[derive(Debug, Default)]
    pub struct Palette
    {
        pub colors: Vec<RGBAColor>

    }

    impl Palette
    {
        pub fn build(&mut self, pal_type:&str)
        {
            if pal_type=="256"
            {
                self.colors.push(RGBAColor{r:0,g:0,b:0,a:255});
                self.colors.push(RGBAColor{r:0,g:0,b:170,a:255});
                self.colors.push(RGBAColor{r:0,g:170,b:0,a:255});
                self.colors.push(RGBAColor{r:0,g:170,b:170,a:255});
                self.colors.push(RGBAColor{r:170,g:0,b:0,a:255});
                self.colors.push(RGBAColor{r:170,g:0,b:170,a:255});
                self.colors.push(RGBAColor{r:170,g:85,b:0,a:255});
                self.colors.push(RGBAColor{r:170,g:170,b:170,a:255});

                self.colors.push(RGBAColor{r:85,g:85,b:85,a:255});
                self.colors.push(RGBAColor{r:85,g:85,b:255,a:255});
                self.colors.push(RGBAColor{r:85,g:255,b:85,a:255});
                self.colors.push(RGBAColor{r:85,g:255,b:255,a:255});
                self.colors.push(RGBAColor{r:255,g:85,b:85,a:255});
                self.colors.push(RGBAColor{r:255,g:85,b:255,a:255});
                self.colors.push(RGBAColor{r:255,g:255,b:85,a:255});
                self.colors.push(RGBAColor{r:255,g:255,b:255,a:255});


                self.colors.push(RGBAColor{r:239,g:239,b:239,a:255});  // ok 64
                self.colors.push(RGBAColor{r:225,g:225,b:225,a:255});
                self.colors.push(RGBAColor{r:211,g:211,b:211,a:255});
                self.colors.push(RGBAColor{r:198,g:198,b:198,a:255});
                self.colors.push(RGBAColor{r:184,g:184,b:184,a:255});  // 1/2
                self.colors.push(RGBAColor{r:170,g:170,b:170,a:255});
                self.colors.push(RGBAColor{r:156,g:156,b:156,a:255});
                self.colors.push(RGBAColor{r:142,g:142,b:142,a:255});

                self.colors.push(RGBAColor{r:129,g:129,b:129,a:255});
                self.colors.push(RGBAColor{r:115,g:115,b:115,a:255});
                self.colors.push(RGBAColor{r:101,g:101,b:101,a:255});
                self.colors.push(RGBAColor{r:87,g:87,b:87,a:255});
                self.colors.push(RGBAColor{r:73,g:73,b:73,a:255});
                self.colors.push(RGBAColor{r:60,g:60,b:60,a:255});
                self.colors.push(RGBAColor{r:46,g:46,b:46,a:255});
                self.colors.push(RGBAColor{r:32,g:32,b:32,a:255});


                self.colors.push(RGBAColor{r:255,g:0,b:0,a:255});
                self.colors.push(RGBAColor{r:242,g:0,b:0,a:255});
                self.colors.push(RGBAColor{r:230,g:0,b:0,a:255});
                self.colors.push(RGBAColor{r:217,g:0,b:0,a:255});
                self.colors.push(RGBAColor{r:204,g:0,b:0,a:255});
                self.colors.push(RGBAColor{r:192,g:0,b:0,a:255});
                self.colors.push(RGBAColor{r:179,g:0,b:0,a:255});
                self.colors.push(RGBAColor{r:166,g:0,b:0,a:255});

                self.colors.push(RGBAColor{r:154,g:0,b:0,a:255});
                self.colors.push(RGBAColor{r:141,g:0,b:0,a:255});
                self.colors.push(RGBAColor{r:128,g:0,b:0,a:255});
                self.colors.push(RGBAColor{r:116,g:0,b:0,a:255});
                self.colors.push(RGBAColor{r:103,g:0,b:0,a:255});
                self.colors.push(RGBAColor{r:90,g:0,b:0,a:255});
                self.colors.push(RGBAColor{r:78,g:0,b:0,a:255});
                self.colors.push(RGBAColor{r:65,g:0,b:0,a:255});


                self.colors.push(RGBAColor{r:255,g:219,b:219,a:255});
                self.colors.push(RGBAColor{r:255,g:188,b:188,a:255});
                self.colors.push(RGBAColor{r:255,g:156,b:156,a:255});
                self.colors.push(RGBAColor{r:255,g:125,b:125,a:255});
                self.colors.push(RGBAColor{r:255,g:94,b:94,a:255});
                self.colors.push(RGBAColor{r:255,g:63,b:63,a:255});
                self.colors.push(RGBAColor{r:255,g:31,b:31,a:255});
                self.colors.push(RGBAColor{r:255,g:0,b:0,a:255});

                self.colors.push(RGBAColor{r:255,g:170,b:93,a:255});
                self.colors.push(RGBAColor{r:255,g:164,b:65,a:255});
                self.colors.push(RGBAColor{r:255,g:138,b:32,a:255});
                self.colors.push(RGBAColor{r:255,g:121,b:0,a:255});
                self.colors.push(RGBAColor{r:231,g:109,b:0,a:255});
                self.colors.push(RGBAColor{r:207,g:97,b:0,a:255});
                self.colors.push(RGBAColor{r:182,g:85,b:0,a:255});
                self.colors.push(RGBAColor{r:158,g:77,b:0,a:255});


                self.colors.push(RGBAColor{r:255,g:255,b:219,a:255});
                self.colors.push(RGBAColor{r:255,g:255,b:186,a:255});
                self.colors.push(RGBAColor{r:255,g:255,b:158,a:255});
                self.colors.push(RGBAColor{r:255,g:255,b:125,a:255});
                self.colors.push(RGBAColor{r:255,g:251,b:93,a:255});
                self.colors.push(RGBAColor{r:255,g:247,b:65,a:255});
                self.colors.push(RGBAColor{r:255,g:247,b:31,a:255});
                self.colors.push(RGBAColor{r:255,g:247,b:0,a:255});

                self.colors.push(RGBAColor{r:231,g:219,b:0,a:255});
                self.colors.push(RGBAColor{r:207,g:199,b:0,a:255});
                self.colors.push(RGBAColor{r:182,g:174,b:0,a:255});
                self.colors.push(RGBAColor{r:158,g:158,b:0,a:255});
                self.colors.push(RGBAColor{r:134,g:134,b:0,a:255});
                self.colors.push(RGBAColor{r:113,g:109,b:0,a:255});
                self.colors.push(RGBAColor{r:89,g:85,b:0,a:255});
                self.colors.push(RGBAColor{r:65,g:65,b:0,a:255});


                self.colors.push(RGBAColor{r:211,g:255,b:93,a:255});
                self.colors.push(RGBAColor{r:199,g:255,b:64,a:255});
                self.colors.push(RGBAColor{r:182,g:255,b:31,a:255});
                self.colors.push(RGBAColor{r:162,g:255,b:0,a:255});
                self.colors.push(RGBAColor{r:146,g:231,b:0,a:255});
                self.colors.push(RGBAColor{r:130,g:207,b:0,a:255});
                self.colors.push(RGBAColor{r:117,g:182,b:0,a:255});
                self.colors.push(RGBAColor{r:97,g:158,b:0,a:255});

                self.colors.push(RGBAColor{r:219,g:255,b:219,a:255});
                self.colors.push(RGBAColor{r:190,g:255,b:186,a:255});
                self.colors.push(RGBAColor{r:158,g:255,b:158,a:255});
                self.colors.push(RGBAColor{r:130,g:255,b:125,a:255});
                self.colors.push(RGBAColor{r:97,g:255,b:93,a:255});
                self.colors.push(RGBAColor{r:64,g:255,b:65,a:255});
                self.colors.push(RGBAColor{r:30,g:255,b:31,a:255});
                self.colors.push(RGBAColor{r:0,g:255,b:0,a:255});

                for c in self.colors.len()..255
                {
                    self.colors.push(RGBAColor{r:127,g:127,b:127,a:127});
                }

                self.colors.push(RGBAColor{r:255,g:255,b:255,a:255});
            }
        }

    }

    #[derive(Debug, Default)]
    pub struct Canvas
    {

    }

    impl Canvas{

        pub fn put_pixel(x:u16, y:u16, col:u8, indexed_pixels: &mut [u8], config: &DPConfig)
        {
            //println!("put pixel: x={}, y={}, cfr={}",x,y, config.selected_resolution.x);
            let byte = (x as u32+(y as u32*config.selected_resolution.x as u32)) as usize;
            indexed_pixels[byte]=col;
        }


        pub fn draw_line(x1:u16, y1:u16, x2:u16, y2:u16, col:u8, indexed_pixels: &mut [u8], config: &DPConfig)
        {
            //println!("draw_line: x1={}, x2={}", x1, x2);

            let dx = ( ( (x2 as i16) - (x1 as i16) ) as i16).abs();
            let mut sx:i16 = 1;
            if x2<x1 {sx = -1;}

            let dy = -(( (y2 as i16) - (y1 as i16) ) as i16).abs();
            let mut sy:i16 = 1;
            if y2<y1 {sy = -1;}

            let mut err = dx+dy;
            let mut e2;

            let mut xx1=x1;
            let mut yy1=y1;

            //println!("before line loop, xx1={}, yy1={}, dx={}, dy={}, sx={}, sy={}", xx1, yy1, dx, dy, sx, sy);
            while true{
                Canvas::put_pixel(xx1, yy1, col, indexed_pixels, config );
                if xx1==x2 && yy1==y2 {break;}
                e2 = 2*err;
                //println!("line loop, e2={}, err={}", e2, err);
                if e2>= dy
                {
                    err += dy; xx1 = ((xx1 as i16) + sx) as u16;
                }
                if e2<= dx
                {
                    err += dx; yy1 = ((yy1 as i16) + sy) as u16;
                }
            }
        }

    }

    #[derive(Debug, Default)]
    pub struct Resolution
    {
        pub x: u32,
        pub y: u32,
        pub n_colors: u32,
        pub desc: String,
    }

    #[derive(Debug, Default)]
    pub struct DPConfig
    {
        pub selected_resolution : Resolution,
        pub canvas: Canvas,
        pub palette: Palette,
        pub fg_index_color: u8,
        pub bg_index_color: u8,
    }

}

use winit::event::{Event, WindowEvent, VirtualKeyCode};
use winit::window::{WindowBuilder,CursorIcon,Window};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event_loop::{ControlFlow, EventLoop};
use pixels::{Error, Pixels, SurfaceTexture};

use winit_input_helper::WinitInputHelper;

use crate::ui_interface::Canvas;

use crate::ui_interface::Dimensions;
use crate::ui_interface::Resolution;
use crate::ui_interface::Palette;
use crate::ui_interface::DPConfig;

use std::io::{self, Write};



fn main() -> Result<(), Error>   {

    //let mut config:DPConfig = DPConfig {selected_resolution:Resolution{x:80,y:50,n_colors:256,desc:"MCGA".to_string()}, ..Default::default() };
    //let mut config:DPConfig = DPConfig {selected_resolution:Resolution{x:160,y:100,n_colors:256,desc:"MCGA".to_string()}, ..Default::default() };

    //let mut config:DPConfig = DPConfig {selected_resolution:Resolution{x:240,y:200,n_colors:256,desc:"MCGA".to_string()}, ..Default::default() };

    let mut config:DPConfig = DPConfig {selected_resolution:Resolution{x:320,y:200,n_colors:256,desc:"MCGA".to_string()}, ..Default::default() };

    // WORKS OK !!!
    //let mut config:DPConfig = DPConfig {selected_resolution:Resolution{x:400,y:300,n_colors:256,desc:"MCGA".to_string()}, ..Default::default() };

    // WORKS KO !!! every 4 pixels, there's a double width pixel.
    //let mut config:DPConfig = DPConfig {selected_resolution:Resolution{x:640,y:480,n_colors:256,desc:"MCGA".to_string()}, ..Default::default() };

    let mut c:Canvas = Canvas { ..Default::default() };

    let mut previous_mouse_x:u16=0;
    let mut previous_mouse_y:u16=0;

    config.palette.build("256");

    //let mut fg_index_color=0x00;
    //let mut bg_index_color=0x0F;

    let mut input = WinitInputHelper::new();

    let event_loop = EventLoop::new();


    let window = WindowBuilder::new()
        .with_title(format!("RUST WGPU PIXELS TEST") )
        .build(&event_loop)
        .unwrap();


    let default_size = LogicalSize::new(config.selected_resolution.x , config.selected_resolution.y);
    window.set_inner_size(default_size);

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

    let mut pixels = Pixels::new( config.selected_resolution.x, config.selected_resolution.y, surface_texture)?;

    // virtual user pixel space where to store 1 byte per pixel (an u8 pixel means 1 rgba palette entry);
    let mut indexed_pixels:Vec<u8> = vec![0xff; (config.selected_resolution.x * config.selected_resolution.y) as usize];

    window.set_cursor_visible(true);
    window.set_cursor_icon(CursorIcon::Hand);


    // fill with checkered pixel pattern 20 x 20 pixels
    let mut x=0;
    let mut y=0;
    while y < 20
    {
        x=0;
        if y%2 == 1
        {
            x = 1;
        }

        while x < 20
        {
            Canvas::put_pixel(x,y,0, &mut indexed_pixels, &config);
            x += 2;
        }
        y += 1;
    }


    // main loop

    event_loop.run(move |event, _, control_flow| {


        let mut needs_redraw = false;

        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => ()

        }

        if let Event::RedrawRequested(_) = event {

            draw(pixels.get_frame(), &indexed_pixels, &config);
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            needs_redraw = false;
        }


        if input.update(event) {
            // Close events
            if input.key_released(VirtualKeyCode::Q) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.mouse_pressed(0)
            {
                let pos = input.mouse();
                println!("pressed: Mouse position is: {:?}", pos);

                match pos{
                    Some(pos) => {

                        let window_size = window.inner_size();

                        previous_mouse_x = ( (pos.0 as f32) * (config.selected_resolution.x as f32 / window_size.width as f32  )) as u16;
                        previous_mouse_y = ( (pos.1 as f32) * (config.selected_resolution.y as f32 / window_size.height as f32 ))as u16;
                        //println!("window.size.width/height={}/{}, corr x/y={} {}", window_size.width, window_size.height, corr_x, corr_y);
                    } ,
                    None => println!("none!"),
                }

            }

            if input.mouse_held(0) {

                let pos = input.mouse();

                //println!("held: Mouse position is: {:?}", pos);

                match pos{
                    Some(pos) => {

                        let window_size = window.inner_size();
                        window.set_title(format!("[{} x {}]", window_size.width, window_size.height).as_str());

                        let corr_x = ( (pos.0 as f32) * (config.selected_resolution.x as f32 / window_size.width as f32  )).round() as u16;
                        let corr_y = ( (pos.1 as f32) * (config.selected_resolution.y as f32 / window_size.height as f32 )).round() as u16;
                        //println!("window.size.width/height={}/{}, corr x/y={} {}", window_size.width, window_size.height, corr_x, corr_y);

                        //Canvas::put_pixel( corr_x, corr_y, config.fg_index_color,&mut indexed_pixels, &config)
                        Canvas::draw_line( corr_x, corr_y, previous_mouse_x, previous_mouse_y, config.fg_index_color,&mut indexed_pixels, &config);
                        needs_redraw=true;
                        previous_mouse_x = corr_x;
                        previous_mouse_y = corr_y;
                    } ,
                    None => println!("none!"),
                }

            }


            if input.window_resized() != None
            {
                let window_size = window.inner_size();
                window.set_title(format!("[{} x {}]", window_size.width, window_size.height).as_str());
            }

            if(needs_redraw)
            {
                window.request_redraw();
            }

            //let s1 = window.inner_size();
            //println!("default size ={},{}", s1.width, s1.height);

        }


    });

    fn draw(frame: &mut [u8], indexed_pixels: &[u8], config: &DPConfig) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % config.selected_resolution.x as usize) as i32;
            let y = (i / config.selected_resolution.x as usize) as i32;

            let index = (x+(y*config.selected_resolution.x as i32)) as usize;

            let pal = &config.palette.colors[ indexed_pixels[ index ] as usize ];
            let rgba = [pal.r, pal.g, pal.b, pal.a ];

            pixel.copy_from_slice(&rgba);
        }
        print!("D"); io::stdout().flush();
    }
}

/*
Hi all, I'm a somewhat newbie in Rust (with a historical background in many other languages around, being C the one I started with decades ago). As of today my main
everyday language is Swift.

The thing is I have some 2D/retro ideas to develop in Rust, and I've come accross pixels (among the rest of the pixel related troupe of crates).
What is the problem I've detected? Somewhat strange: Some pixels are painted "double width/height" when multiples of certain value (different values depending on the
selected window resolution).

I've simplified my (aesthetically) faulty program into an easy git project for someone to test: https://github.com/friguron/report_rust

As of now I've downgraded it to a main loop and a mouse silly painter... The mouse drawer part was optional but I left it there because of the potential it adds for debugging.
I've also rawn this pattern to test it without mouse interference, just cold put pixel:

1010101010101010101
0101010101010101010
1010101010101010101
0101010101010101010
1010101010101010101
0101010101010101010
1010101010101010101
0101010101010101010

What do I tipically get?  Different groups of problems:

a) Resolutions that show faulty pixel accuracy when set up with no need to resize the window to trigger the faulty render (of course, when you resize these windows, you get the faulty effect, with bigger size)
typically, 640 x 480 (and bigger sizes) trigger this mode.
b) Resolutions that show perfect pixel accuracy when set up, but show faulty behaviour when resizing the window. 320 x 200 falls into this cattegory.
c) Resolutions that show perfect pixel accuracy when set up AND when resizing their window (160 x 100, for example). No matter how big you resize the window, you get proportional pixels, unlike in the other cases.

In the initial lines of the main function of my code you can find my resolution setup function...
The one that started to smell fishy (and behave not as intended) was a "simple" 640 x 480 one, where every 4 pixels, (column or row, it doesn't matter),
a "doubled" pixel line is drawn...
If you enlarge a 640 x 480 window you can see the effect with more detail.

If you play with 320 x 200 otoh, you find it works PIXEL PERFECT when the window is not resized, just at boot time, but it's faulty on the X axis if you
enlarge the window (the Y axis shows perfectly uniform pixel heights).

If you play with 160 x 100 (or around these small values), you get pixel perfect resolutions at boot time, AND when you resize the window



So:

1.- Is it me? I can assume I'm newbie not only in Rust but in all the crates involved (Winit, Pixels, wgpu, etc...)
and maybe I've forgotten something in the init part of Pixels for the expected pixel perfect behaviour?
2.- Is it my Mac ? (I can't test in on windows/linux as of now)
3.- Is it Pixels? Can it be some underlying crate instead? (wgpu being the main candidate?)

My main concern, specially if I don't resize my working window, is that some resolutions are shown perfetly, others don't. Why that?














I'm using MacOS Mojave, and I want to create






*/