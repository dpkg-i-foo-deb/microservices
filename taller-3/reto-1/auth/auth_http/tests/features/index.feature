Feature: Index

    Scenario: Querying the API Index
        Given I want to visit the / route
        When I send a request to the index route
        Then I get a successful response