import base64
import io
import json
import uuid
import subprocess
import sqlite3
import uuid
import os

from flask import Flask, request, jsonify, send_file
from flask_cors import CORS
from together import Together
from PIL import Image
from ollama import Client
from langchain_core.pydantic_v1 import BaseModel, Field
from langchain_ollama import ChatOllama
from langchain_together import ChatTogether

USE_LOCAL_OLLAMA = False

TOGETHER_API_KEY = ""
TOGETHER_MODEL = "meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo"
TOGETHER_CLIENT = Together(api_key=TOGETHER_API_KEY)

OLLAMA_MODEL = "gemma3:12b"
OLLAMA_CLIENT = Client(
    host="http://192.168.88.242:11434",
)

WEBAPP = Flask(__name__)
CORS(WEBAPP)

# -----------
# API Handles
# -----------


@WEBAPP.route("/api/llm", methods=["POST"])
def llm():
    data = request.get_json()
    prompt = data.get("prompt", "")
    response = prompt_llm(prompt)
    return jsonify({"response": response}), 200


@WEBAPP.route("/api/image", methods=["GET"])
def generate_image():
    prompt = request.args.get("prompt", "")
    response = TOGETHER_CLIENT.images.generate(
        prompt=prompt,
        model="black-forest-labs/FLUX.1-schnell-Free",
        width=1792,
        height=1008,
        steps=4,
        n=1,
        response_format="b64_json",
        update_at="2024-11-20T09:27:43.295Z",
    )

    image_bytes = base64.b64decode(response.data[0].b64_json)
    image_stream = io.BytesIO(image_bytes)
    image = Image.open(image_stream)

    file_extension = image.format.lower()
    unique_filename = f"{uuid.uuid4()}.{file_extension}"

    image_path = f"images/{unique_filename}"
    image.save(image_path)

    return send_file(image_path, mimetype="image/jpeg")


@WEBAPP.route("/api/image/v2", methods=["GET"])
def generate_image_v2():
    prompt = request.args.get("prompt", "")
    return generate_image_from_prompt(prompt)


@WEBAPP.route("/api/image/v2/<image_name>", methods=["GET"])
def serve_image_by_hash(image_name):
    image_path = f"images/{image_name}"

    if not os.path.exists(image_path) or not os.path.isfile(image_path):
        return jsonify({"error": "Image not found"}), 404

    return send_file(image_path, mimetype="image/jpeg")


@WEBAPP.route("/api/nft/create", methods=["POST"])
def create_story_nft():
    data = request.get_json()

    scenario = data.get("scenario", "")
    owner = data.get("owner", "")

    command = ["node", "mint.js", "mint", owner]
    result = subprocess.run(command, capture_output=True, text=True).stdout.strip("\n")

    nft_id = int(result)

    metadata = create_nft_metadata_from_scenario(nft_id, scenario)
    metadata_json = json.dumps(metadata, indent=4)

    with sqlite3.connect("database.db") as conn:
        cursor = conn.cursor()

        cursor.execute(
            "INSERT INTO scenarios (nft_id, data) VALUES (?, ?)",
            (nft_id, metadata_json),
        )
        conn.commit()

        return jsonify({"nft_id": nft_id}), 200


@WEBAPP.route("/api/nft/<nft_id>", methods=["GET"])
def get_story_nft(nft_id):
    with sqlite3.connect("database.db") as conn:
        cursor = conn.cursor()

        # Query the scenario_nfts table for the given uuid
        query = "SELECT data FROM scenarios WHERE nft_id = ?"
        cursor.execute(query, (nft_id,))

        # Check if a record was found
        result = cursor.fetchone()
        if not result:
            return jsonify({"error": "NFT not found"}), 404

        nft_response = json.loads(result[0])
        return jsonify(nft_response), 200


# ---------
# LangChain
# ---------


class Scenario(BaseModel):
    summary: str = Field(description="Brief summary of scenario")
    title: str = Field(description="Title of scenario")
    poster: str = Field(description="Description of poster for movie based on scenario")


# OLLAMA_STRUCTURED_MODEL = ChatOllama(
#     model="gemma3", base_url="http://192.168.88.242:11434"
# )
# STRUCTURED_LLM = OLLAMA_STRUCTURED_MODEL.with_structured_output(Scenario)

TOGETHER_STRUCTURED_MODEL = ChatTogether(
    model=TOGETHER_MODEL,
    api_key=TOGETHER_API_KEY
)
STRUCTURED_LLM = TOGETHER_STRUCTURED_MODEL.with_structured_output(Scenario)

# -----------------
# Utility Functions
# -----------------


def create_nft_metadata_from_scenario(nft_id, scenario):
    clean_scenario = remove_hide_and_show(scenario)
    sce = STRUCTURED_LLM.invoke("Process this scenario " + clean_scenario)
    print(sce)
    image_response = generate_image_from_prompt(sce.poster)

    nft_response = {
        "name": f"#{nft_id}: {sce.title}",
        "description": sce.summary,
        "poster": sce.poster,
        "image": f"https://kakuseinosekainokokujoninarudaikinonisemono.space/api/image/v2/{image_response['hash']}",
        "scenario": scenario,
        "version": 2,
    }

    return nft_response


def remove_hide_and_show(renpy_string):
    """
    Removes lines starting with "show" or "hide" from a Ren'Py scenario string.

    Args:
      renpy_string: The Ren'Py scenario as a string.

    Returns:
      A new string with the "show" and "hide" lines removed.
    """

    lines = renpy_string.splitlines()
    filtered_lines = [
        line
        for line in lines
        if not (line.startswith("show") or line.startswith("hide"))
    ]
    return "\n".join(filtered_lines)


def refresh_story_nft(nft_id):

    with sqlite3.connect("database.db") as conn:
        cursor = conn.cursor()

        # Query the scenario_nfts table for the given uuid
        query = "SELECT data FROM scenarios WHERE nft_id = ?"
        cursor.execute(query, (nft_id,))

        # Check if a record was found
        result = cursor.fetchone()
        if not result:
            return None

        data_string = result[0]

        try:
            json.loads(data_string)
            return

        except:
            metadata = create_nft_metadata_from_scenario(nft_id, data_string)
            matadata_json = json.dumps(metadata, indent=4)

            cursor.execute(
                "UPDATE scenarios SET data = ? WHERE nft_id = ?",
                (matadata_json, nft_id),
            )
            conn.commit()


def generate_image_from_prompt(prompt):
    res = TOGETHER_CLIENT.images.generate(
        prompt=prompt,
        model="black-forest-labs/FLUX.1-schnell-Free",
        width=1792,
        height=1008,
        steps=4,
        n=1,
        response_format="b64_json",
        update_at="2024-11-20T09:27:43.295Z",
    )

    image_bytes = base64.b64decode(res.data[0].b64_json)
    image_stream = io.BytesIO(image_bytes)
    image = Image.open(image_stream)

    file_extension = image.format.lower()
    unique_filename = f"{uuid.uuid4()}.{file_extension}"

    image_path = f"images/{unique_filename}"
    image.save(image_path)

    return {"hash": unique_filename}


def prompt_llm(prompt):
    messages = [
        {"role": "user", "content": prompt},
    ]

    if not USE_LOCAL_OLLAMA:
        response = TOGETHER_CLIENT.chat.completions.create(
            model=TOGETHER_MODEL, messages=messages
        )
        response = response.choices[0].message.content
    else:
        response = OLLAMA_CLIENT.chat(model=OLLAMA_MODEL, messages=messages)
        response = response.message.content

    return response


# -----------
# Entry Point
# -----------

if __name__ == "__main__":
    WEBAPP.run(host="0.0.0.0", port=5000, debug=False)
