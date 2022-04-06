FROM node:alpine as builder
WORKDIR /app
COPY package.json yarn.lock /app/
RUN yarn install --pure-lockfile
COPY . .
RUN yarn run build

FROM nginx
COPY --from=builder /app/build /usr/share/nginx/html