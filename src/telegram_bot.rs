use diesel::{QueryDsl, RunQueryDsl, TextExpressionMethods};
use crate::database::establish_connection;
use crate::models::Ingredient;
use crate::schema::ingredient::dsl::ingredient;
use crate::schema::ingredient::name;

pub fn run() {

    let connection = establish_connection();

    let results = ingredient
        .filter(name.like("Potato%"))
        .limit(5)
        .load::<Ingredient>(&connection)
        .expect("Error loading ingredients");

    println!("Displaying {} ingredients", results.len());
    for ingredient_item in results {
        println!("{}", ingredient_item.name);
    }
}
