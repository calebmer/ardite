# Overview
The goal of Ardite is to be a no-SQL and immutable database. Whose goal is to power fast front end applications.

## no-SQL
Relationships can still be expressed in Ardite, however this does not make it a traditional relational database like its SQL predecessors.

The goal is that Ardite can replicate a website URL structure in the way its data is structured. Another advantage of being no-SQL is providing features from the rich JSON toolset including JSON Pointer, JSON Schema, JSON Patch, and more.

## Immutable
Data is not stored in a traditional mutable database way, but rather data is stored in “patches” or immutable facts.

The advantage of this immutable approach is that the database is easier to audit, easier to replicate, and can be time traveled. Even if a user deletes some data, it is still a true fact that at some point the user uploaded that data.

When the user wants to read the database‘s data then it is in a **“reduced”** state. That is all patches have been applied in such a way where the resultant object is all of Ardite’s data up to that point in time.

## Further Reading
- [Turning the database inside out](http://www.confluent.io/blog/turning-the-database-inside-out-with-apache-samza/). A must read for understanding Ardite’s architecture.
