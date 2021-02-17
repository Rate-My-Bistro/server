use arangors::{Database, Connection, AqlQuery};
use arangors::client::surf::SurfClient;
use chrono::NaiveDate;
use cucumber::{Steps, t};
use crate::world::{PersistedMenu, MyWorld};
use crate::config::CucumberConfig;

async fn get_bistro_db(config: &CucumberConfig) -> Database<SurfClient> {
    let conn = Connection::establish_jwt(&config.database.url, &config.database.username, &config.database.password).await.unwrap();
    let db = conn.db(&config.database.name).await.unwrap();

    db
}

async fn create_menu(config: &CucumberConfig, name: String, date: String) -> Vec<String> {
    let aql = AqlQuery::builder()
        .query(r#"
             INSERT {
                name: @name,
                date: @date,
                price: 7.99,
                image: "http://some.image/image.png",
                low_kcal: true,
                optional_supplements: [{
                    name: "Pommes",
                    price: 10.99
                }, {
                    name: "Salat",
                    price: 2.85
                }],
                mandatory_supplements: [{ name: "Schranke", price: 4.99 }]
             }
             INTO menus
             LET inserted = NEW
             RETURN inserted._key
        "#)
        .bind_var("name", name)
        .bind_var("date", date)
        .build();

    get_bistro_db(config).await.aql_query(aql).await.unwrap()
}

async fn remove_unknown_menus(config: &CucumberConfig, menu_ids: Vec<String>, earliest: NaiveDate, latest: NaiveDate) -> Vec<String> {
    let aql = AqlQuery::builder()
        .query(r#"
             FOR m IN menus
             FILTER m._key NOT IN @ids
             FILTER m.date >= @earliest && m.date <= @latest
             REMOVE { _key: m._key } IN menus
             LET removed = OLD
             RETURN removed._key
        "#)
        .bind_var("ids", menu_ids)
        .bind_var("earliest", earliest.to_string())
        .bind_var("latest", latest.to_string())
        .build();

    get_bistro_db(config).await.aql_query(aql).await.unwrap()
}

pub fn steps() -> Steps<MyWorld> {
    let mut builder: Steps<MyWorld> = Steps::new();

    builder
        // .given_regex_async(
        //     r"^I got a menu (.*) served at (.*)$",
        //     t!(|world, matches, _step| {
        //             let insert_menus = create_menu(matches.first().unwrap().to_string(), matches.get(1).unwrap().to_string()).await;
        //
        //             assert_eq!(insert_menus.len(), 1);
        //
        //             world
        //     })
        // )
        .given_async(
            "I got the following list of menus",
            t!(|mut world: MyWorld, step| {
                    let table = step.table().unwrap().clone();

                    for row in table.rows.iter().skip(1) {
                        let name = row[0].to_owned();
                        let date = row[1].to_owned();
                        let inserted_menu_ids = create_menu(&world.config, name.clone(), date.clone()).await;

                        assert_eq!(inserted_menu_ids.len(), 1);

                        world.menus.push(PersistedMenu {
                            id: inserted_menu_ids.first().unwrap().to_string(),
                            name: name,
                            date: NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap()
                        })
                    }

                    world
                })
        )
        .given_async(
            "No other menus exist (between/on) the given dates",
            t!(|mut world, step| {
                    let menu_ids: Vec<String> =world.menus.iter().map(|menu| menu.id.clone()).collect();
                    let earliest: Option<NaiveDate> = world.menus.iter().map(|menu| menu.date).fold_first(|a,b| if a < b { a } else { b });
                    let latest: Option<NaiveDate> = world.menus.iter().map(|menu| menu.date).fold_first(|a,b| if a > b { a } else { b });
                    let mut dates: Vec<NaiveDate> = world.menus.iter().map(|menu| menu.date).collect();
                    dates.sort();

                    assert!(menu_ids.len() > 0, "Dont use this step if no menus were previously persisted");
                    assert!(earliest.is_some(), "All menus are lacking a serving date");
                    assert!(latest.is_some(), "All menus are lacking a serving date");

                    remove_unknown_menus(&world.config, menu_ids, earliest.unwrap(), latest.unwrap()).await;

                    world
                })
        );

    builder
}
