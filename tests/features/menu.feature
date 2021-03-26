Feature: Menu API

    The Menu API is responsible for querying a set of menus
    by using different parameters.

    Scenario: Fetch a menu by its id
        Given is the menu 'Test Menu' that is served at 2121-01-16
        When I request this menu by its id
        Then I expect to receive this menu

    Scenario: Fetch an unknown menu by id
        Given is a menu not known to the system
        When I request this menu by its id
        Then I expect to receive a 404 code in response

    Scenario: Fetch menus inside a date range
        Given is the following list of menus
            | Name                | Served at  |
            | Spaghetti Bolognese | 2121-02-18 |
            | Eggplant Moussaka   | 2121-02-19 |
            | Butter Chicken      | 2121-02-20 |
        And no other menus exist for the given dates (or in between)
        When I request menus between 2121-02-10 and 2121-02-28
        Then I expect to receive all menus served between these two dates

    Scenario: Fetch menus that are served on earliest boundary
        Given is the following list of menus
            | Name             | Served at  |
            | Pommes Schranke  | 2121-03-21 |
            | Stuffed Cabbage  | 2121-03-22 |
            | Mexican Sandwich | 2121-03-23 |
        And no other menus exist for the given dates (or in between)
        When I request menus between 2121-03-22 and 2121-03-28
        Then I expect to receive all menus served between these two dates

    Scenario: Fetch menus that are served on latest boundary
        Given is the following list of menus
            | Name          | Served at  |
            | Bun Bo Nam Bo | 2121-04-24 |
            | Spring Rolls  | 2121-04-25 |
            | Miso Soup     | 2121-04-26 |
        And no other menus exist for the given dates (or in between)
        When I request menus between 2121-04-01 and 2121-04-25
        Then I expect to receive all menus served between these two dates

    Scenario: Fetch no menus if none match the queried date range
        Given is the following list of menus
            | Name            | Served at  |
            | Sarma           | 2121-05-27 |
            | Pizza Magherita | 2121-05-28 |
        And no other menus exist for the given dates (or in between)
        When I request menus between 2121-05-01 and 2121-05-25
        Then I expect to receive no menus



