label start:
    jump chapter1

label chapter1:
    scene intro_1

    "CHAPTER 1"

    "This world has lost all meaning."
    "All narratives are gone."
    "I am here to fix this world."

    scene intro_2

    "Play Poker Solitaire to earn score."
    "Each turn you place one card on any free place on table."
    "Your goal is to maximize score by making poker combinations."

    "You can spend your score to buy action cards."
    "There is 3 types of action cards that affect story narrative."
    "Setting, Plot Twist and Conflict"

    game_mechanic "card shop"
    game_mechanic "card play narrative characters"
    game_mechanic "card play narrative setting"

    llm_generate storyteller "{PROMPT} Setting of novel is: ```{SETTING}```. Characters are: ```{CHARACTERS}."
    game_mechanic "card play narrative conflict"

    game_mechanic "game over"
