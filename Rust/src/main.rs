use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut out = File::create(r"..\display-villager\data\display_villagers\functions\backend\display.mcfunction")?;
    // print_type(&out);
    
    let species: [&str; 2] = ["villager", "zombie_villager"];
    let biomes: [&str; 7] = ["plains", "desert", "savanna", "taiga", "snow", "swamp", "jungle"];
    let professions: [&str; 15] = ["none", "nitwit", "farmer", "shepherd", "fisherman", "fletcher", "butcher", "leatherworker", "mason", "toolsmith", "weaponsmith", "armorer", "cleric", "librarian", "cartographer"];
    
    let mut is_first_villager = true;
    let dx = 3;
    let dz = 3;
    let mut x_off = 0;  // generates the first villager, but line can be removed manually
    let mut z_off = 0;

    for species in species {
        // println!("{}", species); // each element of species shadows species
        // println!("{:?}", biomes); // biomes is not out of scope

        for biome in biomes {

            for profession in professions {
                let line = format!(
                    r#"execute at @s run summon {species} ~{x_off} ~ ~{z_off} {{PersistenceRequired:1b,NoAI:1b,Silent:1b,Tags:["statue"],VillagerData:{{profession:"minecraft:{profession}",type:"minecraft:{biome}"}},ArmorItems:[{{}},{{}},{{}},{{"id":"stone_button","Count":1b}}]}}"#
                );
                if !is_first_villager {
                    write!(out, "{}\n", line)?;
                } else {
                    println!("{}", line);
                    is_first_villager = false;
                }
                x_off += dx;
            }
            
            z_off += dz;
            x_off = 0;
        }
    }
    Ok(())
}