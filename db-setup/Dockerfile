FROM postgres:latest
RUN apt-get update && apt-get install -y curl pgloader sqlite3
COPY allprintings_prep.sh /docker-entrypoint-initdb.d/allprintings_prep.sh