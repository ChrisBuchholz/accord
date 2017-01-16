# Rocket + Accord example

This is example builds a simple REST API that enables a route / which
takes a JSON that must contain a name, email and age values. Some rules
for the JSON data has been set up and are enforced by Rocket and Accord in
collaboration. They are:

* name: must be a string between 1 and 64 characters
* email: must be a string between 5 and 64 characters and must contain a
. and a @
* age: must be tween 12 and 127

You can run the webserver with:

```
cargo run
```

When the webserver is running, you can send a proper formed request to it with:

```
curl -i -H "Accept: application/json" -H "Content-Type: application/json" -X POST -d '{"name": "Test Test", "email": "test@test.test", "age": 25}' http://localhost:8000
```

You can also try sending a request where the age is below 12, there is only
an empty string for a name and the email does not contain a @. You will see
that the server responds with a 422 Unprocessable Entity and a JSON body
that explains exactly what is wrong:

```
curl -i -H "Accept: application/json" -H "Content-Type: application/json" -X POST -d '{"name": "", "email": "testtesttest", "age": 9}' http://localhost:8000
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
having to deal with the variables like *12* and *@*, and can easily be parsed
and shown to the user submitting the data in order to aid in fixing the
problems.
