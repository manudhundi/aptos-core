## Introducing Move

The first step in building any application is defining your data model. Move’s data model is based upon structures or structs. A struct can contain various primitive types as well as other structs, however, it cannot contain itself, Move, currently, does not support recursive data structures.

### Packages

A Move application consists of a series of packages where a package consists of one or more Move modules. Each Move module typically corresponds to a single file of Move code. To get our project started, we will build the following layout:

```rust
fight_the_baddies
├── Move.toml      (toml file describing the package)
├── sources        (directory containing all application code)
├── doc_templates  (documentation template directory)
└── tests          (test code that sits outside the application code)
```

Your `Move.toml` code will look like the following:

```toml
# Path: fight_the_baddies/Move.toml
[package]
name = 'fight_the_baddies'
version = '1.0.0'

[addresses]
fight_the_baddies = "_"

[dependencies.MoveStdlib]
git = 'https://github.com/aptos-labs/aptos-core.git'
rev = 'framework-mainnet'
subdir = 'aptos-move/framework/move-stdlib'
```

### Core Data

The core of our game are the characters. A character has an affinity, evil or good, various attributes, a name, and a gender:

```rust
/// Path: fight_the_baddies/sources/character.move
module fight_the_baddies::character {
use std::string::String;

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
}
```

This module defines the named address `fight_the_baddies` with the module `character`. This file name for this should be `character.move`. The struct itself shows the field labels and types including an example of nesting other structs like `String` and several primitives.

Move supports the following primitives:

```rust
address - a 256-bit value representing a location a location in global storage
bool - True/False
u8 - 8-bit unsigned integer
u16 - 16-bit unsigned integer
u32 - 32-bit unsigned integer
u64 - 64-bit unsigned integer
u128 - 128-bit unsigned integer
u256 - 256-bit unsigned integer
```

### Accessing Data

Now that we know the basics of data structure layout, we need a means to create the data structures and interact with them. As part of Move’s security primitives, a struct defined in one module cannot be directly created or manipulated in another. Specifically, the following example would fail to compile:

```rust
/// Path: fight_the_baddies/sources/game.move
/// Note: this is not a real file, it is intended as a demonstration
module fight_the_baddies::game {
  use std::string::{Self, String};
  use baddies::character::Character;

  public fun generate_character(name: String): Character {
    Character {
      name,
      gender: string::utf8(b"female"),
      affinity: string::utf8(b"good"),
      level: 1,
      health: 32,
      defense: 5,
      strength: 3,
      agility: 8,
      experience: 0,
    }
  }
}
```

Instead, all functions that directly create and manipulate must be defined in the original module, this allows the module to define what functionality is exported for other developers and users of the application. For example, it prevents anyone from creating a super hero in *****************Fight the Baddies.*****************

```rust
/// Path: fight_the_baddies/sources/character.move
module baddies::character {
use std::string::{Self, String};
...

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
}
```

Note, this guide represents earlier snippets of code with `...` to keep repeated code to a minimum.

There are several aspects of Move that stand out in the above code:

**********************************Importing Modules**********************************

We can import a module or structs and functions within it. `use address::module` imports only the module, whereas `use address::module::{Self, Struct, func}` would import the module, the struct `Struct` and the function `func`.

An import can be directly referenced by the top-level name imported.

If we did `use std::string` , we would have to define string types as `string::String`, whereas `use std::string::String` allows us to define types as `String`.

********************************Building and Using Structs********************************

When creating a struct, if a variable shares the name of a field, for example, `name: name` can be shortened to `name`.

Structure fields are accessed via dot notation, e.g., `character.agility` returns the agility field of the given character.

**********************************Calling Functions**********************************

Functions must be defined `public` to be accessed outside the module. A non-public module is inaccessible from outside that module.

When we call the public functions, we pass in a reference `&Character`, this means we’re literally passing around the character’s data back into the module to read fields.

******Struct Fields and the Copy Ability******

The fields are returned as values. Certain values can be seamlessly copied, for example:

```rust
struct Copyable has copy {
  value: u8,
}

struct NotCopyable {
  value: u8,
}

public fun copy_copyable(copyable: &Copyable): Copyable {
  *copyable
}

public fun copy_non_copyable(non_copyable: &NonCopyable): NonCopyable {
  NotCopyable {
    value: non_copyable.value,
  }
}
```

`Copyable` can be copied in any module and in any code without reservations. `NonCopyable` is only copyable because of the existence of the `copy_non_copyable` function, otherwise it is not.

************************************************************************General Overview of Struct Abilities************************************************************************

Struct definitions allow for a handful of abilities or behavior modifiers to be attributed to the struct. The general format is `struct Foo has ability0, ability1, ability3 { ... }`

There are 4 struct abilities in Move:

- Copy — enables implicit copy of a struct without calling into the module. A struct that has copy can be duplicated.
- Drop — a struct can be destroyed implicitly, there is no need to explicitly call a deconstructor or delete the struct.
- Key — the struct can be stored directly in global storage.
- Store — the struct can be stored indirectly in global storage.

While these terms may seem foreign at this point, they are covered in greater depth in their respective sections.

************************************************************To Return References or Values************************************************************

An acute observer may notice that we return a copy of the `String` when reading the `name` or `gender` attributes. `String` supports the `copy` ability and can be implicitly copied. In the above code, we could have also returned a `&String` as well, saving the VM from copying the `String` bytes. However, as we begin our exploration of storage, such semantics are currently not supported, hence we default to return by copying the value here.

### Mutating Data

Now that we have the ability to build characters, let’s get them fighting each other, taking damage, dying, and getting level ups.

```rust
/// Path: fight_the_baddies/sources/character.move
module baddies::character {
...
friend fight_the_baddies::level_up;
friend fight_the_baddies::fight;

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
```

In this extension of `character.move` , we introduce several new concepts:

****************************************************Module Accessibility via friend****************************************************

The notation `friend baddies::level_ups`, means that the module `baddies::level_ups` can call into any `public(friend)` function within this module. Only modules that have `friend` can access `public(friend)` and those modules must be defined at the same address and within the same package.

`friend` enables us to both reduce the code size per file and locate common logic, among many other benefits.

**************************************************Mutable Struct References**************************************************

Much like Rust, in Move, a data structure can only be mutated or changed if there’s a mutable handle to it or own it, such as `& mut Character` or `Character`. 

**********************Destruction and Dropping**********************

In Move, structs can not be silently deleted unless properly permissioned, `Character` has no such permission, so an explicit `destroy` function exists for the application to clean up characters that are no longer needed. Sometimes, structs benefit from implicit deletion, this can be done by using the `drop` ability on the struct:

```rust
struct Dropable has drop { }
public fun drop_me(droppable: Droppable) {}
```

This compiles without issue, whereas if `Droppable` did not have `drop` , compilation would fail.

### Data Across Modules

```rust
/// Path: fight_the_baddies/sources/fight.move
module fight_the_baddies::fight {
use std::error;
use fight_the_baddies::character::{Self, Character};
use fight_the_baddies::level_up;

/// Character was not a hero.
const ENOT_HERO: u64 = 1;
/// Character was not a baddy.
const ENOT_BADDY: u64 = 2;
/// Hero character has 0 health, i.e., is dead
const EHERO_DEAD: u64 = 3;
/// Baddy character has 0 health, i.e., is dead
const EBADDY_DEAD: u64 = 4;
/// Character is not daead.
const ENOT_DEAD: u64 = 5;

public fun fight(hero: &mut Character, baddy: &mut Character) {
  assert!(character::is_hero(hero), error::invalid_argument(ENOT_HERO));
  assert!(character::is_baddy(baddy), error::invalid_argument(ENOT_BADDY));
  assert!(character::health(hero) > 0, error::invalid_argument(EHERO_DEAD));
  assert!(character::health(baddy) > 0, error::invalid_argument(EBADDY_DEAD));

  attack(hero, baddy);
  if (character::health(baddy) > 0) {
    attack(baddy, hero);
    if (character::health(hero) == 0) {
      level_up::level_up(baddy, hero);
    }
  } else {
    level_up::level_up(hero, baddy);
  }
}

fun attack(left: &mut Character, right: &mut Character) {
  let left_str = character::strength(left);
  let right_def = character::defense(right);

  // Avoid the potential underflow and set the minimum damage to 1.
  let damage = if (left_str > right_def) {
    left_str - right_def
  } else {
    1
  };
  let damage = (damage as u32);

  let right_health = character::health(right);
  let new_health = if (right_health > damage) {
    right_health - damage
  } else {
    0
  };
  character::set_health(right, new_health);
}

public fun sacrifice(character: Character) {
  character::destroy(character);
}

public fun eliminate(character: Character) {
  assert!(character::health(&character) == 0, error::invalid_argument(ENOT_DEAD));
  character::destroy(character);
}
}
```

************Constants************

Move supports constants for creating easily identifiable values and avoiding magic numbers within code. Currently constants can only be used within the module that define it, so if you need to expose a constant across the module boundaries, export it as a function.

********************************************Conditional Evaluation********************************************

Like Rust, Move allows a value to be set as the output of conditional evaluation. That is, one can call `let value = if (something) value_a else value_b` . This paradigm allows for developers to avoid creating unnecessary mutable values.

******************************Assertions and Errors******************************

As we will see in later sections, Move’s default error messages can leave the developer confused about where the issue was. To that end, it is quite natural to add many invariants to the code. In the above code, the invariants provide more protection against users leveraging the functions incorrectly.

All the Move error types are defined in [error.move](https://aptos.dev/reference/move/?branch=mainnet&page=move-stdlib/doc/error.md#0x1_error). The comments above errors provide value to applications. This is returned during run-time if a assertion is violated.

Later on, we will demonstrate other mechanisms that provide better type-safety to limit the need for assertions in code.

**************Integer Underflow and Overflow**************

In Move, if you ever have an integer, even temporarily have a value outside its range, then the application will immediately abort. This includes even complex operations that might ultimately result in a viable number. For example, the following will both underflow:

`5 - 8`

`5 - 8 + 5`

So it is imperative to check conditions around these numbers. Move currently lacks signed integers, so there is no way to temporarily cast the numbers to evade bounds checking.

**************Casting**************

Move allows for casting between different types; however, it must be done explicitly. In the above snippet, `let damage = (damage as u32);` causes there to be a new variable named damage that is a `u32` whereas the original damage was a `u16`. If this was not done, the code would fail to compile due to an implicit type coercion of `u16` to a `u32`. It is important to note that the Move compiler is rather rigid in the syntax for casting `let damage = damage as u32;` fails to compile.

******************************Safe Destruction of Structs******************************

As mentioned earlier, `Character` could have had the ability `drop`, that would have let users delete the structs as they are no longer useful. Of course, in this game, we want to keep all characters accounted for and thus have an explicit destructor: `characer::destroy` . That function still requires a layer of indirection as it can only be called locally or via other `friend` modules. For this purpose, the `fight` module offers two functions: `eliminate` that destroys a `Character` with zero health and `sacrifice` that destroys any `Character`. At this point in the game play, they have limited special logic, but over time we can add more complexity.

This pattern is known as a **hot potato**, one can pass around a struct but must explicitly destroy it via another well-defined endpoint.

We’ll see this code leveraged in the test code presented in a bit.

### Rounding out the Basic Game Experience

In a traditional RPG game, as the character gains levels, the amount of experience gained from a victorious fight decreases based upon the level of the character receiving the experience. Alternatively, the amount of experience required per-level increases non-linearly. For this model, we chose a simple bit shifting model.

```rust
/// Path: fight_the_baddies/sources/level_up.move
module fight_the_baddies::level_up {
use std::error;
use fight_the_baddies::character::{Self, Character};

friend fight_the_baddies::fight;

/// Losing character is not dead.
const ENOT_DEAD: u64 = 1;
/// Winning character is dead.
const EDEAD: u64 = 2;

const BASE_EXP: u64 = 25;
const MAX_LEVEL: u64 = 255;
const MAX_EXP: u64 = 25500;

/// Provide experience and level increase. The algorithm is that each level requires
/// 100 experience points. Each victory for the same level results in 25 experience
/// points. That amount is shifted left for each level greater and right for each
/// level less than.
public(friend) fun level_up(winner: &mut Character, loser: &mut Character) {
  assert!(character::health(winner) > 0, error::invalid_argument(EDEAD));
  assert!(character::health(loser) == 0, error::invalid_argument(ENOT_DEAD));

  let winner_level = character::level(winner);
  let loser_level = character::level(loser);
  let win_exp = if (winner_level > loser_level) {
    BASE_EXP >> (winner_level - loser_level)
  } else {
    BASE_EXP << (loser_level - winner_level)
  };

  let current_exp = character::experience(winner);
  let new_exp = if (MAX_EXP < current_exp + win_exp) {
    MAX_EXP
  } else {
    current_exp + win_exp
  };
  character::set_experience(winner, new_exp);

  let current_level = (character::level(winner) as u64);
  let next_level = new_exp / 100;
  if (current_level < MAX_LEVEL && current_level < next_level) {
    let next_level = if (MAX_LEVEL < next_level) {
      MAX_LEVEL
    } else {
      next_level
    };
    character::set_level(winner, (next_level as u8));
  };
}
}
```

At this point, the above code should introduce no new concepts, but simply provides completeness to the application thus far. Note, the current level up scheme leaves out changes to attributes, we will return to that in a later section, when we introduce randomness.

### Building the Code

Earlier, we set up the Aptos CLI, as a reminder, you can follow this guide to [install the Aptos CLI](https://aptos.dev/tools/install-cli/).

Now let’s examine some test code that proves the code written thus far actually works!

First we can verify that the code compiles by executing:

```bash
aptos move compile \
  --package-dir fight_the_baddies \
  --named-addresses fight_the_baddies=0xf00ba5
```

This assumes that the package `fight_the_baddies` with the `Move.toml` and `sources` is located in the current directory. Note, `fight_the_baddies` was set to `"_"` in the `Move.toml` , so to build, we must supply a named address. We have chosen the address `0xf00ba5` at random.

### End to End Tests

Now that we have a working CLI and code that can be built, it is time to build a tests. First the test code:

```rust
/// Path: fight_the_baddies/tests/end_to_end.move
#[test_only]
module fight_the_baddies::end_to_end {
use std::string;
use fight_the_baddies::character::{Self, Character};
use fight_the_baddies::fight;
use fight_the_baddies::test_utils;

#[test]
fun generate_and_destroy_hero() {
  let hero = test_utils::get_hero();
  assert!(character::is_hero(&hero), 0);
  assert!(character::name(&hero) == string::utf8(b"Alice"), 1);
  assert!(character::gender(&hero) == string::utf8(b"female"), 2);
  fight::sacrifice(hero);
}

#[test]
fun generate_and_destroy_baddy() {
  let baddy = test_utils::get_baddy();
  assert!(character::is_baddy(&baddy), 0);
  assert!(character::name(&baddy) == string::utf8(b"Bob"), 1);
  assert!(character::gender(&baddy) == string::utf8(b"male"), 2);
  fight::sacrifice(baddy);
}

#[test]
/// The goal with this test is to get level ups, so we'll create a hero who always kill steals
/// without taking damage. Then we'll create new heroes along the way to beat the baddy to near
/// death.
fun end_to_end() {
  let main_hero = test_utils::get_hero();
  let current_level = character::level(&main_hero);

  while (current_level == character::level(&main_hero)) {
    let sad_hero = test_utils::get_hero();
    let baddy = test_utils::get_baddy();
    let main_str = character::strength(&main_hero);
    let baddy_def = character::defense(&baddy);

    while (character::health(&baddy) > 0) {
      let baddy_health = character::health(&baddy);
      if (baddy_health + (baddy_def as u32) <= (main_str as u32)) {
          fight::fight(&mut main_hero, &mut baddy)
      } else {
          fight::fight(&mut sad_hero, &mut baddy)
      };
    };
    fight::sacrifice(sad_hero);
    fight::eliminate(baddy);
  };

  // We should only get one level up given the current mechanics
  assert!(current_level + 1 == character::level(&main_hero), 0);
  fight::sacrifice(main_hero);
  // The test will timeout if it cannot get here.
}
}
```

```rust
/// Path: fight_the_baddies/tests/test_utils.move
#[test_only]
module fight_the_baddies::test_utils {
use std::string;
use fight_the_baddies::character::{Self, Character};
public fun get_baddy(): Character {
  character::generate_baddy(
    string::utf8(b"Bob"),
    string::utf8(b"male"),
  )
}
public fun get_hero(): Character {
  character::generate_hero(
    string::utf8(b"Alice"),
    string::utf8(b"female"),
  )
}
}
```

****************************Running Tests****************************
In order to run the tests, execute the following command:

```bash
aptos move test \
  --package-dir fight_the_baddies \
  --named-addresses fight_the_baddies=0xf00ba5
```

This is very similar to the compile command, but it searches the `sources` and `tests` diretory for any functions labeled `#[test]` it also enables compiling all code marked as `#[test_only]`. As demonstrated above, those components allow us to mix production code with test code. This also means we can have our test code next to our production code without worrying whether it will be accessible to production deployments. It will not.

******************************About the Tests******************************

The tests above are pretty simple. It uses a bit of knowledge of the game mechanics to ensure that a hero will defeat a series of baddies to get a level up. Of course, the game currently doesn’t have the mechanism to add life back to the hero, so we are left using `sad_hero` to do all the dirty work, so that `main_hero` can claim the kill.

******************************************************Leveraging the Safe Destroy******************************************************

In this test, we create several heroes, `sad_hero` that sacrifice their health to basically kill the `baddy` before the `main_hero` swoops in for the kill without taking any damage. Because of the lack of `drop` on `Character` . We explicitly call into `character::eliminate` for `baddy` , as it is dead. We also explicitly call into `character::sacrifice` for `sad_hero` , since it is most likely still alive.

### Conclusion

By the end of this section, we have built a trivial RPG. We can create new characters, level them up, and destroy them. We have a fully functioning yet not persistent Move program. But who wants to play an RPG that can only persist the duration of a single call? In the next section, we’ll understand how persistent storage works in Move and Aptos Move.

## Nesting Structures

At this point in time, we have a working mini-game, but it lacks a bit of personalization. In any RPG, one would expect to be able to customize their character with new weapons and equipment. In Move this involves exploring the concepts of nested structs and options. It also begins our path into more subtle challenges of developing with basic Move that amplifies our direction in Aptos.

### Adding Weapons

For the most part, adding weapons involves adding a new source `weapon.move` and a few modifications to the other files to address the impact of this new functionality. The core change to our existing game play is that weapons allow a character to inflict additional damage based upon the strength of the weapon. If there is no weapon, there is no increase in strength. We also add in the weight field for potential future use:

```rust
/// Path: fight_the_baddies/sources/weapon.move

module fight_the_baddies::weapon {
use std::string::{Self, String};

friend fight_the_baddies::character;
struct Weapon {
  name: String,
  type: String,
  strength: u16,
  weight: u16,
}

public fun generate_knife(name: String): Weapon {
  Weapon {
    name,
    type: string::utf8(b"knife"),
    strength: 2,
    weight: 1,
  }
}

public fun generate_sword(name: String): Weapon {
  Weapon {
    name,
    type: string::utf8(b"sword"),
    strength: 10,
    weight: 4,
  }
}

public fun generate_axe(name: String): Weapon {
  Weapon {
    name,
    type: string::utf8(b"axe"),
    strength: 17,
    weight: 6,
  }
}

public fun name(weapon: &Weapon): String {
  weapon.name
}

public fun type(weapon: &Weapon): String {
  weapon.type
}

public fun strength(weapon: &Weapon): u16 {
  weapon.strength
}

public fun weight(weapon: &Weapon): u16 {
  weapon.weight
}

public(friend) fun destroy(weapon: Weapon) {
  let Weapon {
    name: _,
    type: _,
    strength: _,
    weight: _,
  } = weapon;
}
}
```

There’s nothing that new in the above module from previous.

### Equipping Weapons

In this section, we make a few modifications to the character module to take into consideration weapons:

```rust
/// Path: fight_the_baddies/sources/character.move

module fight_the_baddies::character {
use std::option::{Self, Option};
use std::string::{Self, String};
use fight_the_baddies::weapon::{Self, Weapon};

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
  weapon: Option<Weapon>,
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
    weapon: option::none(),
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
    weapon: option::none(),
  }
}

...

public fun effective_strength(character: &Character): u16 {
  let weapon = if (option::is_some(&character.weapon)) {
    weapon::strength(option::borrow(&character.weapon))
  } else {
    0
  };
  weapon + character.strength
}

public fun effective_agility(character: &Character): u16 {
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

...

public fun equip_weapon(character: &mut Character, weapon: Weapon) {
  if (option::is_some(&character.weapon)) {
    let old_weapon = option::extract(&mut character.weapon);
    weapon::destroy(old_weapon);
  };
  option::fill(&mut character.weapon, weapon);
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
    weapon,
  } = character;

  if (option::is_some(&weapon)) {
    weapon::destroy(option::extract(&mut weapon));
  };
  option::destroy_none(weapon);
}
}
```

### Nesting Structs

Move allows structs to be seamlessly nested within each other. This was demonstrated by allowing a `Character` to contain a `Weapon` and has been demonstrated prior by having the various structs contain strings. When a struct is contained within another struct the entirety of that data is stored therein.

For example, when we create the weapon via `generate_sword`, the weapon is returned to the caller as a value. That means we are passing around the entirety of struct. When the struct is placed into the `Option` within the `weapon` field on `Character`, the operation moves the value or the instance of the struct into that storage location.

This is an example of nesting struct values within each other.

### Introducing Options

There are often times where there is no default value for a struct, especially at time of creation. In the above example, characters may be empty handed, there may be no default weapon. To represent these occurrences, Move provides an `Option` container. This allows the `Character` instance to be created without adding in boiler plate code for an non-existent weapon.

There are some caveats with using an `Option`:

- There are no implicit interactions with an `Option` so values must be explicitly borrowed, checked, extracted.
- `Option` requires negligible more storage (a byte) and slightly more execution costs due to the additional function calls.
- On the storage side, `Option` leverages the same layout as a `Vector`, which may be confusing at first glance, but does make for a rather simplified view point for an empty or non-empty field.

### Implications to Gameplay

The testing harness makes a few adjustments to make use of weapons. Notably, that a character can equip a weapon and that weapon can give the character a higher effective strength. We then demonstrate this by simplifying the `end_to_end` test below:

```rust
/// Path: fight_the_baddies/sources/end_to_end.move
#[test_only]
module fight_the_baddies::end_to_end {
use std::string;
use fight_the_baddies::character::{Self, Character};
use fight_the_baddies::fight;
use fight_the_baddies::weapon;

#[test]
fun generate_and_destroy_hero() {
  let hero = get_hero();
  assert!(character::is_hero(&hero), 0);
  assert!(character::name(&hero) == string::utf8(b"Alice"), 1);
  assert!(character::gender(&hero) == string::utf8(b"female"), 2);
  assert!(character::strength(&hero) == character::effective_strength(&hero), 3);
  character::equip_weapon(&mut hero, weapon::generate_sword(string::utf8(b"katana")));
  assert!(character::strength(&hero) < character::effective_strength(&hero), 3);
  fight::sacrifice(hero);
}

#[test]
fun generate_and_destroy_baddy() {
  let baddy = get_baddy();
  assert!(character::is_baddy(&baddy), 0);
  assert!(character::name(&baddy) == string::utf8(b"Bob"), 1);
  assert!(character::gender(&baddy) == string::utf8(b"male"), 2);
  fight::sacrifice(baddy);
}

#[test]
/// The goal with this test is to get level ups, in this scenario the sword is over-powering,
/// which enables single blows to fell the baddy.
fun end_to_end() {
  let hero = get_hero();
  character::equip_weapon(&mut hero, weapon::generate_sword(string::utf8(b"katana")));
  let current_level = character::level(&hero);
  while (current_level == character::level(&hero)) {
    let baddy = get_baddy();
    fight::fight(&mut hero, &mut baddy);
    fight::eliminate(baddy);
  };
  // We should only get one level up given the current mechanics
  assert!(current_level + 1 == character::level(&hero), 0);
  fight::sacrifice(hero);
  // The test will timeout if it cannot get here.
}

fun get_baddy(): Character {
  character::generate_baddy(
    string::utf8(b"Bob"),
    string::utf8(b"male"),
  )
}

fun get_hero(): Character {
  character::generate_hero(
    string::utf8(b"Alice"),
    string::utf8(b"female"),
  )
}
}
```

## Global Storage

In this section, we review concepts that make it possible to build meaningful applications in Move. We still refrain from introducing Aptos-specific concepts and focus on the core components of the language. By the end of this section, you will have written a complete and useful on-chain game!

Persistent data is stored in global storage. Global storage is represented as the following:

```rust
Resources: BTreeMap<address, BTreeMap<name, vec<u8>>;
Modules: BTreeMap<address, BTreeMap<module_name, vec<u8>>;

Where name is the module name and the struct name.
```

Applications cannot directly read or write to module storage and resource storage requires the use of certain functions and other annotations on the struct, as we’ll cover.

TODO: Introduce BCS, modules and types

### Making Our Characters Persistent

This is our first option to storing characters by making them resources within storage. A resource is a top level struct in the global storage. Returning back to the earlier code, we make the following modifications:

```rust
/// Path: fight_the_baddies/sources/character.move
module fight_the_baddies::character {
use std::error;
use std::option::{Self, Option};
use std::signer;
use std::string::{Self, String};

use fight_the_baddies::weapon::{Self, Weapon};

friend fight_the_baddies::level_up;
friend fight_the_baddies::fight;

/// Only a single character resource can be stored at a given address.
const ECANNOT_STORE_MULTIPLE_CHARACTERS: u64 = 1;
/// There is no character resource stored at this address.
const ENO_CHARACTER_STORED: u64 = 2;

/// Key enables this to be stored to persistent storage
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
  weapon: Option<Weapon>,
}

public function store(owner: &signer, character: Character) {
  assert!(
    !exists<Character>(signer::address_of(owner)),
    error::already_exists(ECANNOT_STORE_MULTIPLE_CHARACTERS),
  );
  move_to(owner, character);
}

/// Protected load function, only the owner can load the character
public function load(owner: &signer): Character acquires Character {
  let owner_addr = signer::address_of(owner);
  assert!(
    exists<Character>(owner_addr),
    error::not_found(ENO_CHARACTER_STORED),
  );
  move_from(owner_addr)
}

...
}
```

****************************Signers and Basic Account Concepts****************************

The above code introduces the concept of a `signer`. The `signer` represents ownership of an address from the perspective of global storage. As we begin to explore how Move binds to real applications, we’ll see that the `signer` is the entity that submitted the transaction.

************************Storing Global Data************************

The above snippet introduces the `key` ability for structs. `key` allows a structure to be stored into persistent storage via the `move_to` function and extracted from storage via the `move_from` function. A struct stored in global storage is known as a **resource**. Note that while Move does support the ability to borrow data from storage, a function cannot return a reference to data stored in global storage. As a result, it is not applicable yet. There is also an `exists` function that lets us check if a resource exists at an address.

It is important to note that all these storage functions can only access structs defined within the module. One module cannot directly access another modules global storage.

The biggest limitation to this model is that it can only support a single character per account. This is because storage, by default, has no concept of lists.

********************************************Storing Nested Data********************************************

By default, Move would not allow `Weapon` to be stored with the `Character`. To enable that, we added the `store` ability to weapon:

```rust
struct Weapon has store {
  name: String,
  type: String,
  strength: u16,
  weight: u16,
}
```

This minor change allows additional structs to be stored into global storage as part of a resource but not as a resource itself. We’ll go over this again in more detail in the next section.

**************************Testing**************************

```rust
/// Path: fight_the_baddies/tests/character_as_a_resource.move
#[test_only]
module fight_the_baddies::character_as_a_resource {
use std::string;

use fight_the_baddies::character;
use fight_the_baddies::fight;
use fight_the_baddies::test_utils;
use fight_the_baddies::weapon;

#[test(owner = @0xa11ce)]
fun store_and_load(owner: &signer) {
  let hero = test_utils::get_hero();
  character::equip_weapon(&mut hero, weapon::generate_sword(string::utf8(b"katana")));
  let experience = character::experience(&hero);
  assert!(experience == 0, 1);

  let baddy = test_utils::get_baddy();
  fight::fight(&mut hero, &mut baddy);
  fight::eliminate(baddy);

  let experience = character::experience(&hero);
  assert!(experience > 0, 1);

  character::store(owner, hero);
  let resumed_hero = character::load(owner);
  let resumed_experience = character::experience(&resumed_hero);
  assert!(resumed_experience == experience, 2);
  assert!(character::strength(&resumed_hero) < character::effective_strength(&resumed_hero), 3);

  character::store(owner, resumed_hero);
}

#[test(owner = @0xa11ce)]
#[expected_failure(abort_code = 0x80001, location = fight_the_baddies::character)]
fun store_two(owner: &signer) {
  let hero = test_utils::get_hero();
  character::store(owner, hero);
  let hero = test_utils::get_hero();
  character::store(owner, hero);
}

#[test(owner = @0xa11ce)]
#[expected_failure(abort_code = 0x60002, location = fight_the_baddies::character)]
fun load_none(owner: &signer) {
  let char = character::load(owner);
  fight::sacrifice(char);
}
}
```

******************************************Signers in Test Cases******************************************

As signers are so critical to the global storage mode, we need a means to generate them in test cases. Move offers this functionality via parameters in the `test` attribute. Specifically, `name = @address` will ensure that the parameter in the function with `name` will get a signer for the address defined at `@address`. Note, Move uses the `@` character to represent address values.

******************************************************Verifying Global Storage******************************************************

`store_and_load` demonstrates an important concept in Move, that is resource conservation. Because the `Character` struct does not have the `drop` ability, we must either explicitly destroy the character or alternatively we can place it into storage, as done on the last line `character::store`.

********************************************Testing Negative Cases********************************************

In testing, it is often critical to evaluate that errors are properly handled. We added `store_two` and `load_none` to demonstrate thendis. Specifically these functions have the `expected_failure` attribute, the `abort_code` , and the module or `location` where the `abort_code` was triggered. This means the test will only pass if the function aborts with the `abort_code` emitted from the `location` specified in the test attributes.

### Storing Multiple Characters

In order to store multiple characters, we need to make a couple of changes around the way in which the character is stored. That is, it is no longer a resource but instead must be stored within a resource that enables storing multiples. For that, we will employ the Move standard library’s vector.

```rust
/// Path fight_the_baddies/sources/character.move
module fight_the_baddies::character {
use std::error;
use std::signer;
use std::string::{Self, String};
use std::vector;

friend fight_the_baddies::level_up;
friend fight_the_baddies::fight;

/// The character store already has a charcter with that name.
const ENAME_AREADY_IN_USE: u64 = 1;
/// There is no character store at the specified address.
const ENO_CHARACTERS_STORED: u64 = 2;
/// There was no character by the given name found at the specified address.
const ECHARACTER_NOT_FOUND: u64 = 3;

struct Character has store {
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

struct CharacterStore has key {
  characters: vector<Character>,
}

public fun store(owner: &signer, character: Character) acquires CharacterStore {
  let owner_addr = signer::address_of(owner);
  if (!exists<CharacterStore>(owner_addr)) {
    move_to(owner, CharacterStore { characters: vector::empty() });
  };

  let new_name = name(&character);
  let character_store = borrow_global_mut<CharacterStore>(owner_addr);

  let idx = 0;
  while (idx < vector::length(&character_store.characters)) {
    let existing_character = vector::borrow(&character_store.characters, idx);
    assert!(
      new_name != name(existing_character),
      error::invalid_argument(ENAME_AREADY_IN_USE),
    );
    idx = idx + 1;
  };

  vector::push_back(&mut character_store.characters, character);
}

/// Protected load function, only the owner can load the character
public fun load(owner: &signer, lookup_name: String): Character acquires CharacterStore {
  let owner_addr = signer::address_of(owner);
  assert!(
    exists<CharacterStore>(owner_addr),
    error::not_found(ENO_CHARACTERS_STORED),
  );

  let character_store = borrow_global_mut<CharacterStore>(owner_addr);

  let idx = 0;
  while (idx < vector::length(&character_store.characters)) {
    let character = vector::borrow(&character_store.characters, idx);
    if (name(character) == lookup_name) {
      return vector::swap_remove(&mut character_store.characters, idx)
    };
    idx = idx + 1;
  };

  abort(error::not_found(ECHARACTER_NOT_FOUND))
}
```

**********************************************Storing Multiple Structs of the Same Type**********************************************

The attribute `store` let’s you store the struct within a nested struct that eventually is part of global storage. This layer of indirection allows developers a lot of flexibility in where data is stored. In the above example, the `CharacterStore` becomes the value in global storage. It contains a `vector<Character>` . The `vector` let’s us place many characters into global storage without creating the conflicts of shared space that limited the `Character` as a resource.

In the above example, each character is expected to have a unique name within the `CharacterStore` of a user. While there are other methods for creating uniqueness, such as giving each character a unique numeric id, this was chosen for brevity as our destination method resolves this constraint.

**************Testing**************

The testing code this time introduces no new concepts and is presented below for completeness:

```rust
/// Path: fight_the_baddies/tests/character_as_a_resource.move
#[test_only]
module fight_the_baddies::character_as_a_resource {
use std::string;
use fight_the_baddies::character;
use fight_the_baddies::fight;
use fight_the_baddies::test_utils;

#[test(owner = @0xa11ce)]
fun store_and_load(owner: &signer) {
  let hero = test_utils::get_hero();
  let experience = character::experience(&hero);
  assert!(experience == 0, 1);

  let baddy = test_utils::get_baddy();
  while (character::health(&baddy) > 0) {
    fight::fight(&mut hero, &mut baddy);
  };

  let experience = character::experience(&hero);
  assert!(experience > 0, 1);

  let hero_name = character::name(&hero);
  character::store(owner, hero);
  let baddy_name = character::name(&baddy);
  character::store(owner, baddy);

  let resumed_hero = character::load(owner, hero_name);
  let resumed_experience = character::experience(&resumed_hero);
  assert!(resumed_experience == experience, 2);
  character::store(owner, resumed_hero);

  let resumed_baddy = character::load(owner, baddy_name);
  let resumed_health = character::health(&resumed_baddy);
  assert!(resumed_health == 0, 3);
  character::store(owner, resumed_baddy);
}

#[test(owner = @0xa11ce)]
#[expected_failure(abort_code = 0x10001, location = fight_the_baddies::character)]
fun store_same_name(owner: &signer) {
  let hero = test_utils::get_hero();
  character::store(owner, hero);
  let hero = test_utils::get_hero();
  character::store(owner, hero);
}

#[test(owner = @0xa11ce)]
#[expected_failure(abort_code = 0x60002, location = fight_the_baddies::character)]
fun load_none(owner: &signer) {
  let char = character::load(owner, string::utf8(b"alice"));
  fight::sacrifice(char);
}

#[test(owner = @0xa11ce)]
#[expected_failure(abort_code = 0x60003, location = fight_the_baddies::character)]
fun load_missing(owner: &signer) {
  let hero = test_utils::get_hero();
  character::store(owner, hero);
  let char = character::load(owner, string::utf8(b"fake_alice"));
  fight::sacrifice(char);
}
}
```

## Move Objects

The previous section introduced global storage and two mechanisms for placing structs into storage: `key` and `store`. While both approaches work and are intuitive, they leave a lot of components up to the developer to implement. Aptos introduces Move Objects to extend upon the basics of the Move model and offer a framework for creating and managing a globally accessible set of heterogeneous resources that can expedite and improve the developer experience.

Why objects? Well minimally an object gets you access to many cool features including:

- A common framework that defines ownership of an asset. No need to have one-off asset management as was demonstrated for hero in the previous chapter, nor come up with mechanisms to differentiate between similar yet different assets, e.g.., those that share the same common name.
- Global addressability of all objects that only requires virtual or referenced nesting of assets. Weapons were only accessible if you knew which hero had them and further nesting of assets make it much more complex to manage both in terms of code and data model.
- Events or metadata that make it easier to reason about changes to the object. An area not yet touched upon, Move Objects not only represent an asset but can express events as actions are taken involving the objects, making it easier for off-chain understanding of complex on-chain interactions.
- A permission framework that makes data convenient and safe to use. Limiting the possibility that `signer`s could be abused to manipulate or affect data.

We’ll go into each one of these in greater depth and further emphasize the utility of Objects over resources alone.

### Characters as Objects

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

************************************What is a Move Object?************************************

An object an Aptos is represented by a set of resources including the `ObjectCore` resource stored at an address within global storage. Objects offer interoperability by defining a core set of primitives via the `ObjectCore` resource. `ObjectCore` defines several basic properties of the object including who the owner is, whether it can be transferred and events notifying transferring. It also allows for users to add events that are sourced from the object.

********************************Creating Objects********************************

### Creating Objects

### Resource Groups

### Weapon Objects

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

## Enter Aptos

Before pressing forward in our fight against the baddies, let’s explore other aspects of the Aptos to better understand how it can be used extend our gaming mechanics both on and off-chain. By the end of this chapter, you will understand:

- How to run and interact with a local Aptos testnet
- The on-chain Aptos account
- How to emit events on-chain and read them off-chain
- Setup a shell script for end-to-end tests

### Running a Local Testnet

To begin, please [install the Aptos CLI](https://aptos.dev/tools/install-cli/), first. Note, a lot of this content can be found in [Aptos.dev site](https://aptos.dev/nodes/local-testnet/using-cli-to-run-a-local-testnet).

****************************Prepare a User****************************

As most of our interaction will be with the local testnet, we’ll create two identities

```bash
# Create alice's profile
aptos init \
  --profile alice \
  --rest-url http://localhost:8080 \
  --faucet-url http://localhost:8081

# Create bob's profile
aptos init \
  --profile alice \
  --rest-url http://localhost:8080 \
  --faucet-url http://localhost:8081
```

The addresses and private keys will persist for all future testnets, so we will not need to call init again. Instead we will always start with a fresh testnet.

****************************************Starting the Testnet****************************************

To start a local testnet, execute the following command:

```bash
aptos node run-local-testnet --with-faucet --force-restart --assume-yes
```

This guide always assumes a pristine state, hence the inclusion of `--force-restart` 

The output from the command should be similar to the following:

```bash
Completed generating configuration:
        Log file: "/home/davidiw/aptos/aptos-core/.aptos/testnet/validator.log"
        Test dir: "/home/davidiw/aptos/aptos-core/.aptos/testnet"
        Aptos root key path: "/home/davidiw/aptos/aptos-core/.aptos/testnet/mint.key"
        Waypoint: 0:e881e7134588985689c47a8c5c6a15dd4d95f72e5e68ec6246a2f3a6d65ddc45
        ChainId: testing
        REST API endpoint: http://0.0.0.0:8080
        Metrics endpoint: http://0.0.0.0:9101/metrics
        Aptosnet fullnode network endpoint: /ip4/0.0.0.0/tcp/6181

Aptos is running, press ctrl-c to exit

Faucet is running. Faucet endpoint: http://0.0.0.0:8081
```

**************************************Creating and Funding the Account**************************************

As we already have profiles for Alice and Bob, we can execute the following commands to get them some useful test coins:

```bash
aptos account fund-with-faucet --profile alice --account alice
aptos account fund-with-faucet --profile bob --account bob
```

### The Aptos Account

Within this book, we will leverage some of the fields and functions associated with the Aptos Account and so we provide some details to help build context on nuances of Aptos.

When we create and fund an account, it causes the Move Aptos run-time to actually create [accounts on-chain](https://aptos.dev/concepts/accounts) and store some testnet Apt coin with those accounts.

An account on Aptos is a collection of metadata that describes ownership of an account and a set of assets managed by that account. In terms of the technical aspects, there’s a 1:1 mapping between address and account. An account is a collection of resources stored at a specific address. Furthermore, the Aptos [`Account`](https://aptos.dev/reference/move/?branch=mainnet&page=aptos-framework/doc/account.md#0x1_account) structure embeds relevant metadata like the account sequence number, authentication key, and a globally unique identifier generator.

When funding an account, that creates a second resource, the `[CoinStore](https://aptos.dev/reference/move/?branch=mainnet&page=aptos-framework/doc/coin.md#0x1_coin)` for `[AptosCoin](https://aptos.dev/reference/move/?branch=mainnet&page=aptos-framework/doc/aptos_coin.md#0x1_aptos_coin)` . This is the utility token for the Aptos blockchain and is consumed as gas for each transaction submitted.

### Understanding Transaction Output

The most basic transaction on the Aptos blockchain is `0x1::coin::transfer` , this represents transfer of a specified amount of `AptosCoin` from the account sending the transaction to the account addressed in the parameters.

In the following output, the account `0x810026ca8291dd88b5b30a1d3ca2edd683d33d06c4a7f7c451d96f6d47bc5e8b` submitted and executed a transaction that called the following function

```bash
{
  "version": "13629679",
  "gas_used": "4",
  "success": true,
  "vm_status": "Executed successfully",
  "changes": [
    {
      "address": "0xb258b91eee04111039320a85b0c24a2dd433909e14a6b5c32ee722e0fdecfddc",
      "data": {
        "type": "0x1::coin::CoinStore<0x1::aptos_coin::AptosCoin>",
        "data": {
          "coin": {
            "value": "1000"
          },
          "deposit_events": {
            "counter": "1",
            "guid": {
              "id": {
                "addr": "0x5098df8e7969b58ab3bd2d440c6203f64c60a1fd5c08b9d4abe6ae4216246c3e",
                "creaton_num": "2",
              }
            }
          },
          ...
        }
      },
      "type": "write_resource"
    },
    ...
  ],
  "sender": "0x810026ca8291dd88b5b30a1d3ca2edd683d33d06c4a7f7c451d96f6d47bc5e8b",
  "sequence_number": "0",
  "max_gas_amount": "2000",
  "gas_unit_price": "1",
  "expiration_timestamp_secs": "1660616127",
  "payload": {
    "function": "0x1::coin::transfer",
    "type_arguments": [
      "0x1::aptos_coin::AptosCoin"
    ],
    "arguments": [
      "0x5098df8e7969b58ab3bd2d440c6203f64c60a1fd5c08b9d4abe6ae4216246c3e",
      "1000"
    ],
    "type": "entry_function_payload"
  },
  "events": [
    {
      "key": "0x0300000000000000810026ca8291dd88b5b30a1d3ca2edd683d33d06c4a7f7c451d96f6d47bc5e8b",
      "guid": {
        "id": {
          "addr": "0x810026ca8291dd88b5b30a1d3ca2edd683d33d06c4a7f7c451d96f6d47bc5e8b",
          "creation_num": "3"
          }
        }
      },
      "sequence_number": "0",
      "type": "0x1::coin::WithdrawEvent",
      "data": {
        "amount": "1000"
      }
    },
    {
      "key": "0x02000000000000005098df8e7969b58ab3bd2d440c6203f64c60a1fd5c08b9d4abe6ae4216246c3e",
      guid": {
        "id": {
          "addr": "0x5098df8e7969b58ab3bd2d440c6203f64c60a1fd5c08b9d4abe6ae4216246c3e",
          "creation_num": "2"
          }
        }
      },
      "sequence_number": "0",
      "type": "0x1::coin::DepositEvent",
      "data": {
        "amount": "1000"
      }
    }
  ],
  "timestamp": "1660615531147935",
  "type": "user_transaction"
}
```

### Emitting Events On-Chain and Reading Them Off-Chain

Storage efficiency is key to a blockchain as it scales. The way in which transactions store their output to the blockchain can be impacted as a result thereof, impeding readability as a result. In fact, even updating a small chunk of data can result in a relatively large update if that data lives within a much larger struct. As a result, determining the specific effects of a transaction can be very difficult.

Aptos supports events in Move that allow developers to indicate state changes. Events are represented as a Move struct and stored as part of the transaction output. Events must be emitted from event handles, wherein each handle allocates a unique, sequential sequence number for each event. Thus events are fully countable. The only caveat with events is that their contents are driven by the developer, ideally they accurately represent the changes in storage, but they might also not. It is imperative to verify the authenticity of events both when processing them and prior to using them in production.

### A Simple Approach to End-to-End Tests

Aptos accounts, events, transactions, and running a local blockchain for end-to-end testing.

## Aptos Objects

Objects, extensions, and value of global storage versus store

## Aptos Tokens

Unification on a global type for interoperability, marketplaces, swaps

## On the Aptos Blockchain

Understanding game state, users, indexers, and transactions

## Advanced Concepts

Randomness
