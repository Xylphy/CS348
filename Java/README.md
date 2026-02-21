# CS348 - Operating System Java Activities

A collection of Java implementations for various Operating System activities and assignments, built with Gradle.

## Project Structure

```
cs348/
├── app/                          # Main application directory
│   └── src/main/java/
│       └── ParkingGarageSimulation/ # Example activity package
│           ├── Main.java
│           └── Car.java
├── build-logic/                  # Gradle convention plugins
└── gradle/                       # Gradle wrapper and configuration
```

## Prerequisites

- **Java 25** or later (configured in build configuration)
- **Gradle 9.3.1** (included via wrapper)

## Building the Project

### Build All Projects

```bash
./gradlew build
```

### Clean Build

```bash
./gradlew clean build
```

## Project Layout for Future Activities

To add new CS348 activities:

1. **Create a new package** under `app/src/main/java/`:

   ```
   app/src/main/java/
   ├── ParkingGarageSimulation/
   │   ├── Main.java
   │   └── Car.java
   ├── NewActivity/              # New activity package
   │   ├── Main.java
   │   └── [Other classes].java
   ```

2. **Add a new Gradle task** in `app/build.gradle.kts`:

   ```kotlin
   tasks.register<JavaExec>("ActivityName"){
       mainClass = "PackageName.Main"
       classpath = sourceSets["main"].runtimeClasspath
       standardInput = System.`in` # If activity requires user input from terminal
   }
   ```

3. **Run the new activity:**
   ```bash
   ./gradlew ActivityName
   ```

## Available Gradle Tasks

View all available tasks:

```bash
./gradlew tasks
```

Common tasks:

- `./gradlew build` - Build the entire project
- `./gradlew test` - Run all tests
- `./gradlew clean` - Delete build artifacts
- `./gradlew ParkingGarageSimulation` - Run Parking Garage Simulation

## Testing

Run the test suite:

```bash
./gradlew test
```

The project uses **JUnit Jupiter (JUnit 5)** for testing.

## Dependencies

The project includes:

- `org.apache.commons:commons-text:1.13.0` - Common text utilities

## Topics Covered

- ✅ **Semaphores & Mutexes** (Parking Garage Simulation)

## Notes

- The Gradle wrapper scripts (`gradlew` and `gradlew.bat`) are included for Linux/macOS and Windows respectively
- Java toolchain is automatically managed by Gradle
- All convention plugins are defined in the `build-logic` directory

---
