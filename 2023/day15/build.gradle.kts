plugins {
    id("java")
}

group = "sk.aoc"
version = ""

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(platform("org.junit:junit-bom:5.9.1"))
    testImplementation("org.junit.jupiter:junit-jupiter")
}

tasks.withType<Jar> {
    manifest {
        attributes["Main-Class"] = "sk.aoc.Main"
    }
}

tasks.test {
    useJUnitPlatform()
}