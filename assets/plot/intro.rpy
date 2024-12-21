define me = Character("Me", color="#FFFFFF")

label start:
    jump chapter1_1

label chapter1_1:

    "PRESS SPACE"

    scene intro_2
    game_mechanic "card play poker"
    # game_mechanic "card shop"

    "This world has lost all meaning."
    "All narratives are gone."
    "I am here to fix this world."

    scene intro_4

    "Play Poker Solitaire to earn score."
    "Each turn you place one card on any free place on table."
    "Your goal is to maximize score by making poker combinations."

    "You can spend your score to buy action cards."
    "There is 3 types of action cards that affect story narrative."
    "Setting, Plot Twist and Conflict"

    game_mechanic "card play narrative setting"

    llm_generate storyteller "{PROMPT} Setting of novel is: ```{SETTING}```."

    game_mechanic "card play narrative conflict"

    llm_generate storyteller "{PROMPT} Story so far ```{STORY}```. Continue this story with a conflict: ```{CONFLICT}```."

    game_mechanic "card play narrative plot twist"

    llm_generate storyteller "{PROMPT} Story so far ```{STORY}```. Continue this story with a plot twist: ```{PLOT TWIST}```."

    llm_generate storyteller "{PROMPT} Story so far ```{STORY}```. Continue this story with a plot twist: ```{PLOT TWIST}```."

    llm_generate storyteller "{PROMPT} Story so far ```{STORY}```. Continue this story with a plot twist: ```{PLOT TWIST}```."
