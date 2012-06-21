Feature: relative searching

  Scenario: find a literal occurance
    Given a file named "binary.bin" with:
      """
      ABC
      """
    When I run `stromoy -f binary.bin -s "ABC"`
    Then the output should contain:
       """
       65 66 67 .. .. .. .. ..  A B C . . . . .
       """
