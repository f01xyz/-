FROM projectserum/build:v0.31.1

WORKDIR /app
COPY . .

CMD ["anchor", "build"]
