Feature: Register a new user

    Scenario: Successful registration 
        Given I'm not registered in the system
        When I send a request to register myself, with valid credentials
        Then I'm registered succcessfully 