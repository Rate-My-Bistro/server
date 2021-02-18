Feature: Menu API

    The Menu API is responsible for querying a set of menus
    by using different parameters.

    Scenario: Fetch a menu by its id
        Given is the menu 'Test Menu' that is served at 2121-01-19
        When I request this menu by its id
        Then I expect to receive this menu

    Scenario: Fetch menus inside a date range
        Given is the following list of menus
            | Name                | Served at  |
            | Spaghetti Bolognese | 2121-01-19 |
            | Auberginen Moussaka | 2121-01-19 |
        And no other menus exist for the given dates (or in between)
        When I request menus between 2121-01-10 and 2121-01-31
        Then I expect to receive all given menus
