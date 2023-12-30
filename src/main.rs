use std::io;
use std::process;

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

struct Armor {
    id: String,
    atk: i32,
    hp: i32,
    def: i32,
}

struct Inventory {
    player_id: String,
    weapons: Vec<Weapon>,
    armor: Vec<Armor>,
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

fn read_user_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line. :(");
    input.trim().to_ascii_lowercase()
}

fn equip(inventory: &Inventory, player: &mut Player) {
    let prompt = read_user_input("What would you like to equip?");

    match prompt.as_str() {
        "weapon" => equip_weapon(&inventory.weapons, player),
        "quit" => std::process::exit(1),
        _ => println!("Valid option not selected, your only option is weapon >:D"),
    }
}

fn unequip(player: &mut Player) {
    loop {
        let mut type_to_equip = String::new();
        println!("What would you like to unequip?");
        println!("Valid options are: Weapon");

        io::stdin()
            .read_line(&mut type_to_equip)
            .expect("Failed to read line. :(");

        let type_to_equip = type_to_equip.to_ascii_lowercase();
        let type_to_equip: &str = type_to_equip.trim();

        match type_to_equip {
            "weapon" => {let equipped_weapon: Weapon = player.weapon.clone(); player.unequip(&equipped_weapon)},
            "quit" => std::process::exit(1),
            _ => println!("Valid option not selected, your only option is weapon >:D"),
        }
    }
}

fn equip_weapon(weapon_inventory: &Vec<Weapon>, player: &mut Player) {
    if weapon_inventory.len() > 0 {
        let mut weapon = String::new();
        println!("Please select an available weapon: ");
        for item in weapon_inventory {
            println!("{:?}", item)
        }

        io::stdin()
            .read_line(&mut weapon)
            .expect("Failed to read line. :(");

        let weapon = weapon.to_ascii_lowercase();
        let weapon: &str = weapon.trim();

        for item in weapon_inventory {
            if weapon == item.id {
                let weapon_equipped = player.weapon.clone();
                player.unequip(&weapon_equipped);
                player.equip(item);
            }
        }
    } else {println!("No weapons in inventory. :(");}
}

fn main() {
    println!("Welcome!");
    
    run_program();

    println!("Sayonara :(");
}

fn init_weapons() {
    
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

}

fn get_player_stats(player: &Player) {
    println!("ID: {}, Lvl: {}, Exp: {}, HP: {}({} Raw), ATK: {}({} Raw), DEF: {}({} Raw), Weapon Equipped: {}, Weapon: {}", &player.id, &player.lvl, &player.exp, player.stat.hp, &player.raw_stat.hp, &player.stat.atk, &player.raw_stat.atk, &player.stat.def, &player.raw_stat.def, &player.weapon_equipped, &player.weapon.id);
}

fn run_program() {
    // Init player for now till i figure out how to do this shit :/

    let player_id = read_user_input("Please enter a username: ");

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

    let mut weapon_inventory = Vec::new();
    let mut armor_inventory = Vec::new();

    player.equip(&fists);
    weapon_inventory.push(fists);

    let mut player_inventory = Inventory {
        player_id: player_id.to_string(),
        weapons: weapon_inventory,
        armor: armor_inventory,
    };

    init_weapons();

    'menu: loop {
        let mut game_state: GameState = GameState::Menu;

        println!("Gamestate: {:?}", game_state);

        let command = read_user_input("Please input a command: ");

        match command.as_str() {
            "battle" => {
                game_state = GameState::Battle;
                println!("Gamestate: {:?}", game_state);
                'battle: loop {
                    println!("Please input a command: ");
                    
                    let mut command = read_user_input("Please input a command: ");
                    
                    match command.as_str() {
                        "quit" => break 'menu,
                        "return" => break 'battle,
                        "menu" => break 'battle,
                        "equip" => {equip(&player_inventory, &mut player); println!("Function was called :)");},
                        "unequip" => {unequip(&mut player); println!("Function was called :D");},
                        "playerstats" => get_player_stats(&player),
                        _ => println!("Valid commands are: Quit, Menu, Return, Equip, Unequip"),
                    }
                }
            }
            "equip" => {equip(&player_inventory, &mut player); println!("Function was called :)");},
            "unequip" => {unequip(&mut player); println!("Function was called :D");},
            "playerstats" => get_player_stats(&player),
            "menu" => println!("You are already in the menu."),
            "quit" => break 'menu,
            _ => println!("Valid commands are: Battle, Quit, PlayerStats, Equip, Unequip"),
        }
    }
}