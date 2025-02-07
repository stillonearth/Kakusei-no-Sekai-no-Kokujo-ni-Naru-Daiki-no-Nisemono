FROM python:3.12
WORKDIR /app
COPY requirements.txt .
RUN pip install -r requirements.txt
RUN mkdir images
COPY . .
CMD ["python", "server.py"]