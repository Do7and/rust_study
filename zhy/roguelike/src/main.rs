use tcod::colors::*;
use tcod::console::*;
use tcod::map::{FovAlgorithm, Map as FovMap};
use std::cmp;
use rand::Rng;

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS:bool = true;
const TORCH_RADIUS :i32 = 10;

// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

// size of the map
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;
const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_LIGHT_WALL: Color = Color {
    r: 130,
    g: 110,
    b: 50,
};
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};
const COLOR_LIGHT_GROUND: Color = Color {
    r: 200,
    g: 180,
    b: 50,
};

//parameters for dungeon generator
const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;


const LIMIT_FPS: i32 = 60; // 20 frames-per-second maximum


struct Tcod {
    root: Root,
    con: Offscreen,
    fov:FovMap,
}

#[derive(Debug)]
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

    pub fn move_by(&mut self, dx:i32,dy:i32,game:&Game){
        if !game.map[(self.x+dx) as usize][(self.y + dy) as usize].blocked{
            self.x += dx;
            self.y += dy;
        }//不够合理，如果出现多步移动会导致穿墙目前是movebyone所以暂时没错
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

/// A tile of the map and its properties
#[derive(Clone, Copy, Debug)] //自动实现trait
struct Tile {
    blocked:bool,
    block_sight:bool,
}

impl Tile{
    pub fn empty() ->Self{
        Tile { blocked: false, block_sight: false, }
    }

    pub fn wall() ->Self{
        Tile { blocked: true, block_sight: true, }
    }
}

type Map = Vec<Vec<Tile>>;

struct Game{
    map:Map,
}

/// A rectangle on the map, used to characterise a room.
#[derive(Clone, Copy, Debug)]
struct Rect{
    x1:i32,
    y1:i32,
    x2:i32,
    y2:i32,
}
impl Rect{
    pub fn new(x:i32,y:i32,w:i32,h:i32) ->Self{
        Rect { x1: x, y1: y, x2: x+w, y2: y+h }
    }

    pub fn center(&self) -> (i32,i32){
        let center_x = (self.x1 + self.x2)/2;
        let center_y = (self.y1 + self.y2)/2;
        (center_x,center_y)
    }
    pub fn intersects_with(&self,other:&Rect) ->bool{
        (self.x1 <= other.x2)
        && (self.x2 >= other.x1)
        && (self.y1 <= other.y2)
        && (self.y2 >= other.y1)
    }
}
fn create_room(room:Rect,map:&mut Map){
    for x in (room.x1 + 1)..room.x2{
        for y in (room.y1 + 1)..room.y2{
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

fn make_map(player:&mut Object) -> Map {
    // fill map with "unblocked" tiles
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    
    //1 place two pillars to test the map
    // map[30][22] = Tile::wall();
    // map[50][22] = Tile::wall();

    //2 create two rooms
    // let room1 =Rect::new(20, 15, 10, 15);
    // let room2 =Rect::new(50, 15, 10, 15);
    // create_room(room1, &mut map);
    // create_room(room2, &mut map);
    // create_h_tunnel(25, 55, 23, &mut map);

    //3 RANDOM!!!
    let mut rooms = vec![];
    for _ in 0..MAX_ROOMS{
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        // random position without going out of the boundaries of the map
        let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
        let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);
        
        let new_room = Rect::new(x, y, w, h);

        // run through the other rooms and see if they intersect with this one
        let failed = rooms
            .iter()
            .any(|other_room| new_room.intersects_with(other_room));

        if !failed {
            // this means there are no intersections, so this room is valid

            // "paint" it to the map's tiles
            create_room(new_room, &mut map);

            // center coordinates of the new room, will be useful later
            let (new_x, new_y) = new_room.center();

            if rooms.is_empty() {
                // this is the first room, where the player starts at
                player.x = new_x;
                player.y = new_y;
            }else{
                // if not first one, it should connected to last one.
                // memorize the last coordinates of the previous room 
                let (prev_x,prev_y) = rooms[rooms.len()-1].center();
                // toss a coin (random bool value -- either true or false)
                if rand::random() {
                    // first move horizontally, then vertically
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                    create_v_tunnel(prev_y, new_y, new_x, &mut map);
                } else {
                    // first move vertically, then horizontally
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                    create_h_tunnel(prev_x, new_x, new_y, &mut map);
                }
            }
            rooms.push(new_room);
        }
        // finally, append the new room to the list
        
    }
    map
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    // horizontal tunnel. `min()` and `max()` are used in case `x1 > x2`
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    // vertical tunnel
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn render_all(tcod: &mut Tcod, game: &Game, objects: &[Object],fov_recompute :bool) {
    

    if fov_recompute {
        // recompute FOV if needed (the player moved or something)
        let player = &objects[0];
        tcod.fov
            .compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
    }
    // go through all tiles, and set their background color
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let visible = tcod.fov.is_in_fov(x, y);
            let wall = game.map[x as usize][y as usize].block_sight;
            let color = match (visible, wall) {
                // outside of field of view:
                (false, true) => COLOR_DARK_WALL,
                (false, false) => COLOR_DARK_GROUND,
                // inside fov:
                (true, true) => COLOR_LIGHT_WALL,
                (true, false) => COLOR_LIGHT_GROUND,
            };
            tcod.con
                .set_char_background(x, y, color, BackgroundFlag::Set);
        }
    }

    // draw all objects in the list and in fov
    for object in objects {
        if fov_recompute {
            if tcod.fov.is_in_fov(object.x, object.y) {
                object.draw(&mut tcod.con);
            }
        }else{
            object.draw(&mut tcod.con);
        }
        
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
}

fn handle_keys(tcod: &mut Tcod,game:&Game, player:&mut Object) -> bool {
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
        Key { code: Up, .. } => player.move_by(0, -1,game),
        Key { code: Down, .. } => player.move_by(0, 1,game),
        Key { code: Left, .. } => player.move_by(-1, 0,game),
        Key { code: Right, .. } => player.move_by(1, 0,game),

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
    let fov = FovMap::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut tcod = Tcod { root, con,fov};
    
    

    tcod::system::set_fps(LIMIT_FPS);

    let player = Object::new(SCREEN_WIDTH / 2,SCREEN_HEIGHT / 2,'@',WHITE);
    let npc = Object::new(SCREEN_WIDTH / 2 - 5,SCREEN_HEIGHT / 2,'@',WHITE);
    let mut objects = [player,npc];
    // force FOV "recompute" first time through the game loop
    let mut previous_player_position = (-1, -1);

    let game = Game {
        // generate map (at this point it's not drawn to the screen)
        map: make_map(&mut objects[0]),
    };

    // populate the FOV map, according to the generated map
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            tcod.fov.set(
                x,
                y,
                !game.map[x as usize][y as usize].block_sight,
                !game.map[x as usize][y as usize].blocked,
            );
        }
    }
    
    while !tcod.root.window_closed() {
        
        tcod.con.clear();
        
        for object in &objects {
            object.draw(&mut tcod.con);
        }

        // render the screen
        let fov_recompute = previous_player_position != (objects[0].x, objects[0].y);
        render_all(&mut tcod, &game, &objects, fov_recompute);
        tcod.root.flush();
        
        // handle keys and exit game if needed
        let player = &mut objects[0];
        previous_player_position = (player.x, player.y);
        let exit = handle_keys(&mut tcod,&game, player);
        if exit {
            break;
        }
    }


}
