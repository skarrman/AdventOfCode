FROM gradle:alpine as BASE
WORKDIR /app
COPY ./ .
RUN gradle build

FROM openjdk:18-alpine
COPY --from=BASE /app/build/libs/kotlin_cygni_aoc-1.0.0.jar /kotlin_cygni_aoc.jar
COPY input.txt .
CMD java -jar /kotlin_cygni_aoc.jar
