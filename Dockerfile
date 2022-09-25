# syntax=docker/dockerfile:1
FROM fedora:latest

RUN mkdir ukrgeo
WORKDIR /ukrgeo/
COPY . .

RUN dnf install rust cargo npm sqlite sqlite-devel -y
WORKDIR /ukrgeo/frontend
RUN npm install
RUN npm run build
WORKDIR /ukrgeo/

ENV PROD="TRUE"

RUN mkdir target

# COPY db.sqlite /etc/ukrgeo/db.sqlite
# ENV DATABASE_URL="/etc/ukrgeo/db.sqlite"
ENV ROCKET_DATABASES={sqlite_locs={url="/etc/ukrgeo/db.sqlite"}}
RUN cargo build
CMD cp -n /ukrgeo/db.sql* /etc/ukrgeo/db.sqlite; /ukrgeo/target/debug/backend
