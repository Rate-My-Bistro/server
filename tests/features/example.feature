Feature: Example feature

    Scenario: An example scenario
        Given I got the following list of menus
            | Name        | Date       |
            | test_menu_1 | 2121-01-19 |
            | test_menu_2 | 2121-01-19 |
        And I fetch the index route
        When I consider what I am doing
        Then I am interested in ATDD
        And we can implement rules with regex
