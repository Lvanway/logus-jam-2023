# logus-jam-2023
A private weekend jam for learning Rust and the Bevy engine!

Theme: _**YOU ARE WHAT YOU EAT**_

# Ideas

Space Invaders theme

- Persistent powerups - edible, with negative consequences?
- Slay the spire type map
- Choice of when to drop power up... slower with more power ups so you're forced to choose wisely
- Eating more power ups mutates you and you lose control in different ways more often, e.g. 3...2...1... thrusters fire 
- Bosses with benefits and negatives that accumulate 
- classes of enemies, some disrupt your abilities or are more susceptible to certain abilities

## Win conditions:
- **Speed of clear**
- Survival
- High score 
- Set number of bosses

## Lose condition:
- don't clear the wave because you lose health and die

## Controls:
- full controls up, down, left, right

## Types of enemies:
- basic ship tries to ram into you and damages your health
- shooter
- missile or an **asteroid** that moves horizontally or vertically quickly across the screen with indicator they're coming... doesn't matter if you don't kill them
- asteroid rush like you're going through a belt of them
- immobile charge up guy who blocks off a section of the screen. gotta kill it before it fires
- warden type that buffs other units
- beefy tank guys moving down the screen slowly

# Steps:
1. moveable character 
2. base type enemy, maybe ram guy
3. health system
4. timer 
5. one power up that you eat!

start game by eating a ram guy... clear all the ram guys. start next room with the power of the last guy you ate