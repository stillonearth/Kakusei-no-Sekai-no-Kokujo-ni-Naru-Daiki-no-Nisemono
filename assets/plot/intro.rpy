define me = Character("Me", color="#FFFFFF")

label start:
    jump chapter1_1

label chapter1_1:

    game_mechanic "card play narrative setting"

    llm_generate storyteller "{PROMPT} Setting of novel is: ```{SETTING}```."

    game_mechanic "card play narrative conflict"

    llm_generate storyteller "{PROMPT} Story so far ```{STORY}```. Continue this story with a conflict: ```{CONFLICT}```."

    game_mechanic "card play narrative plot twist"

    llm_generate storyteller "{PROMPT} Story so far ```{STORY}```. Continue this story with a plot twist: ```{PLOT TWIST}```."

    llm_generate storyteller "{PROMPT} Story so far ```{STORY}```. Continue this story with a plot twist: ```{PLOT TWIST}```."

    llm_generate storyteller "{PROMPT} Story so far ```{STORY}```. Continue this story with a plot twist: ```{PLOT TWIST}```."

