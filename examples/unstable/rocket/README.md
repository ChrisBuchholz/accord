# Rocket + Accord example

This example builds a simple REST API using [Rocket] and Accord enabling
a route `/` that accepts JSON content that must contain a name, email and
age value. Some rules for the JSON has been set up and are enforced
by Rocket and Accord in collaboration, Rocket doing the type-checking and
Accord the validation. The rules are:

[Rocket]: https://rocket.rs

* name: must be a string between **1** and **64** characters
* email: must be a string between **5** and **64** characters and must contain a **.** and a **@**
* age: must be tween **12** and **127**

You can run the webserver with:

```
cargo run
```

When the webserver is running, you can send a proper formed request to it with:

```
curl -i -H "Accept: application/json" -H "Content-Type: application/json" -X POST -d '{"name": "Test Test", "email": "test@test.test", "age": 25}' http://localhost:8000
```

You can also try sending a request where there is only an empty string
for a name, the email is neither long enough nor does it contain **.**
and **@**. You will see that the server responds with a
422 Unprocessable Entity and a JSON body that explains exactly what is wrong.
Send the request:

```
curl -i -H "Accept: application/json" -H "Content-Type: application/json" -X POST -d '{"name": "", "email": "test", "age": 9}' http://localhost:8000
```

This is the returned JSON body:

```
[
    {
        "tag": "name",
        "invalids": [
            {
                "msg": "Must not be less than %1.",
                "args": ["1"]
            }
        ]
    }, {
        "tag": "email",
        "invalids": [
            {
                "msg": "Must not be less than %1.",
                "args": ["5"]
            {
                "msg": "Must contain %1.",
                "args": ["@"]
            },
            {
                "msg": "Must contain %1.",
                "args": ["."]
            }
        ]
    }, {
        "tag": "age",
        "invalids": [
            {
                "msg": "Must be in the range %1..%2.",
                "args": ["12","127"]
            }
        ]
    }
]
```

As you can see, this JSON body is easily translatable to other languages without
having to deal with the variables like **12** and **@**, and can easily be parsed
and shown to the user submitting the data in order to aid in fixing the
problems.
