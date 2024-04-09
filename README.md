# LDAE-SIMS

A School Information Management System built with rust.

## How to run 

### Prerequisites

#### Install Rust and Cargo

You'll need to install the rust programming language and the cargo package manager, you can do it by following the instructions [here](https://www.rust-lang.org/tools/install).

#### Install `cargo-run-script`

Cargo-run-script is a command line utility tool that will help us run predefined scripts from the `Cargo.toml` file. To install it run the following command.

```sh
cargo install cargo-run-script

```

### Locally

The project can be run directly from the source code in your computer following the next steps:

#### Install `sqlx-cli`

Sqlx-cli is a command line utility tool used for migration related tasks, to install it run the following command:

```sh
cargo install sqlx-cli

```

#### PostgreSQL database server

This project database runs on a postgreSQL server which can be executed easily through docker. Supposing you have docker already installed you can run the following command:

```sh
cargo run-script run-database-service
```

Now, a postgreSQL server container should be running in the background available to accept incoming connections.

You can also run a postgreSQL server directly on your computer without docker but you'll need to make sure to set it up similarly to the instance defined in this project.

#### Run migrations

With `sqlx-cli` installed and the database instance running we can now run:

```sh
cargo run-script run-database-migrations
```

This should execute every migration from the project into the running instance of our database.

#### Run the API

Now you can simply run:

```sh
cargo run-script run-app-locally
```

And the API server should now be available at `0.0.0.0:3000`. You can now go to `0.0.0.0:3000/api/courses` in your web browser and see a JSON response with the courses available in the database.

### In a Docker container

The easiest way to run the project is through docker, the API image can be pulled and run in a container running the following command:

```sh
cargo run-script run-dockerized-app
```

This should pull the necessary images of the required services and run instances of them, this includes the database and api services.

Migrations will also be run and our database instance should be ready to go.

Finally, you can just go to `0.0.0.0:3000/api/courses` in your web browser and see a JSON response with the courses available in the database.

## Available Routes

This is a brief description of the available routes in the API.

### Programs

- [GET] Get All Programs: `0.0.0.0:3000/api/programs`
  - Description: Returns all available programs.
- [GET] Get Program by Code: `0.0.0.0:3000/api/program/{program_code}`
  - Description: Returns the program specified by its program code.
  - Example `program_code` path param: `isc-2018-p`.
- [POST] Create Program: `0.0.0.0:3000/api/program`
  - Description: Creates a new entry of a program based on the given body.
  - Example body:
    ```json
    {
      "display_name": "Ingeniería en Tecnologías de la Información",
      "duration": 9,
      "duration_type": "Semestral",
      "program_code": "iti-2019-p",
      "program_type": "Licenciatura"
    }
    ```
- [PATCH] Edit Program by Code: `0.0.0.0:3000/api/program/{program_code}`
  - Description: Updates an entry matching the given program code with the given body.
  - Example `program_code` path param: `iti-2019-p`.
  - Example body:
    ```json
    {
      "display_name": "Ingeniería en Tecnologías de la Información",
      "duration": 12,
      "duration_type": "Semestral",
      "program_type": "Licenciatura"
    }
    ```
- [DELETE] Delete Program by Code: `0.0.0.0:3000/api/program/{program_code}`
  - Description: Deletes an entry matching the given program code.
  - Example `program_code` path param: `iti-2019-p`.

### Specializations

- [GET] Get All Specializations: `0.0.0.0:3000/api/specializations`
  - Description: Returns all available specializations.
- [GET] Get Specialization by Code: `0.0.0.0:3000/api/specialization/{specialization_code}`
  - Description: Returns the specialization specified by its specialization code.
  - Example `specialization_code` path param: `dae-2020-s`.
- [POST] Create Specialization: `0.0.0.0:3000/api/specialization`
  - Description: Creates a new entry of a specialization based on the given body.
  - Example body:
    ```json
    {
      "display_name": "Ingeniería de Software",
      "program_code": "isc-2018-p",
      "specialization_code": "isw-2020-s"
    }
    ```
- [PATCH] Edit Specialization by Code: `0.0.0.0:3000/api/specialization/{specialization_code}`
  - Description: Updates an entry matching the given specialization code with the given body.
  - Example `specialization_code` path param: `isw-2020-s`.
  - Example body:
    ```json
    {
      "display_name": "Ingeniería de Software",
      "program_code": "isc-2018-p"
    }
    ```
- [DELETE] Delete Specialization by Code: `0.0.0.0:3000/api/specialization/{specialization_code}`
  - Description: Deletes an entry matching the given specialization code.
  - Example `specialization_code` path param: `isw-2020-s`.
- [GET] Get All Specializations by Program Code: `0.0.0.0:3000/api/specializations/byprogram/{program_code}`
  - Description: Returns all the specializations specified by their program code.
  - Example `program_code` path param: `isc-2018-p`.

### Courses

- [GET] Get All Courses: `0.0.0.0:3000/api/courses`
  - Description: Returns all available courses.
- [GET] Get Course by Code: `0.0.0.0:3000/api/course/{course_code}`
  - Description: Returns the course specified by its course code.
  - Example `course_code` path param: `lae-2020-c`.
- [POST] Create course: `0.0.0.0:3000/api/course`
  - Description: Creates a new entry of a course based on the given body.
  - Example body:
    ```json
    {
      "course_code": "ac3-2014-c",
      "display_name": "Arquitectura de Computadoras III",
      "program_code": "isc-2018-p",
      "specialization_code": null
    }
    ```
- [PATCH] Edit course by Code: `0.0.0.0:3000/api/course/{course_code}`
  - Description: Updates an entry matching the given course code with the given body.
  - Example `course_code` path param: `ac3-2014-c`.
  - Example body:
    ```json
    {
      "display_name": "Arquitectura de Computadoras 3",
      "specialization_code": "dae-2020-s"
    }
    ```
- [DELETE] Delete course by Code: `0.0.0.0:3000/api/course/{course_code}`
  - Description: Deletes an entry matching the given course code.
  - Example `course_code` path param: `ac3-2014-c`.
- [GET] Get All Courses by Program Code: `0.0.0.0:3000/api/course/{program_code}`
  - Description: Returns all the courses specified by their program code.
  - Example `program_code` path param: `isc-2018-p`.
- [GET] Get All Courses by Specialization Code: `0.0.0.0:3000/api/course/{specialization_code}`
  - Description: Returns all the courses specified by their specialization code.
  - Example `specialization_code` path param: `dae-2020-s`.