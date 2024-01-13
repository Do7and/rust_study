use tcod::colors::*;
use tcod::console::*;

// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20; // 20 frames-per-second maximum


struct Tcod {
    root: Root,
    con: Offscreen,
}

struct Object{
    x:i32,
    y:i32,
    char:char,
    color:Color,
}
impl Object {
    pub fn new(x:i32,y:i32,char:char,color:Color) -> Self{
        Object{
            x,y,char,color,
        }
    }

    pub fn move_by(&mut self, dx:i32,dy:i32){
        self.x += dx;
        self.y += dy;
    }

    pub fn draw(&self,con:&mut dyn Console){
        /*
        The dyn keyword in &mut dyn Console highlights that Console is a trait and not a concrete type (such as a struct or enum). 
        Earlier versions of Rust allowed to say &Type regardless of whether Type was trait or not. And indeed at least in Rust 1.37 2018 this is still allowed, but it will emit a warning. 
        This has real-life implications as Rust’s pointers to traits are twice the size of pointers to structs. This way anyone reading the code (us, our collaborators, people taking over) can always tell whether a pointer signature (reference, box, etc.) is a trait object or a normal pointer.
        */
        //这里的dyn关键字是表明Console是一个trait 凡是实现了这个trait的都可以
        //TODO dyn到底是啥怎么用的
        con.set_default_foreground(self.color);
        con.put_char(self.x,self.y,self.char,BackgroundFlag::None);
    }
}

fn handle_keys(tcod: &mut Tcod, player:&mut Object) -> bool {
    // todo: handle keys
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    let key = tcod.root.wait_for_keypress(true);
    //println!("{:?}",key);
    match key {
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true, // exit game

        // movement keys
        Key { code: Up, .. } => player.move_by(0, -1),
        Key { code: Down, .. } => player.move_by(0, 1),
        Key { code: Left, .. } => player.move_by(-1, 0),
        Key { code: Right, .. } => player.move_by(1, 0),

        _ => {}
    }
    false
}
fn main() {
    let root = Root::initializer()
    .font("arial10x10.png", FontLayout::Tcod)
    .font_type(FontType::Greyscale)
    .size(SCREEN_WIDTH, SCREEN_HEIGHT)
    .title("Rust/libtcod tutorial")
    .init();

    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut tcod = Tcod { root, con };

    tcod::system::set_fps(LIMIT_FPS);

    let player = Object::new(SCREEN_WIDTH / 2,SCREEN_HEIGHT / 2,'@',WHITE);
    let npc = Object::new(SCREEN_WIDTH / 2 - 5,SCREEN_HEIGHT / 2,'@',WHITE);
    let mut objects = [player,npc];

    
    while !tcod.root.window_closed() {
        
        tcod.con.clear();
        
        for object in &objects {
            object.draw(&mut tcod.con);
        }
        blit(
            &tcod.con,
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut tcod.root,
            (0, 0),
            1.0,
            1.0,
        );
        tcod.root.flush();
        
        // handle keys and exit game if needed
        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, player);
        if exit {
            break;
        }
    }


}
