from flask import Flask, request, jsonify, send_file
from together import Together
from PIL import Image

import base64
import io
import uuid

app = Flask(__name__)


API_KEY = ""
LLM_MODEL = "meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo"

TOGETHER_CLIENT = Together(api_key=API_KEY)


@app.route("/api/llm", methods=["POST"])
def chat():
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


if __name__ == "__main__":
    app.run(host="0.0.0.0", port=5000, debug=False)
