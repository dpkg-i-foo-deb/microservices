Feature disable

    Scenario: You cannot log in if your account is disabled
        Given my account mateo@mail.com is disabled
        When I try to log in into the system with my email mateo@mail.com and password michi
        Then I'm denied the access

    Scenario: You can disable your user account
        Given my account mateo@mail.com is active
        When I try to disable it using valid credentials
        Then my account is disabled successfully 

    Scenario: You cannot see disabled accounts
        Given my account mateo@mail.com is disabled
        When I check for the accounts on the system
        Then mateo@mail.com must not appear there