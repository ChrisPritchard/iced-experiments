

use std::collections::HashSet;

use crate::world::prelude::*;

use rand::prelude::SliceRandom;

pub fn new_world() -> World {
    let mut world = World::default();

    let city_entities = add_cities(&mut world);

    let rng = &mut rand::thread_rng();
    for _ in 0..20 {
        let city = *city_entities.choose(rng).unwrap();
        add_test_agent(&mut world, city);
    }

    world 
}

fn add_cities(world: &mut World) -> Vec<Entity> {
    let cities: Vec<(City, Info)> = get_all_city_names().iter().map(|c| (City{}, Info{name: c.to_string(), description: "".to_string()})).collect();
    world.extend(cities).into()
}

fn get_used_agent_names(world: &World) -> HashSet<String> {
    <(&Agent, &Info)>::query().iter(world).map(|(_,i)| i.name.clone()).collect()
}

fn add_test_agent(world: &mut World, starting_city: Entity) {
    let existing_names = get_used_agent_names(world);
    let idle_order = (OrderInfo{next_order: None}, OrderType::Idle(starting_city));
    let initial_status = world.push(idle_order);
    let agent = (
        Agent::new("Player".into(), initial_status), 
        generate_random_agent_info(existing_names)
    );
    world.push(agent);
}

fn get_all_city_names() -> Vec<&'static str> {
    vec![
        "London", "Paris", "Moscow", "Beijing", "Tokyo", "New York", "Los Angeles", "Berlin", "Rome",
        "Sydney", "Cairo", "Mumbai", "Rio de Janeiro", "Toronto", "Amsterdam", "Seoul", "Istanbul",
        "Bangkok", "Dubai", "Athens", "Havana", "Nairobi", "Cape Town", "Buenos Aires", "Stockholm",
        "Mexico City", "New Delhi", "Singapore", "Vienna", "Prague",
        "Wellington", "Washington DC", 
    ]
}

fn generate_random_agent_info(existing_names: HashSet<String>) -> Info {
    let first_names = vec![
        "Bond", "Nikita", "Evelyn", "Archer", "Lara", "Jason", "Natasha", "Ethan", "Dominic", "Max",
        "Sydney", "Xander", "Athena", "Jack", "Isabella", "Gideon", "Valentina", "Cole", "Serena", "Alexei",
    ];

    let last_names = vec![
        "Black", "Blade", "Steele", "Frost", "Silver", "Stone", "Knight", "Phoenix", "Wolfe", "Steel",
        "Fox", "Hawke", "Viper", "Raven", "Shadow", "Saber", "Dragon", "Jagger", "Slate", "Hunter",
    ];

    loop {
        let first_name = first_names.choose(&mut rand::thread_rng()).unwrap().to_string();
        let last_name = last_names.choose(&mut rand::thread_rng()).unwrap().to_string();

        let name = format!("{first_name} {last_name}");
        if existing_names.contains(&name) {
            continue;
        }

        let description = format!("{name} is a renowned contract killer known for their precise execution and strategic thinking. With a reputation for being discreet and professional, they has successfully eliminated high-profile targets across the globe. They possesses exceptional marksmanship skills, specializing in long-range sniping and silent takedowns. Their calm demeanor and ability to adapt to any situation make them a valuable asset in complex operations. Equipped with cutting-edge technology and a meticulous approach, they ensures their targets meet their fate without leaving a trace. Clients seek their services for their most sensitive assignments, knowing that they delivers swift, undetectable results.");

        return Info { name, description }
    }
}
