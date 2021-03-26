use arangors::{Database, Connection, AqlQuery, ClientError};
use arangors::client::surf::SurfClient;
use chrono::NaiveDate;
use cucumber_rust::{given, when, then, gherkin};
use restson::{Error, RestClient, RestPath};

use crate::config::CucumberConfig;
use crate::world::{BistroWorld, DateRange, FailureResponse, PersistedMenu, PersistedMenus};

/// RestClient implementation of the GET /menu/<id> route
///
impl RestPath<String> for PersistedMenu {
    fn get_path(param: String) -> Result<String, Error> { Ok(format!("menus/{}", param)) }
}

/// RestClient Implementation of the GET /menu?from=xxx&to=xxx route
///
impl RestPath<()> for PersistedMenus {
    fn get_path(_: ()) -> Result<String, Error> { Ok(String::from("menus")) }
}


/// Either returns a new connection to arango db for the given
/// configuration or it forwards an error if no connection could
/// be established
///
/// This method is used as a testing adapter in order to manage
/// the persistence for menus
///
async fn get_bistro_db(config: &CucumberConfig) -> Result<Database<SurfClient>, ClientError> {
    Connection::
    establish_jwt(
        &config.database.url,
        &config.database.username,
        &config.database.password,
    ).await?
        .db(&config.database.name).await
}

/// Creates a new menu with a given name and serving date (e.g. 2020-01-19).
///
/// If any error occurs during connection establishment or insert operation,
/// this error will be returned instead.
///
async fn create_menu(config: &CucumberConfig, name: String, date: String) -> Result<String, ClientError> {
    let aql = AqlQuery::builder()
        .query(r#"
             INSERT {
                name: @name,
                date: @date,
                price: 7.99,
                lowKcal: true,
                image: "http://some.image/image.png",
                mandatorySupplements: [{ name: "Schranke", price: 4.99 }],
                optionalSupplements: [
                    { name: "Pommes", price: 10.99 },
                    { name: "Salat", price: 2.85 }
                ]
             }
             INTO menus
             LET inserted = NEW
             RETURN inserted._key
        "#)
        .bind_var("name", name)
        .bind_var("date", date)
        .build();

    let query_result: Result<Vec<String>, _> = get_bistro_db(config).await?
        .aql_query(aql).await;

    match query_result {
        Ok(menu_ids) => Ok(menu_ids.first().unwrap().to_string()),
        Err(e) => Err(e)
    }
}

/// Searches for menus having a serving date between (inclusive) given earliest and latest date.
/// All found menus not having one of the given ids will be removed.
///
/// In case an error occurs during connection establishment or delete operation,
/// this error will be returned instead.
///
async fn remove_unknown_menus(config: &CucumberConfig, menu_ids: Vec<String>, earliest: NaiveDate, latest: NaiveDate) -> Result<Vec<String>, ClientError> {
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

    get_bistro_db(config).await?
        .aql_query(aql).await
}

#[given("is a menu not known to the system")]
fn unknown_menus_are_served(world: &mut BistroWorld) {
    world.expected_menus.push(PersistedMenu {
        name: String::from("Unknown menu"),
        id: String::from("this-is-an-unknown-id"),
        date: NaiveDate::from_ymd(2021, 1, 10),
    });
}

#[given(regex = r"^is the menu '(.*)' that is served at (.*)$")]
async fn a_menu_is_served(world: &mut BistroWorld, name: String, served_at: String) {
    let insert_menu_result = create_menu(&world.config, name.clone(), served_at.clone()).await;

    assert!(insert_menu_result.is_ok(), format!("The new menu could not be created: {:?}", insert_menu_result));

    world.expected_menus.push(PersistedMenu {
        name,
        id: insert_menu_result.unwrap(),
        date: NaiveDate::parse_from_str(&served_at, "%Y-%m-%d").unwrap(),
    });
}

#[given("is the following list of menus")]
async fn a_list_of_menus_is_served(world: &mut BistroWorld, step: &gherkin::Step) {
    let rows = &step.table().unwrap().rows;
    for row in rows.iter().skip(1) {
        let name = row[0].to_owned();
        let date = row[1].to_owned();

        let inserted_menu_id = create_menu(&world.config, name.clone(), date.clone()).await;
        assert!(inserted_menu_id.is_ok(), format!("The new menu '{}' could not be created", name));

        world.expected_menus.push(PersistedMenu {
            name,
            id: inserted_menu_id.unwrap(),
            date: NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap(),
        });
    }

    assert_eq!(world.expected_menus.len(), rows.len() - 1, "The menu table is lacking entries");
}

#[given("no other menus exist for the given dates (or in between)")]
async fn no_other_menu_exist(world: &mut BistroWorld) {
    let menu_ids: Vec<String> = world.expected_menus.iter().map(|menu| menu.id.clone()).collect();
    let earliest: Option<NaiveDate> = world.expected_menus.iter().map(|menu| menu.date).reduce(|a, b| if a < b { a } else { b });
    let latest: Option<NaiveDate> = world.expected_menus.iter().map(|menu| menu.date).reduce(|a, b| if a > b { a } else { b });

    assert!(menu_ids.len() > 0, "Dont use this step if no menus were previously persisted");
    assert!(earliest.is_some() && latest.is_some(), "All menus are lacking a serving date");

    let removed_ids = remove_unknown_menus(&world.config, menu_ids, earliest.unwrap(), latest.unwrap()).await;
    assert!(removed_ids.is_ok(), "A new menu should have been created");
}

#[when("I request this menu by its id")]
async fn request_menu_by_id(world: &mut BistroWorld) {
    assert_eq!(world.expected_menus.len(), 1, "There are multiple menus known to the context");

    let menu = world.expected_menus.first().unwrap();
    let mut client = RestClient::new("http://localhost:8001").unwrap();
    let menu_result: Result<PersistedMenu, Error> = client.get(menu.id.clone());

    match menu_result {
        Ok(menus) => world.actual_menus.push(menus),
        Err(Error::HttpError(status_code, _)) => {
            world.expected_failure = Some(FailureResponse { status_code });
        },
        Err(_) => {}
    }
}

#[when(regex = "I request menus between (.*) and (.*)")]
async fn request_menu_by_date_range(world: &mut BistroWorld, earliest_serving_date: String, latest_serving_date: String) {
    let from = NaiveDate::parse_from_str(&*earliest_serving_date, "%Y-%m-%d").expect("Could not parse 'from' date");
    let to = NaiveDate::parse_from_str(&*latest_serving_date, "%Y-%m-%d").expect("Could not parse 'to' date");

    let query = vec![("from", &*earliest_serving_date), ("to", &*latest_serving_date)];
    let mut client = RestClient::new("http://localhost:8001").unwrap();
    let menus_result: Result<PersistedMenus, Error> = client.get_with((), &query);

    assert!(menus_result.is_ok(), format!("Failed to request menus: {:?}", menus_result));

    world.actual_menus.extend(menus_result.unwrap().0);
    world.served_range = Some(DateRange { from, to });
}

#[then("I expect to receive this menu")]
fn single_menu_is_present(world: &mut BistroWorld) {
    assert_eq!(world.actual_menus.len(), world.expected_menus.len(), "The amount of expected menus and received menus differs");
    assert_eq!(world.actual_menus.len(), 1, "Only a single menu is expected");

    let expected_menu = world.expected_menus.first().unwrap();
    let actual_menu = world.actual_menus.first().unwrap();

    assert_eq!(actual_menu, expected_menu, "Actual and expected menu are not the same");
}

#[then("I expect to receive all menus served between these two dates")]
fn multiple_menus_are_present(world: &mut BistroWorld) {
    assert!(world.served_range.is_some(), "No range was specified");

    let served_range = world.served_range.clone().unwrap();
    let expected_menus = world.expected_menus.iter()
        .filter(|menu| menu.date.ge(&served_range.from) && menu.date.le(&served_range.to))
        .cloned()
        .collect::<Vec<PersistedMenu>>();

    assert!(expected_menus.len() > 0);

    assert_eq!(world.actual_menus.len(), expected_menus.len(), "The amount of expected menus and received menus differs");
    assert_eq!(world.actual_menus, expected_menus, "Actual and expected menu are not the same");
}

#[then("I expect to receive no menus")]
fn no_menus_are_present(world: &mut BistroWorld) {
    assert_eq!(world.actual_menus.len(), 0, "The amount of expected menus and received menus differs");
}

#[then(regex = r"I expect to receive a (\d+) code in response")]
fn got_failure_response(world: &mut BistroWorld, status_code: u16) {
    assert!(world.expected_failure.is_some(), "Expected the http response to report a failure");

    let expected_failure = world.expected_failure.clone().unwrap();
    assert_eq!(expected_failure.status_code, status_code, "Expected and actual status code don't match");
}
