label start:
    jump chapter1_1

label chapter1_1:
    scene intro_1

    # chapter 1

    game_mechanic "card play poker"
    game_mechanic "card shop"
    game_mechanic "card play narrative characters"
    game_mechanic "card play narrative setting"

    llm_generate storyteller "{PROMPT} Setting of novel is: ```{SETTING}```. Characters are: ```{CHARACTERS}."
    game_mechanic "card play narrative conflict"

    llm_generate storyteller "{PROMPT} Story so far ```{STORY}```. Characters are: ```{CHARACTERS}. Continue this story with a conflict: ```{CONFLICT}```."
    game_mechanic "card play narrative plot twist"

    llm_generate storyteller "{PROMPT} Story so far ```{STORY}```. Characters are: ```{CHARACTERS}. Continue this story with a plot twist: ```{PLOT TWIST}```."

    game_mechanic "game over"