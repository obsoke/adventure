use Room;
use Connection;
use Flags;
use Item;
use Direction;

pub fn create_rooms() -> Vec<Room> {
    /*
    The plan was to serialize these from a file, but is that possible with closures?
    */
    /*
    A room has:
      - A description: What the player will see upon entering a room, or using
        the 'look' command.
      - connections: room indices that this room can connect to, as a hashmap:
          - north: i32,
          - south: i32,
          - east: i32,
          - west: i32,
    */
    let rooms = vec!(
        // ROOM 0 - Starting room
        Room {
            connections: Connection::new(Some(1), None, None, None),
            items: vec![
                Item {
                    name: "cat".to_string(),
                    is_grabbable: true,
                    on_grab: Box::new(|flags: &mut Flags| {
                        println!("The cat purrs as you pick it up and fit it in your pocket.");
                        flags.update_key("pickedUpCat", true);
                    }),
                    on_use: Box::new(|flags: &mut Flags, object_name: String, current_room: i32| -> bool {
                        if current_room == 0 && object_name == "lever" {
                            println!("The cat looks at the lever for a second before it begins to lick its paws.");
                            false
                        }
                        else if current_room == 5 && object_name == "altar" {
                            println!("You place the cat on the altar. It walks around for a second before settling down to lick its paws. You hear a clicking sound behind you. It seems like the altar had a pressure-sensitive plate on it, and that putting the cat on it revealed something else in the room.");
                            flags.update_key("isCatOnAltar", true);
                            true
                        }
                        else {
                            println!("The cat isn't sure what to do with that.");
                            false
                        }
                    }),
                },
                Item {
                    name: "lever".to_string(),
                    is_grabbable: false,
                    on_grab: Box::new(|flags: &mut Flags| {
                        if flags.get_key("initialSwitchPulled") == Some(&false) {
                            println!("You pull with all your might on the rusty lever as it slowly begins to fall. A loud crunching noise is heard from behind the walls as one of them shifts aside to reveal a doorway NORTH.");
                            flags.update_key("initialSwitchPulled", true);
                        }
                        else {
                            println!("No matter how hard you try, the switch won't bduge. It seems to have arrived at it's final resting place.");
                        }
                    }),
                    on_use: Box::new(|flags: &mut Flags, object_name: String, current_room: i32| -> bool {
                        // don't need to implement this for items where is_grabbable == false
                        false
                    }),
                }
            ],
            get_description: Box::new(|flags: &Flags| {
                if flags.get_key("pickedUpCat") == Some(&false) {
                    println!("You find yourself waking up in a small room lit by a single torch. A crooked table is in the corner, slightly rocking back and forth as if it took all of it's own strength to stay upright. On top of the table is a fat CAT, staring intently at you.");
                }
                else {
                    println!("You find yourself waking up in a small room lit by a single torch. A crooked table is in the corner, slightly shaking as if it took all of it's own strength to stay upright. There is a recess in the table where the cat was laying (How long was it laying there for!?)");
                }

                if flags.get_key("initialSwitchPulled") == Some(&false) {
                    println!("A rusted-covered LEVER is sticking out of the wall.");
                }
                else {
                    println!("A passageway has been revealed on the NORTH wall.");
                }
            }),
            can_move: Box::new(|flags: &Flags, direction: &Direction| -> bool {
                match *direction {
                    Direction::North => flags.get_key("initialSwitchPulled") == Some(&true),
                    _ => true,
                }
            }),
        },
        // ROOM 1 - Greenhouse
        Room {
            connections: Connection::new(Some(2), Some(0), None, None),
            items: vec![
                Item {
                    name: "shovel".to_string(),
                    is_grabbable: true,
                    on_grab: Box::new(|flags: &mut Flags| {
                        println!("The shovel looks as if it has never been used before; the layer of dust that falls off as you pick it up shows that it has been sitting on that table for a long time. You slip the shovel in your pocket.");
                        flags.update_key("pickedUpShovel", true);
                    }),
                    on_use: Box::new(|flags: &mut Flags, object_name: String, current_room: i32| -> bool {
                        // this sucks; checking if we are in the room before perfoming action
                        if current_room == 1 && object_name == "glass door" {
                            if flags.get_key("smashedDoor") == Some(&false) {
                                println!("It takes a few swings before a couple of cracks appear in the glass. Wondering why such strong glass is needed for a greenhouse door, you continue to swing away until a loud crash and gust of fresh air announces the success of your swinging endeavours.");
                                flags.update_key("smashedDoor", true);
                                false
                            }
                            else {
                                println!("You seem to have already done a number on that poor door - maybe you should leave it alone?");
                                false
                            }
                        }
                        else {
                            println!("You aren't sure how to use the shovel with the {}", object_name);
                            false
                        }
                    }),
                },
                Item {
                    name: "glass door".to_string(),
                    is_grabbable: false,
                    on_grab: Box::new(|flags: &mut Flags| {
                        println!("You search the door for a handle or crevice but find nothing.  It's perfectly flat with nothing to grab onto.");
                    }),
                    on_use: Box::new(|flags: &mut Flags, object_name: String, current_room: i32| -> bool {
                        // don't need to implement this for items where is_grabbable == false
                        false
                    }),
                }
            ],
            get_description: Box::new(|flags: &Flags| {
                print!("You have arrived in what appears to be a greenhouse, filled with strange, brightly-coloured plants and grasses you've never seen before. The scent of sulphur hangs in the air. ");
                if flags.get_key("smashedDoor") == Some(&false) {
                    println!("On the NORTH end of the greenhouse is a GLASS DOOR, tightly shut.")
                }
                else {
                    println!("On the NORTH end of the greenhouse is an an open door with pieces of glass sprinkling the ground around it.");
                }

                if flags.get_key("pickedUpShovel") == Some(&false) {
                    println!("There is a short yet long table along the side of the greenhouse. Many items are sitting on it, including a SHOVEL.");
                }
                else {
                    println!("There is a short yet long table along the side of the greenhouse. Many items are sitting on it.");
                }
            }),
            can_move: Box::new(|flags: &Flags, direction: &Direction| -> bool {
                match *direction {
                    Direction::North => flags.get_key("smashedDoor") == Some(&true),
                    _ => true,
                }
            }),
        },
        // ROOM 2 - Crossroads
        Room {
            connections: Connection::new(None, Some(1), Some(4), Some(3)),
            items: vec![], // no items in crossroads
            get_description: Box::new(|flags: &Flags| {
                println!("For as far as the eye can see, there is nothing but rolling green hills around. You have reached a sort of crossroads with two paths in front of you. To the EAST is a path leading towards a forest. To the WEST, the path continues along the rolling landscape.");
            }),
            can_move: Box::new(|flags: &Flags, direction: &Direction| -> bool {
                true
            }),
        },
        // ROOM 3 - Westward Well
        Room {
            connections: Connection::new(None, None, Some(2), None),
            items: vec![
                Item {
                    name: "rope".to_string(),
                    is_grabbable: false,
                    on_grab: Box::new(|flags: &mut Flags| {
                        if flags.get_key("isBuckedPulledUp") == Some(&false) {
                            println!("You slowly pull up on the rope. Peering down the well, you see a bucket tied to the end. After a minute, you pull the bucket out of the well and set it on the stone wall.");
                            flags.update_key("isBuckedPulledUp", true);
                        }
                        else if flags.get_key("bucketOnFloor") == Some(&false) {
                            println!("You pull on the limp rope, and the bucket falls on the grass. Great job!");
                            flags.update_key("bucketOnFloor", true);
                        }
                        else {
                            println!("You pull on the limp rope. Nothing happens. You make yourself a little sad.");
                        }
                    }),
                    on_use: Box::new(|flags: &mut Flags, object_name: String, current_room: i32| -> bool {
                        false
                    }),
                },
                Item {
                    name: "key".to_string(),
                    is_grabbable: true,
                    on_grab: Box::new(|flags: &mut Flags| {
                        if flags.get_key("isBuckedPulledUp") == Some(&false) {
                            println!("You don't see a key.");
                        }
                        else {
                            println!("You pick up the key and examine it for a second. The key is small and silver with not a single scratch on it. It looks like the sort of key used for a child's diary. You slip it into your pocket.");
                            flags.update_key("pickedUpKey", true);
                        }
                    }),
                    on_use: Box::new(|flags: &mut Flags, object_name: String, current_room: i32| -> bool {
                        // this sucks; checking if we are in the room before perfoming action
                        if current_room == 4 && object_name == "door" {
                                println!("You insert the tiny silver key into the shack door and turn...");
                                println!("It worked! The door is unlocked.");
                                flags.update_key("shackDoorUnlocked", true);
                                true
                        }
                        else {
                            println!("You aren't sure how to use the key with {}.", object_name);
                            false
                        }
                    }),
                },
            ],
            get_description: Box::new(|flags: &Flags| {
                print!("The winding path seems to stop in front of a lone, stone well. Half of the well's wall seems to be falling outward onto the grass surrounding it. ");
                if flags.get_key("isBuckedPulledUp") == Some(&false) {
                    println!("A single ROPE hangs from the top of the well.")
                }
                else if flags.get_key("pickedUpKey") == Some(&false) {
                    println!("A bucket sits on the edge of the well. Laying on the bottom of the bucket is a KEY.");
                }
                else {
                    println!("An empty bucket sits on the edge of the well.");
                }
            }),
            can_move: Box::new(|flags: &Flags, direction: &Direction| -> bool {
                true
            }),
        },
        // ROOM 4 - Weird Shack, ext.
        Room {
            connections: Connection::new(None, None, Some(5), Some(2)),
            items: vec![
                Item {
                    name: "door".to_string(),
                    is_grabbable: false,
                    on_grab: Box::new(|flags: &mut Flags| {
                        if flags.get_key("shackDoorUnlocked") == Some(&false) {
                            println!("You attempt to open the door, but it seems to be locked.");
                        }
                        else if flags.get_key("shackDoorOpen") == Some(&false) {
                            println!("The door to the shack opens.");
                            flags.update_key("shackDoorOpen", true);
                        }
                        else {
                            println!("'Hey, whattya want from me!?' someone yells; it seems to come from the door itself.");
                        }
                    }),
                    on_use: Box::new(|flags: &mut Flags, object_name: String, current_room: i32| -> bool {
                        false
                    }),
                },
            ],
            get_description: Box::new(|flags: &Flags| {
                println!("You have arrived at a tiny building that you can only describe as 'weird'. At first glance, it looks like a wooden garden shed. After staring at it for a second, it seemed as if one side of the shed was slowly growing and shrinking by a few inches. The other side of the shed looked as if it was shivering.");
                if flags.get_key("shackDoorUnlocked") == Some(&false) {
                    println!("The shack has a shut door with a very tiny lock on it.");
                }
                else if flags.get_key("shackDoorOpen") == Some(&true) {
                    println!("To the EAST, the door to the shack is wide open.");
                }
            }),
            can_move: Box::new(|flags: &Flags, direction: &Direction| -> bool {
                match *direction {
                    Direction::East => flags.get_key("shackDoorUnlocked") == Some(&true),
                    _ => true,
                }
            }),
        },
        // ROOM 5 - Weird Shack, int.
        Room {
            connections: Connection::new(None, None, None, Some(4)),
            items: vec![
                Item {
                    name: "altar".to_string(),
                    is_grabbable: false,
                    on_grab: Box::new(|flags: &mut Flags| {
                        println!("The altar won't budge. Something seems to be holding it in place from below.");
                    }),
                    on_use: Box::new(|flags: &mut Flags, object_name: String, current_room: i32| -> bool {
                        false
                    }),
                },
                Item {
                    name: "head".to_string(),
                    is_grabbable: false,
                    on_grab: Box::new(|flags: &mut Flags| {
                        use std::io;
                        println!("You pulled at the device on your head with all your might and it pops off...");
                        println!("Suddenly, the world around you changes. You are no longer in a strange small shack in the middle of a field. You are in a small apartment in the middle of a city. It seems like this whole experience was a virtual reality game that you may have gotten a little to immersed in.");
                        println!("With this realization, you become depressed, eat a bunch of Halloween candy and go to sleep.");
                        println!("THE END!");
                        println!("Press a key to exit...");

                        let mut value = String::new();

                        // read stdin up until \n into value, show error if something goes wrong
                        io::stdin().read_line(&mut value)
                            .expect("Failed to read line!");

                        flags.update_key("isGameRunning", false);
                    }),
                    on_use: Box::new(|flags: &mut Flags, object_name: String, current_room: i32| -> bool {
                        println!("You've made it this far, clearly you've already been using your head. Keep {} away from it!", object_name);
                        false
                    }),
                },
            ],
            get_description: Box::new(|flags: &Flags| {
                println!("You are now inside the strange shack. Inside, there is nothing but a thin ALTAR in the centre of the back wall.");
                if flags.get_key("isCatOnAltar") == Some(&true) {
                    println!("On one of the walls is a mirror. Looking inside of the mirror, you see that there is some strange device on your HEAD.");
                }
            }),
            can_move: Box::new(|flags: &Flags, direction: &Direction| -> bool {
                true
            }),
        },
    );

    rooms
}
