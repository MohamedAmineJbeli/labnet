## Hint 1
The application takes your input and places it directly inside a SQL query. Think about how you can terminate the original string and append a new query.

## Hint 2
In SQL, single quotes denote string literals. If you close the opening quote in the query, the rest of your input will be interpreted as SQL commands. Try entering a single quote in the input field and observe the error.

## Hint 3
You need to extract data from a different table (`users`) while matching the number of columns the original query expects. Research the `UNION` operator.

## Hint 4
Make sure to comment out the trailing quote and semicolon left over from the original query using `-- -`.