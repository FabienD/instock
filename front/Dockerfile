## Stage 1 - Builder ##
FROM node:18 AS builder

WORKDIR /app

COPY . ./

RUN npm install && \
    npm run build

## Stage 2 - Web server ##
FROM nginx:stable

WORKDIR /usr/share/nginx/html

RUN rm -rf ./*

COPY --from=builder /app/dist/ .