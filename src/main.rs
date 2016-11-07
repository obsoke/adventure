/*
Problems encountered:

- Originally, I tried to handle this in a more OO fashion. I had 'Room' structs
  that would contain 'Connection' objects that contained north:
  Option<Box<Room>>, etc... The main 'current_room' variable binding was
  pointing to an element of a Vec<Box<Room>>. Howeveer, I had issues with
  structs that could contain fields that point to itself. I also had trouble
  pulling objects to mutate out of a vector.
*/
mod levels;
use std::collections::HashMap;

enum Command {
    Walk(Direction),
    Grab(String),
    Use(String, String),
    Look,
    Inventory,
    Quit,
    Help,
    Invalid
}

enum Direction {
    North,
    South,
    East,
    West,
}

// source: http://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = HashMap::new();
            $(
                m.insert($key, $value);
            )+
                m
        }
    };
);

struct Flags {
    flag_map: HashMap<&'static str, bool>,
}

impl Flags {
    pub fn new(flag_map: HashMap<&'static str, bool>) -> Self {
        Flags { flag_map: flag_map }
    }

    pub fn get_key(&self, key_name: &'static str) -> Option<&bool> {
        self.flag_map.get(key_name)
    }

    pub fn update_key(&mut self, key_name: &'static str, new_bool: bool) {
        if let Some(mut_key) = self.flag_map.get_mut(&key_name) {
            *mut_key = new_bool;
        }
    }
}

// either the index to the next room or nothing
struct Connection {
    north: Option<usize>,
    south: Option<usize>,
    east: Option<usize>,
    west: Option<usize>,
}

// use trait objects instead of generic trait bounds for this
struct Item {
    name: String,
    is_grabbable: bool,
    on_grab: Box<Fn(&mut Flags)>,
    on_use: Box<Fn(&mut Flags, String, usize) -> bool>,
}

pub struct Room {
    connections: Connection,
    items: Vec<Item>,
    get_description: Box<Fn(&Flags)>,
    can_move: Box<Fn(&Flags, &Direction) -> bool>,
}

impl Connection {
    pub fn new(north: Option<usize>, south: Option<usize>, east: Option<usize>, west: Option<usize>) -> Connection {
        Connection { north: north, south: south, east: east, west: west }
    }
}

struct Game {
    rooms: Vec<Room>,
    current_room: usize,
    inventory: Vec<Item>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            rooms: levels::create_rooms(),
            current_room: 0,
            inventory: Vec::new(),
        }
    }

    pub fn get_command(&self) -> Command {
        use std::io;
        use std::io::Write; // required trait for stdout().flush()

        println!("\nEnter a command (? for help):");
        print!("> ");
        io::stdout().flush().unwrap(); // needed to ensure results of print!() are shown on stdout

        let mut value = String::new();

        // read stdin up until \n into value, show error if something goes wrong
        io::stdin().read_line(&mut value)
            .expect("Failed to read line!");

        // variable binding shadowing & convert to lowervase
        let value = value.to_lowercase();

        // split input into vector of string slices
        let container: Vec<&str> = value.trim().split(' ').collect();

        // match on first command
        match container[0] {
            "g"|"go" => {
                // syntax: go DIRECTION.
                // if container.len() < 2, only one word was entered.
                if container.len() < 2 {
                    Command::Invalid
                }
                else {
                    match container[1] {
                        "n"|"north" => Command::Walk(Direction::North),
                        "s"|"south" => Command::Walk(Direction::South),
                        "e"|"east" => Command::Walk(Direction::East),
                        "w"|"west" => Command::Walk(Direction::West),
                        _ => Command::Invalid,
                    }
                }
            },
            "gr"|"grab" => {
                // syntax: grab ITEM_NAME.
                // if container.len() < 2, only one word was entered.
                if container.len() < 2 {
                    Command::Invalid
                }
                else {
                    let item_name = container[1 .. container.len()].join(" ").clone();
                    Command::Grab(item_name)
                }
            },
            "u"|"use" => {
                // syntax: use ITEM_NAME on OBJECT
                if container.len() < 4 {
                    Command::Invalid
                }
                else {
                    // find "on"
                    let mut for_index: usize = 0;
                    for (i, item) in container.iter().enumerate().skip(2) {
                        if *item == "on" {
                            for_index = i;
                        }
                    }
                    if for_index == 0 || for_index < 2 { return Command::Invalid }

                    let item_name = container[1 .. for_index].join(" ").clone();
                    let object_name = container[for_index + 1 .. container.len()].join(" ").clone();

                    Command::Use(item_name, object_name)
                }
            }
            "i"|"inventory" => Command::Inventory,
            "l"|"look" => Command::Look,
            "quit" => Command::Quit,
            "?" => Command::Help,
            _ => Command::Invalid
        }
    }

    pub fn process_command(&mut self, command: Command, mut global_flags: &mut Flags) {
        match command {
            Command::Walk(direction) => {
                match direction {
                    Direction::North => self.current_room = self.change_room(self.rooms[self.current_room].connections.north, global_flags, &Direction::North),
                    Direction::South => self.current_room = self.change_room(self.rooms[self.current_room].connections.south, global_flags, &Direction::South),
                    Direction::East => self.current_room = self.change_room(self.rooms[self.current_room].connections.east, global_flags, &Direction::East),
                    Direction::West => self.current_room = self.change_room(self.rooms[self.current_room].connections.west, global_flags, &Direction::West),
                }
            },
            Command::Grab(item_name) => self.pick_up_item(&item_name, &mut global_flags),
            Command::Use(item_name, object_name) => self.use_item(&mut global_flags, &item_name, &object_name),
            Command::Look => self.look(global_flags),
            Command::Inventory => self.list_inventory_contents(),
            Command::Help => self.print_help_text(),
            Command::Quit => global_flags.update_key("isGameRunning", false),
            Command::Invalid => println!("Invalid command!"),
        }
    }

    fn change_room(&self, next_room: Option<usize>, global_flags: &Flags, direction: &Direction) -> usize {
        if !(self.rooms[self.current_room].can_move)(global_flags, direction) {
            println!("It seems to be a dead end.");
            return self.current_room;
        }
        match next_room {
            Some(room_id) => {
                (self.rooms[room_id].get_description)(global_flags);
                room_id
            },
            None => {
                println!("It seems to be a dead end.");
                self.current_room
            },
        }
    }

    fn pick_up_item(&mut self, item_name: &str, mut global_flags: &mut Flags) {
        let mut found = false;
        // how to clean this up? constantly referencing things to avoid the
        // borrow checker is not fun...
        for i in 0 .. self.rooms[self.current_room].items.len() { // for every item in the room
            // if item is the one we're looking for...
            if self.rooms[self.current_room].items[i].name == item_name.to_lowercase() {
                // marked item as found...
                found = true;

                (self.rooms[self.current_room].items[i].on_grab)(&mut global_flags); // print on_grab message
                // if item is grabbable, remove from room & add to inventory
                if self.rooms[self.current_room].items[i].is_grabbable {
                    self.inventory.push(self.rooms[self.current_room].items.remove(i));
                }
                // item's been found; no reason to continue to iterate through room's items
                break;
            }
        }

        if !found {
            println!("You found nothing.");
        }
    }

    fn use_item(&mut self, mut global_flags: &mut Flags, item_name: &str, object_name: &str) {
        // 1) ensure item exists / is found
        // we need the index of the item (if we have it in our inventory, that is)
        let index = &self.inventory.iter().position(|i| { i.name.to_lowercase() == item_name.to_lowercase() });
        // make sure we have a value from the resulting Option<usize> and assign it to item
        match *index {
            Some(x) => {
                // remove item from inventory & assign it to binding 'item'
                if self.inventory[x].is_grabbable {
                    if (self.inventory[x].on_use)(&mut global_flags, object_name.to_owned(), self.current_room) {
                        self.inventory.remove(x);
                    }
                }
            },
            None => {
                // item wasn't found; give 'not found' msg and return
                println!("You don't possess a {}.", item_name);
                return;
            },
        };
    }

    fn look(&self, global_flags: &Flags) {
        (self.rooms[self.current_room].get_description)(global_flags);
    }

    fn list_inventory_contents(&self) {
        print!("Peeking inside your bag, you see: ");
        if self.inventory.is_empty() {
            print!("an empty void...");
        }
        else {
            for item in &self.inventory {
                print!("{} ", item.name);
            }
        }
        println!("");
    }

    fn print_help_text(&self) {
        println!("\nAVAILABLE COMMANDS:");
        println!("===================");
        println!("ACTIONS: [l]ook, [gr]ab <item_name>, [u]se <item_name> on <object>, [i]nventory");
        println!("MOVEMENT: [g]o [n]orth|[s]outh|[e]ast|[w]est");
        println!("SYSTEM: quit\n");
    }
}

fn main() {
    use std::io;

    // send a control character to clear terminal screen
    // source: http://bit.ly/2cFyCVY (SO link shortened)
    print!("{}[2J", 27 as char);

    // create global flags
    let global_flag_values = map!{
        // global flags
        "isGameRunning" => true,
        // ROOM 0 FLAGS
        "pickedUpCat" => false, // the cat in room 0
        "initialSwitchPulled" => false, // switch in room 0 opening way to room 1

        // ROOM 1 FLAGS
        "pickedUpShovel" => false, // shovel in greenhouse
        "smashedDoor" => false, // glass door smashed open or not?

        // ROOM 3 FLAGS
        "isBuckedPulledUp" => false, // is the well bucket pulled up?
        "pickedUpKey" => false, // did player get key from bucket?
        "bucketOnFloor" => false, // did the player pull on the rope AGAIN!?

        // ROOM 4 FLAGS
        "shackDoorUnlocked" => false, // weird shack door unlocked with key?
        "shackDoorOpen" => false, // weird shack door open?

        // ROOM 5 FLAGS
        "isCatOnAltar" => false, // has the room 0 cat been placed on altar?
        "didUseMirror" => false // has the mirror been used to reveal the headset?
    };
    let mut global_flags = Flags::new(global_flag_values);

    // create game
    let mut game = Game::new();

    // print title screen
    println!("");
    println!(" ____  ____  _     _____ _      _____ _     ____  _____ _ ");
    println!(r"/  _ \/  _ \/ \ |\/  __// \  /|/__ __Y \ /\/  __\/  __// \\");
    println!(r"| / \|| | \|| | //|  \  | |\ ||  / \ | | |||  \/||  \  | |");
    println!(r"| |-||| |_/|| \// |  /_ | | \||  | | | \_/||    /|  /_ \_/");
    println!(r"\_/ \|\____/\__/  \____\\_/  \|  \_/ \____/\_/\_\\____\(_)");
    println!("");
    println!("A (very) short text adventure by obsoke.");

    println!("Press a key to begin.");

    let mut value = String::new();

    // read stdin up until \n into value, show error if something goes wrong
    io::stdin().read_line(&mut value)
        .expect("Failed to read line!");

    game.process_command(Command::Look, &mut global_flags); // print initial room description

    // main game loop
    while global_flags.get_key("isGameRunning") == Some(&true) {
        let command = game.get_command();
        game.process_command(command, &mut global_flags);
    }
}
