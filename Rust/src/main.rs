use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut out = File::create("out.txt")?;
    // print_type(&out);
    
    let species: [&str; 2] = ["villager", "zombie_villager"];
    let biomes: [&str; 7] = ["plains", "desert", "savanna", "taiga", "snow", "swamp", "jungle"];
    let professions: [&str; 15] = ["none", "nitwit", "farmer", "shepherd", "fisherman", "fletcher", "butcher", "leatherworker", "mason", "toolsmith", "weaponsmith", "armorer", "cleric", "librarian", "cartographer"];

    let dx = 3;
    let dz = 3;
    let mut x_off = 0;  // generates the first villager, but line can be removed manually
    let mut z_off = 0;

    for species in species {
        // println!("{}", species); // each element of species shadows species
        // println!("{:?}", biomes); // biomes is not out of scope

        for biome in biomes {
            // println!("{}", biome);

            for profession in professions {
                let line = format!(
                    r#"execute at @s run summon {species} ~{x_off} ~ ~{z_off} {{FallDistance:255f,OnGround:0b,NoAI:1b,Silent:1b,Tags:["statue"],VillagerData:{{Offers:{{}},profession:"minecraft:{profession}",type:"minecraft:{biome}"}}}}"#
                );
                write!(out, "{}\n", line)?;
                x_off += dx;
            }
            
            z_off += dz;
            x_off = 0;
        }
    }
    Ok(())
}