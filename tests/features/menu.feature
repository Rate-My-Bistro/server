Feature: Menu API

    The Menu API is responsible for querying a set of menus
    by using different parameters.

    Scenario: Fetch a menu by its id
        Given is the menu 'Test Menu' that is served at 2121-01-19
        When I request this menu by its id
        Then I expect to receive this menu

    Scenario: Fetching menus by date range
        Given is the following list of menus
            | Name                | Served at  |
            | Spaghetti Bolognese | 2121-01-19 |
            | Auberginen Moussaka | 2121-01-19 |
        And no other menus exist for the given dates (or in between)
        When I consider what I am doing
        Then I am interested in ATDD
        And we can implement rules with regex
