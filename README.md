# Display-Villagers
Summon all villagers and zombie villagers for testing during resource pack creation

Usage
=====
Installation
------------
1. Download the latest release (or latest for your version/pack format)
2. Add to the save's datapack folder. You don't need to apply it in-game, unlike resource packs.
      • `%appdata%\.minecraft.saves`
3. If in world, run `/reload`.
<!---
'/' is included in the installation section for usability, but intentionally not included elsewhere.
-->

Usage
-----
1. Run `/function display_villagers:run_this`
2. Fly around villagers
3. Run `/kill @e[tag=statue]` to remove the villagers

Design
======
Generating mcfunction
---------------------
• A program is written to generate the mcfunction that has all villagers bar one
• The code is probably really sloppy. It's one of my first projects (jacko-png).
      • For example, it must be run with `Rust/` as the current working directory otherwise the file won't created
            • There is no error checking
      • Also, snake_case in the datapack (not Java semantics).

Datapack implementation
-----------------------
• A villager is summoned by display_villager:run_this as the origin
• This villager runs display_villagers:backend/display
• display_villagers:backend/display summons the rest of the villagers as an offest from the origin
• The origin has its origin tag replaced with a statue tag, so that it can be handled with all the other villagers.
    • For example, `execute as @e[tag=statue] at @s run tp @s ~ 324 ~` (over `execute as @e[tag=statue] at @s run tp @s ~ 324 ~` and `execute as @e[tag=o] at @s run tp @s ~ 324 ~`)
