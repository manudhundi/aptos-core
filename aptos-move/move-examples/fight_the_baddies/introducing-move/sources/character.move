module fight_the_baddies::character {
use std::string::{Self, String};

friend fight_the_baddies::level_up;
friend fight_the_baddies::fight;

struct Character {
  name: String,
  gender: String,
  affinity: String,
  level: u8,
  health: u32,
  defense: u16,
  strength: u16,
  agility: u16,
  experience: u64,
}

public fun generate_hero(name: String, gender: String): Character {
  Character {
    name,
    gender,
    affinity: string::utf8(b"good"),
    level: 1,
    health: 32,
    defense: 5,
    strength: 3,
    agility: 8,
    experience: 0,
  }
}

public fun generate_baddy(name: String, gender: String): Character {
  Character {
    name,
    gender,
    affinity: string::utf8(b"bad"),
    level: 1,
    health: 8,
    defense: 2,
    strength: 1,
    agility: 3,
    experience: 0,
  }
}

public fun is_hero(character: &Character): bool {
  character.affinity == string::utf8(b"good")
}

public fun is_baddy(character: &Character): bool {
  character.affinity == string::utf8(b"bad")
}

public fun name(character: &Character): String {
  character.name
}

public fun gender(character: &Character): String {
  character.gender
}

public fun level(character: &Character): u8 {
  character.level
}

public fun health(character: &Character): u32 {
  character.health
}

public fun defense(character: &Character): u16 {
  character.defense
}

public fun strength(character: &Character): u16 {
  character.strength
}

public fun agility(character: &Character): u16 {
  character.agility
}

public fun experience(character: &Character): u64 {
  character.experience
}

public(friend) fun set_level(character: &mut Character, level: u8) {
  character.level = level
}

public(friend) fun set_health(character: &mut Character, health: u32) {
  character.health = health
}

public(friend) fun set_defense(character: &mut Character, defense: u16) {
  character.defense = defense
}

public(friend) fun set_strength(character: &mut Character, strength: u16) {
  character.strength = strength
}

public(friend) fun set_agility(character: &mut Character, agility: u16) {
  character.agility = agility
}

public(friend) fun set_experience(character: &mut Character, experience: u64) {
  character.experience = experience
}


public(friend) fun destroy(character: Character) {
  let Character {
    name: _,
    gender: _,
    affinity: _,
    level: _,
    health: _,
    defense: _,
    strength: _,
    agility: _,
    experience: _,
  } = character;
}
}
