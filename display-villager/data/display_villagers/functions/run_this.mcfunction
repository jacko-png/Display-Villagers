execute at @s run summon villager ~ ~ ~ {NoAI:1b,Silent:1b,Invulnerable:1b,Tags:["o"],VillagerData:{Offers:{},profession:"minecraft:none",type:"minecraft:plains"}}
execute at @e[type=villager,tag=o] run function display_villagers:backend/display
tag @e[type=villager,tag=o] add statue
tag @e[type=villager,tag=o,tag=statue] remove o