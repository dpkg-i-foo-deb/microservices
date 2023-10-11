Feature login

    Scenario: Login successful
        Given I'm registered in the system as mateo@mail.com
        When I try to login using my email mateo@mail.com and password michi
        Then I log in successfully

    Scenario: Wrong credentials
        Given I'm registered in the system as mateo@mail.com
        When I try to login using my email mateo@mail.com and password none
        Then I'm denied the access

    Scenario: User not registered
        Given I'm not registered in the system
        When I try to login using my email mateo@mail.com and password michi
        Then I'm denied the access