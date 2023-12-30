use std::io;

// COMMENT TO CHECK FOR GIT PUSH/COMMIT WORKING PLSSSSS

#[derive(Debug)]
struct Player {
    id: String,
    lvl: u32,
    exp: u32,
    raw_stat: PlayerStatRaw,
    stat: PlayerStat,
    weapon_equipped: bool,
    weapon: Weapon,
    default_weapon: Weapon,
}

#[derive(Debug)]
struct PlayerStatRaw {
    hp: i32,
    atk: i32,
    def: i32,
}

#[derive(Debug)]
struct PlayerStat {
    hp: i32,
    atk: i32,
    def: i32,
}

#[derive(Clone)]
#[derive(Debug)]
struct Weapon {
    id: String,
    atk: i32,
    hp: i32,
    def: i32,
}

#[derive(Debug)]
enum GameState {
    Menu,
    Battle,
}

trait Attack {
    fn attack(&self, target: &mut Self);
}

impl Attack for Player {
    fn attack(&self, target: &mut Self) {
        target.stat.hp -= self.stat.atk
    }
}

trait Equip {
    fn equip(&mut self, weapon: &Weapon);
}

impl Equip for Player {
    fn equip(&mut self, weapon: &Weapon) {
        if self.weapon_equipped == false {
            if weapon.id == self.default_weapon.id {} else {println!("Equipping {} with {}", self.id, weapon.id);}
            self.stat.atk += weapon.atk;
            self.stat.hp += weapon.hp;
            self.stat.def += weapon.def;
            self.weapon_equipped = true;
            self.weapon = weapon.clone();
        } else {
            println!("Weapon already equipeed.");
        }
    }
}

trait Unequip {
    fn unequip(&mut self, weapon: &Weapon);
}

impl Unequip for Player {
    fn unequip(&mut self, weapon: &Weapon) {
        if self.weapon_equipped == true {
            println!("Unequipping {} from {}", weapon.id, self.id);
            self.stat.atk -= weapon.atk;
            self.stat.hp -= weapon.hp;
            self.stat.def -= weapon.def;
            self.weapon_equipped = false;
            self.weapon = self.default_weapon.clone();
        } else {
            println!("No weapon equipped.");
        }
    }
}

fn main() {
    println!("Welcome!");
    
    println!("Please enter a username: ");

    let mut player_id = String::new();

    io::stdin()
            .read_line(&mut player_id)
            .expect("Failed to read line.");

        let player_id: &str = player_id.trim();

    let mut player_stat_raw = PlayerStatRaw {
        hp: 10,
        atk: 0,
        def: 0,
    };

    let mut player_stat = PlayerStat {
        hp: player_stat_raw.hp,
        atk: player_stat_raw.atk,
        def: player_stat_raw.def,
    };

    let fists = Weapon {
        id: String::from("fists"),
        atk: 1,
        hp: 0,
        def: 0,
    };

    let mut player = Player {
        id: player_id.to_string(),
        lvl: 1,
        exp: 0,
        raw_stat: player_stat_raw,
        stat: player_stat,
        weapon_equipped: false,
        weapon: fists.clone(),
        default_weapon: fists.clone(),
    };

    let wooden_stick = Weapon {
        id: String::from("wooden_stick"),
        atk: 2,
        hp: 0,
        def: 0,
    };

    let metal_stick = Weapon {
        id: String::from("metal_stick"),
        atk: 8,
        hp: 0,
        def: 0,
    };

    let stone_stick = Weapon {
        id: String::from("stone_stick"),
        atk: 5,
        hp: 0,
        def: 0,
    };

    let mut weapon_inventory: Vec<Weapon> = Vec::new();
    player.equip(&fists);

    weapon_inventory.push(stone_stick);
    weapon_inventory.push(fists);

    
    'menu: loop {
        let mut game_state: GameState = GameState::Menu;
        
        let mut command = String::new();

        println!("Gamestate: {:?}", game_state);

        println!("Please input a command: ");

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line. :(");

        let command = command.to_ascii_uppercase();
        let command: &str = command.trim();

        match command {
            "BATTLE" => {
                game_state = GameState::Battle;
                println!("Gamestate: {:?}", game_state);
                'battle: loop {
                    println!("Please input a command: ");
                    
                    let mut command = String::new();

                    io::stdin()
                        .read_line(&mut command)
                        .expect("Failed to read line. :(");

                    let command = command.to_ascii_uppercase();
                    let command: &str = command.trim();
                    
                    match command {
                        "QUIT" => break 'menu,
                        "RETURN" => break 'battle,
                        "MENU" => break 'battle,
                        "EQUIP" => {
                            let mut type_to_equip = String::new();
                            println!("What would you like to equip?");
                            println!("Valid options are: Weapon");

                            io::stdin()
                                .read_line(&mut type_to_equip)
                                .expect("Failed to read line. :(");
                        
                            let type_to_equip = type_to_equip.to_ascii_lowercase();
                            let type_to_equip: &str = type_to_equip.trim();

                            match type_to_equip {
                                "weapon" => {
                                    if weapon_inventory.len() > 0 {
                                        let mut weapon = String::new();
                                        println!("Please select an available weapon: ");
                                        for item in &weapon_inventory {
                                            println!("{:?}", item)
                                        }
                        
                                        io::stdin()
                                            .read_line(&mut weapon)
                                            .expect("Failed to read line. :(");
                        
                                        let weapon = weapon.to_ascii_lowercase();
                                        let weapon: &str = weapon.trim();
                        
                                        for item in &weapon_inventory {
                                            if weapon == item.id {
                                                let weapon_equipped = player.weapon.clone();
                                                player.unequip(&weapon_equipped);
                                                player.equip(item);
                                            }
                                        }
                                    } else {println!("No weapons in inventory. :(");}
                                },
                                "quit" => break 'menu,
                                _ => println!("Invalid option, your only option is weapon >:D"),
                            };
                        }
                        "UNEQUIP" => {
                            let mut type_to_unequip = String::new();
                            println!("What would you like to unequip?");

                            io::stdin()
                                .read_line(&mut type_to_unequip)
                                .expect("Failed to read line. :(");
                        
                            let type_to_unequip = type_to_unequip.to_ascii_lowercase();
                            let type_to_unequip: &str = type_to_unequip.trim();

                            match type_to_unequip {
                                "weapon" => {
                                    let equipped_weapon: Weapon = player.weapon.clone();
                            player.unequip(&equipped_weapon);
                                },
                                _ => println!("Invalid option, your only option is weapon >:D"),
                            };
                        },
                        "PLAYERSTATS" => println!("ID: {}, Lvl: {}, Exp: {}, HP: {}({} Raw), ATK: {}({} Raw), DEF: {}({} Raw), Weapon Equipped: {}, Weapon: {}", player.id, player.lvl, player.exp, player.stat.hp, player.raw_stat.hp, player.stat.atk, player.raw_stat.atk, player.stat.def, player.raw_stat.def, player.weapon_equipped, player.weapon.id),
                        _ => println!("Valid commands are: Quit, Menu, Return, Equip, Unequip"),
                    }
                }
            }
            "EQUIP" => {
                let mut type_to_equip = String::new();
                println!("What would you like to equip?");
                println!("Valid options are: Weapon");

                io::stdin()
                    .read_line(&mut type_to_equip)
                    .expect("Failed to read line. :(");
            
                let type_to_equip = type_to_equip.to_ascii_lowercase();
                let type_to_equip: &str = type_to_equip.trim();

                match type_to_equip {
                    "weapon" => {
                        if weapon_inventory.len() > 0 {
                            let mut weapon = String::new();
                            println!("Please select an available weapon: ");
                            for item in &weapon_inventory {
                                println!("{:?}", item)
                            }
            
                            io::stdin()
                                .read_line(&mut weapon)
                                .expect("Failed to read line. :(");
            
                            let weapon = weapon.to_ascii_lowercase();
                            let weapon: &str = weapon.trim();
            
                            for item in &weapon_inventory {
                                if weapon == item.id {
                                    let weapon_equipped = player.weapon.clone();
                                    player.unequip(&weapon_equipped);
                                    player.equip(item);
                                }
                            }
                        }
                    }
                    _ => {println!("Valid option not chosen."); println!("Your only option is weapon! >:)");},
                }
            }
            "UNEQUIP" => {
                let mut type_to_unequip = String::new();
                println!("What would you like to unequip?");

                io::stdin()
                    .read_line(&mut type_to_unequip)
                    .expect("Failed to read line. :(");
            
                let type_to_unequip = type_to_unequip.to_ascii_lowercase();
                let type_to_unequip: &str = type_to_unequip.trim();

                match type_to_unequip {
                    "weapon" => {
                        let equipped_weapon: Weapon = player.weapon.clone();
                player.unequip(&equipped_weapon);
                    },
                    _ => println!("Invalid option, your only option is weapon >:D"),
                };
            }
            "PLAYERSTATS" => println!("ID: {}, Lvl: {}, Exp: {}, HP: {}({} Raw), ATK: {}({} Raw), DEF: {}({} Raw), Weapon Equipped: {}, Weapon: {}", player.id, player.lvl, player.exp, player.stat.hp, player.raw_stat.hp, player.stat.atk, player.raw_stat.atk, player.stat.def, player.raw_stat.def, player.weapon_equipped, player.weapon.id),
            "QUIT" => break 'menu,
            "MENU" => println!("You are already in the menu."),
            _ => println!("Valid commands are: Battle, Quit, PlayerStats, Equip, Unequip"),
        }
    }
    println!("Sayonara :(");
}