# OxiDish
Organize recipies using Rust.

This project is split up into "workspaces"--the goal being to separate code into modules.

The code responsible for transforming database entries into structs resides in the 'database' workspace. That workspace is configured as a library, that can be imported by other workspaces.
The 'web' workspace imports the database library, and transforms the Rust structs into JSON that it serves over HTTP. 
This might seem weird, but the idea was to potentially have different backends, like one that serves a REST API, and one that works more like old-school SSR with templating.

See [database/README.md](database/README.md) for info on the database schema and API
see [web/README.md](web/README.md) for a potential REST API


