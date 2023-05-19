use dtdb::data::models::season::Season;

const FIRST_YEAR: u16 = 1947;

fn main() {
    let range = 0u16..200u16;
    let count = range.len();
    let initial_seasons =
        range
            .map(|i|
                Season::create(
                    (i + 1).try_into().unwrap(),
                    FIRST_YEAR + i,
                    FIRST_YEAR + i + 1
                ).unwrap()
            )
            .enumerate();
    
    println!(r#"INSERT INTO seasons ("id", "season_number", "start_year", "end_year")"#);
    println!(r#"VALUES"#);
    for (idx, season) in initial_seasons {
        println!(
            r#"  ('{}', '{}', '{}', '{}'){}"#,
            season.id.to_string(),
            season.season_number,
            season.start_year,
            season.end_year,
            if idx < count - 1 {
                ","
            } else {
                ";"
            }
        );
    }
}