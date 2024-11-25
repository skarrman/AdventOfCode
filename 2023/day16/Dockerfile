FROM gradle as BASE
WORKDIR /app
COPY ./ .
RUN gradle build

FROM openjdk:21
COPY --from=BASE /app/build/libs/day16.jar /kotlinapp.jar
COPY input.txt .
CMD java -jar /kotlinapp.jar
