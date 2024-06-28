FROM rust:alpine3.20
LABEL authors="tulio"
RUN apk update && apk upgrade #new
RUN apk add musl-dev libressl-dev pkgconfig #new
WORKDIR /opt/genercy
ADD https://github.com/tuliomgui/genercy.git .
RUN cargo build
RUN cp target/debug/genercy .
RUN rm -rf target

CMD ["./genercy"]