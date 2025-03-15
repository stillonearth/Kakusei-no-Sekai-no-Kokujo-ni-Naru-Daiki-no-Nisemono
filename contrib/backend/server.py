from flask import Flask, request, jsonify, send_file
from together import Together
from PIL import Image

import base64
import io
import uuid
import subprocess
import sqlite3
import uuid

app = Flask(__name__)


API_KEY = ""
LLM_MODEL = "deepseek-ai/DeepSeek-R1"

TOGETHER_CLIENT = Together(api_key=API_KEY)


@app.route("/api/llm", methods=["POST"])
def llm():
    data = request.get_json()
    prompt = data.get("prompt", "")

    messages = [
        {"role": "user", "content": prompt},
    ]

    response = TOGETHER_CLIENT.chat.completions.create(
        model=LLM_MODEL, messages=messages
    )
    response = response.choices[0].message.content

    return jsonify({"response": response}), 200


@app.route("/api/image", methods=["GET"])
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

    return send_file(image_path, mimetype="image/png")


def persist_string_to_db(data):
    # Create a connection to the SQLite database
    with sqlite3.connect("example.db") as conn:
        cursor = conn.cursor()

        # Create table if not exists
        cursor.execute(
            """CREATE TABLE IF NOT EXISTS data_storage (uuid_str TEXT PRIMARY KEY, data TEXT)"""
        )

        # Generate random UUID
        uuid_key = str(uuid.uuid4())

        # Insert data into the table
        cursor.execute("INSERT INTO data_storage VALUES (?, ?)", (uuid_key, data))

        # Commit the transaction
        conn.commit()

        # Retrieve and print the inserted data for verification
        cursor.execute("SELECT * FROM data_storage WHERE uuid_str = ?", (uuid_key,))
        result = cursor.fetchone()
        if result:
            print("Data persisted successfully:")
            print(f"UUID: {result[0]}")
            print(f"Data: {result[1]}")

    return uuid_key


@app.route("/api/nft", methods=["POST"])
def persist_story():
    data = request.get_json()

    scenario = data.get("scenario", "")
    owner = data.get("owner", "")

    uuid = persist_string_to_db(scenario)
    link = "http://kakuseinosekainokokujoninarudaikinonisemono.space/nft/" + uuid

    command = ["node", "mint.js", "mint", owner, link]
    subprocess.run(command)

    return jsonify({"nft_id": 44}), 200


if __name__ == "__main__":
    app.run(host="0.0.0.0", port=5000, debug=False)
