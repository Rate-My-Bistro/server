Feature: Menu API

    The Menu API is responsible for querying a set of menus
    by different parameters. Consequently, no write operations
    are expected inside here.

    Scenario: Fetch a menu by its id
        Given I got a menu 'Test Menu' served at 2121-01-19
        When I request the menu by its id
        Then I expect to receive this menu

    Scenario: Fetching menus by date range
        Given I got the following list of menus
            | Name        | Date       |
            | test_menu_1 | 2121-01-19 |
            | test_menu_2 | 2121-01-19 |
        And No other menus exist (between/on) the given dates
        When I consider what I am doing
        Then I am interested in ATDD
        And we can implement rules with regex
