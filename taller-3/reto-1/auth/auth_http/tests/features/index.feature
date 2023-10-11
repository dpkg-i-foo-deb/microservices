Feature: Index

    Scenario: The app is currently working
        Given It is possible to connect to the system
        When I send a request to the index route
        Then I get a successful response

    Senario: The app is currently down
        Given It is not possible to connect to the system
        When I send a request to the index route
        Then I get a gateway timeout error