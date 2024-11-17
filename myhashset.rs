use std::collections::HashSet;

pub fn test_hashset_type() {
    let mut planets: HashSet<&str> = HashSet::from(["Mercury", "Venus", "Earth", "Saturn"]);
    let planets_more: HashSet<&str> = HashSet::from(["Mars", "Neptune", "Earth", "Jupiter"]);

    for planet in &planets {
        println!("{}", planet)
    }

    let planet_difference = planets.difference(&planets_more);
    let planet_symdiff = planets.symmetric_difference(&planets_more);
    // for planet_diff in planet_difference {
    //     println!("{}", planet_diff)
    // }
    for planet_diff in planet_symdiff {
        println!("{}", planet_diff)
    }

    planets.insert("Andromeda");
    planets.insert("Milky Way");

    for planet in &planets {
        println!("{}", planet)
    }
}
