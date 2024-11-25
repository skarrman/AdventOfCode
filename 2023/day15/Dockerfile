FROM gradle as BASE
WORKDIR /src
COPY ./ .
RUN gradle build

FROM openjdk:21
COPY --from=BASE /src/build/libs/day15.jar /day15.jar
COPY input.txt .
CMD java -jar /day15.jar
