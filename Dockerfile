FROM rust:1.78.0-slim-bullseye AS build

WORKDIR /usr/src/last-merge

COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=build /usr/src/last-merge/target/release/last-merge /
CMD [ "./last-merge" ]
