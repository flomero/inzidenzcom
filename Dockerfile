From amd64/rust:alpine as builder
COPY . app
WORKDIR /app
RUN apk add npm g++
RUN ls -l
RUN npm install
RUN cargo build --release
RUN mv target/release/inzidenz .
RUN strip inzidenz

From amd64/alpine as runner
WORKDIR /root/
COPY --from=builder /app/static ./static/
COPY --from=builder /app/templates ./templates/
COPY --from=builder /app/inzidenz ./inzidenz
EXPOSE 8000
ENTRYPOINT ["./inzidenz"]
