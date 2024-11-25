FROM gcc:4.9 as BASE
WORKDIR /src
COPY ./ .
RUN gcc -static -std=c1x -O3 -o aoc aoc.c

FROM alpine:latest
COPY --from=BASE /src/aoc /aoc
COPY input.txt .
CMD ["./aoc"]
