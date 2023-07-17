---
title: "Move Objects"
---

# Move Objects

The previous section introduced global storage and two mechanisms for placing structs into storage: `key` and `store`. While both approaches work and are intuitive, they leave a lot of components up to the developer to implement. Aptos introduces Move Objects to extend upon the basics of the Move model and offer a framework for creating and managing a globally accessible set of heterogeneous resources that expedites and improves the developer experience.

Why objects? Well minimally an object gets you access to many features including:

- A common framework that defines ownership of an asset. No need to have one-off asset management as was demonstrated for hero in the previous chapter, nor come up with mechanisms to differentiate between similar yet different assets, e.g.., those that share the same common name.
- Global addressability of all objects that only requires virtual or referenced nesting of assets. Weapons were only accessible if you knew which hero had them and further nesting of assets make it much more complex to manage both in terms of code and data model.
- Events or metadata that make it easier to reason about changes to the object. An area not yet touched upon, Move Objects not only represent an asset but can express events as actions are taken involving the objects, making it easier for off-chain understanding of complex on-chain interactions.
- A permission framework that makes data convenient and safe to use. Limiting the possibility that gaining access to a `signer`s could be abused to manipulate or affect data.

We’ll go into each one of these in greater depth and further emphasize the utility of Objects over resources alone.

## Characters as Objects

For the most part, swapping from resources to objects is largely a set of API changes, but doing so unlocks a lot of opportunities.

```rust
/// Path: fight_the_baddies/sources/character.move
module fight_the_baddies::character {
use std::error;
use std::option::{Self, Option};
use std::string::{Self, String};

use aptos_framework::object::{Self, Object};

use fight_the_baddies::weapon::{Self, Weapon};

friend fight_the_baddies::level_up;
friend fight_the_baddies::fight;

/// There was no character found at the specified address.
const ECHARACTER_NOT_FOUND: u64 = 1;

#[resource_group_member(group = aptos_framework::object::ObjectGroup)]
struct Character has key {
  name: String,
  gender: String,
  affinity: String,
  level: u8,
  health: u32,
  defense: u16,
  strength: u16,
  agility: u16,
  experience: u64,
  weapon: Option<Object<Weapon>>,
}

inline fun create_character(creator: &signer, character: Character): Object<Character> {
  let constructor_ref = object::create_object_from_account(creator);
  let obj_signer = object::generate_signer(&constructor_ref);
  move_to(&obj_signer, character);
  object::object_from_constructor_ref(&constructor_ref)
}

public fun create_hero(creator: &signer, name: String, gender: String): Object<Character> {
  let character = Character {
    name,
    gender,
    affinity: string::utf8(b"good"),
    level: 1,
    health: 32,
    defense: 5,
    strength: 3,
    agility: 8,
    experience: 0,
    weapon: option::none(),
  };
  create_character(creator, character)
}

public fun create_baddy(creator: &signer, name: String, gender: String): Object<Character> {
  let character = Character {
    name,
    gender,
    affinity: string::utf8(b"bad"),
    level: 1,
    health: 8,
    defense: 2,
    strength: 1,
    agility: 3,
    experience: 0,
    weapon: option::none(),
  };
  create_character(creator, character)
}

inline fun assert_exists<T: key>(character: &Object<T>): address {
  let character_address = object::object_address(character);
  assert!(
    exists<Character>(character_address),
    error::not_found(ECHARACTER_NOT_FOUND),
  );
  character_address
}

inline fun borrow<T: key>(character: &Object<T>): &Character acquires Character {
  let character_address = assert_exists(character);
  borrow_global<Character>(character_address)
}

inline fun borrow_mut<T: key>(character: &Object<T>): &mut Character acquires Character {
  let character_address = assert_exists(character);
  borrow_global_mut<Character>(character_address)
}

public fun is_hero<T: key>(character: &Object<T>): bool acquires Character {
  borrow(character).affinity == string::utf8(b"good")
}

public fun is_baddy<T: key>(character: &Object<T>): bool acquires Character {
  borrow(character).affinity == string::utf8(b"bad")
}

public fun name<T: key>(character: &Object<T>): String acquires Character {
  borrow(character).name
}

public fun gender<T: key>(character: &Object<T>): String acquires Character {
  borrow(character).gender
}

public fun level<T: key>(character: &Object<T>): u8 acquires Character {
  borrow(character).level
}

public fun health<T: key>(character: &Object<T>): u32 acquires Character {
  borrow(character).health
}

public fun defense<T: key>(character: &Object<T>): u16 acquires Character {
  borrow(character).defense
}

public fun strength<T: key>(character: &Object<T>): u16 acquires Character {
  borrow(character).strength
}

public fun agility<T: key>(character: &Object<T>): u16 acquires Character {
  borrow(character).agility
}

public fun effective_strength<T: key>(character: &Object<T>): u16 acquires Character {
  let character = borrow(character);
  let weapon = if (option::is_some(&character.weapon)) {
    weapon::strength(option::borrow(&character.weapon))
  } else {
    0
  };
  weapon + character.strength
}

public fun effective_agility<T: key>(character: &Object<T>): u16 acquires Character {
  let character = borrow(character);
  let weapon = if (option::is_some(&character.weapon)) {
    weapon::weight(option::borrow(&character.weapon))
  } else {
    0
  };
  if (character.agility < weapon) {
    0
  } else {
    character.agility - weapon
  }
}

public fun experience<T: key>(character: &Object<T>): u64 acquires Character {
  borrow(character).experience
}

public(friend) fun set_level<T: key>(character: &Object<T>, level: u8) acquires Character {
  borrow_mut(character).level = level
}

public(friend) fun set_health<T: key>(character: &Object<T>, health: u32) acquires Character {
  borrow_mut(character).health = health
}

public(friend) fun set_defense<T: key>(character: &Object<T>, defense: u16) acquires Character {
  borrow_mut(character).defense = defense
}

public(friend) fun set_strength<T: key>(character: &Object<T>, strength: u16) acquires Character {
  borrow_mut(character).strength = strength
}

public(friend) fun set_agility<T: key>(character: &Object<T>, agility: u16) acquires Character {
  borrow_mut(character).agility = agility
}

public(friend) fun set_experience<T: key>(character: &Object<T>, experience: u64) acquires Character {
  borrow_mut(character).experience = experience
}

public fun equip_weapon(
  owner: &signer,
  character: Object<Character>,
  weapon: Object<Weapon>,
) acquires Character {
  object::transfer_to_object(owner, character, weapon);
  let character = borrow_mut(&character);
  if (option::is_some(&character.weapon)) {
    let _old_weapon = option::extract(&mut character.weapon);
  };
  option::fill(&mut character.weapon, weapon);
}
}
```

### Object Identifiers

As Objects are accumulation of resources at an address, each object is effectively represented by an address. The syntax for representing Objects in Move is `Object<T>`, where the `T` denotes a resource within the Object. The only contents of `Object<T>` is the address where the Object is stored. The terms object id and address are typically used interchangeably, but as an address can point to data other than objects, the preferred terminology is object id or object identifier.

### Creating Objects

Objects can be created in a handful of ways:
* The context of a transaction, namely, the sender's state via a call to `object::create_object_from_account`
* From an existing object, `object::create_object_from_object`

Both of these will generate a randon object id that requires reading the output of a transaction to know the generated object id.

Objects can also be generated with a deterministic address via `object::create_named_object`.

### Resource Groups

### Inline Functions

## Weapon Objects

```rust
/// Path: fight_the_baddies/sources/weapon.move
module fight_the_baddies::weapon {
use std::error;
use std::string::{Self, String};

use aptos_framework::object::{Self, Object};

friend fight_the_baddies::character;

/// The was no weapon found at the specified address.
const EWEAPON_NOT_FOUND: u64 = 1;

#[resource_group_member(group = aptos_framework::object::ObjectGroup)]
struct Weapon has key {
  name: String,
  type: String,
  strength: u16,
  weight: u16,
}

inline fun create_weapon(creator: &signer, weapon: Weapon): Object<Weapon> {
  let constructor_ref = object::create_object_from_account(creator);
  let obj_signer = object::generate_signer(&constructor_ref);
  move_to(&obj_signer, weapon);
  object::object_from_constructor_ref(&constructor_ref)
}

public fun create_knife(creator: &signer, name: String): Object<Weapon> {
  let weapon = Weapon {
    name,
    type: string::utf8(b"knife"),
    strength: 2,
    weight: 1,
  };
  create_weapon(creator, weapon)
}

public fun create_sword(creator: &signer, name: String): Object<Weapon> {
  let weapon = Weapon {
    name,
    type: string::utf8(b"sword"),
    strength: 10,
    weight: 4,
  };
  create_weapon(creator, weapon)
}

public fun create_axe(creator: &signer, name: String): Object<Weapon> {
  let weapon = Weapon {
    name,
    type: string::utf8(b"axe"),
    strength: 17,
    weight: 6,
  };
  create_weapon(creator, weapon)
}

inline fun borrow<T: key>(weapon: &Object<T>): &Weapon acquires Weapon {
  let weapon_address = object::object_address(weapon);
  assert!(
    exists<Weapon>(weapon_address),
    error::not_found(EWEAPON_NOT_FOUND),
  );
  borrow_global<Weapon>(weapon_address)
}

public fun name<T: key>(weapon: &Object<T>): String acquires Weapon {
  borrow(weapon).name
}

public fun type<T: key>(weapon: &Object<T>): String acquires Weapon {
  borrow(weapon).type
}

public fun strength<T: key>(weapon: &Object<T>): u16 acquires Weapon {
  borrow(weapon).strength
}

public fun weight<T: key>(weapon: &Object<T>): u16 acquires Weapon {
  borrow(weapon).weight
}
}
```
