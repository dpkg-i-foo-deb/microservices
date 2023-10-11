Feature: Register a new user

    Scenario: Successful registration 
        Given My email is mateo@mail.com and I don't exist in the system
        When I register myself with my email mateo@mail.com and password michi
        Then The system tells me I'm registered

    Scenario: User already exists
        Given I want to register myself again, already existing in the system
        When I try to register with my email mateo@mail.com and password michi
        Then The system tells me I cannot register myself twice