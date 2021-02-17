Feature: Menu API

    Scenario: Fetching menus by date range
        Given I got the following list of menus
            | Name        | Date       |
            | test_menu_1 | 2121-01-19 |
            | test_menu_2 | 2121-01-19 |
        And No other menus exist (between/on) the given dates
        When I consider what I am doing
        Then I am interested in ATDD
        And we can implement rules with regex
