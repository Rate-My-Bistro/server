Feature: Chore and Basics of the API

    The API server requires some convenience routes
    for a better user experience. These functionalities
    are tested here.

    Scenario: Get information on unknown routes
        Given is an up and running api server
        When I request an unknown route
        Then I expect to receive a 404 code in response
        And I expect to receive a list of possible routes

    Scenario: Get information on index route
        Given is an up and running api server
        When I request the index route
        Then I expect to receive a list of possible routes
