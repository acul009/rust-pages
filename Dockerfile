FROM rust:1.95-bookworm AS build

WORKDIR /app
COPY . .
RUN cargo run --release -p site -- build

FROM nginx:alpine AS production
RUN rm /etc/nginx/conf.d/*
COPY --from=build /app/build /var/www/it-rahn.de
COPY ./nginx.conf /etc/nginx/conf.d/it-rahn.de.conf
RUN apk update \
    && apk upgrade --no-cache \
    && nginx -t
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
