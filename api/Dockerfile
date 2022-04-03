FROM rust:1.59 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM alpine as runner
COPY --from=builder /usr/local/cargo/bin/api /usr/local/bin/api
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["api"]