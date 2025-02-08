FROM rust:alpine3.20
LABEL authors="tulio"
RUN apk add --no-cache shadow
RUN apk update && apk upgrade #new
RUN apk add git musl-dev libressl-dev pkgconfig #new
WORKDIR /opt/genercy
RUN groupadd -g 1000 tulio && \
    useradd -u 1000 -g 1000 -m tulio
USER tulio
ADD . .

CMD [""]
