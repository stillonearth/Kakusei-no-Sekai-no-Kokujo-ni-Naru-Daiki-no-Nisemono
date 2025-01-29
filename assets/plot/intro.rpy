define me = Character("Me", color="#FFFFFF")

label start:
    jump chapter1_1

label chapter1_1:
    llm_generate storyteller "{PROMPT} Setting of novel is: ```{SETTING}```."
