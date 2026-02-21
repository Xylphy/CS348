# CS348 Operating Systems — Java Activities

This repository contains Java activity implementations for **CS348 Operating Systems**.
Each activity is organized in its own package and can be run via a dedicated Gradle task.

## Requirements

- Java JDK 25 (automatically managed via Gradle toolchain)
- Gradle (wrapper included — no installation needed)

## Build

```sh
./gradlew build
```

## Run an Activity

Each activity has its own Gradle task. Use the task name corresponding to the activity you want to run.

```sh
./gradlew <ActivityName>
```

### Available Activities

| Activity | Package | Description |
|---|---|---|
| Parking Garage Simulation | `ParkingGarageSimulation` | Simulates a parking garage using semaphores to control concurrent access to limited parking slots. |

#### Example

```sh
./gradlew ParkingGarageSimulation
```

## Project Structure

```
app/
└── src/
    └── main/
        └── java/
            └── <ActivityPackage>/   # One package per activity
                └── Main.java
app/build.gradle.kts                 # Gradle task definitions per activity
build-logic/                         # Shared Gradle convention plugins
gradle/                              # Gradle wrapper and version catalog
```

## Adding a New Activity

1. Create a new package under `app/src/main/java/<ActivityName>/`.
2. Add a `Main.java` with your `main` method inside the package.
3. Register a new `JavaExec` task in [app/build.gradle.kts](app/build.gradle.kts):

```kts
tasks.register<JavaExec>("ActivityName") {
    mainClass.set("ActivityName.Main")
    classpath = sourceSets["main"].runtimeClasspath
    standardInput = System.`in`
}
```

4. Run it with:

```sh
./gradlew ActivityName --console=plain # Optional: --console=plain for cleaner output
```