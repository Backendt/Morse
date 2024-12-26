FROM nginx:latest

# TODO Temporary, mounted as volume for development
#COPY ./src/ /var/www/html

# Nginx configuration
COPY ./nginx.conf /etc/nginx/templates/default.conf.template

# TLS Certificate
COPY ./tls/ /etc/nginx/ssl/

EXPOSE 80
