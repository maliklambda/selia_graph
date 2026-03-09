# Scy-fi

A query interpreter runtime for [Sypher](https://github.com/maliklambda/Sypher) queries.

## About
Scy-fi, a playfull fusion of the words "Science-fiction" or "Sci-fi" and "Sypher", provides the link between the parser and Selia-graph's storage engine.
It takes parsed queries and performs the corresponding calls to the storage engine's API.

## Query Interpretation


## Query Optimization


## Handling Runtime Errors
Runtime errors are errors that occur during the operation of a database query. 
They are distinct from errors that occur before or during the parsing of the query. 
Runtime errors can be of different natures, really they can be anything from IO-failures due to network issues to subqueries returning an invalid value. 
Fortunately, scy-fi handles all of those errors gracefully and returns the appropriate error with a message to the client.



