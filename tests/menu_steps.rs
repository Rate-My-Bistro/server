use arangors::{Database, Connection, AqlQuery};
use arangors::client::surf::SurfClient;
use chrono::NaiveDate;
use cucumber::{Steps, t};
use crate::world::{PersistedMenu, MyWorld};

async fn get_bistro_db() -> Database<SurfClient> {
    let conn = Connection::establish_jwt("http://localhost:8529", "bistrouser", "bistropassword").await.unwrap();
    let db = conn.db("bistro").await.unwrap();

    db
}

async fn create_menu(name: String, date: String) -> Vec<String> {
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

    get_bistro_db().await.aql_query(aql).await.unwrap()
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
                        let inserted_menu_ids = create_menu(name.clone(), date.clone()).await;

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
            "no other menus exist (between/on) the given dates",
            t!(|mut world, step| {
                    let table = step.table().unwrap().clone();

                    for row in table.rows.iter() {
                        let name = row[0].to_owned();
                        let date = row[1].to_owned();
                        let insert_menus = create_menu(name.clone(), date.clone()).await;

                        assert_eq!(insert_menus.len(), 1);
                    }

                    world
                })
        );

    builder
}
