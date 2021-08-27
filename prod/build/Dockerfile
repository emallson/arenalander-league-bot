FROM emallson/gladiator-bot-discord:latest
RUN apt update --allow-releaseinfo-change && apt-get install -y nginx certbot python-certbot-nginx cron && rm /etc/nginx/sites-enabled/default
CMD service nginx start && service cron start && cargo run --release --bin bot --locked
