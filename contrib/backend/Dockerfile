FROM nikolaik/python-nodejs:latest

WORKDIR /app
COPY package.json .
RUN npm install

COPY requirements.txt .

RUN pip install --break-system-packages -r requirements.txt
RUN mkdir images

COPY . .
RUN ln -s /usr/bin/node .
RUN ls .
CMD ["python", "server.py"]
