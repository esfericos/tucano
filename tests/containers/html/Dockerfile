FROM golang:1.22.3-alpine AS build
WORKDIR /app

COPY main.go .
RUN CGO_ENABLED=0 GOOS=linux go build -o html-app /app/main.go

FROM alpine:edge
WORKDIR /app

COPY tpl tpl
COPY --from=build /app/html-app .

# Set the timezone and install CA certificates
RUN apk --no-cache add ca-certificates tzdata

ENTRYPOINT ["/app/html-app"]
