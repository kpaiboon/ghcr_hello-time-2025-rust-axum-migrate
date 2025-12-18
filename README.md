# Curl command to test web api rest
To run this web api use command:


    cargo run

## Get all
    curl --location --request GET 'http://localhost:8080/api/persons' \
    --header 'Content-Type: application/json'

## Insert new person
    curl --location 'http://localhost:8080/api/person' \
    --header 'Content-Type: application/json' \
    --data '{
        "id": 3,
        "name": "A Z",
        "age": 50,
        "date": "1974-02-26"
    }'

## Get new person
    curl --location --request GET 'http://localhost:8080/api/person/3' \
    --header 'Content-Type: application/json'

## Put existing person
    curl --location --request PUT 'http://localhost:8080/api/person' \
    --header 'Content-Type: application/json' \
    --data '{
        "id": 3,
        "name": "A Z",
        "age": 51,
        "date": "1974-07-15"
    }'


## Delete person
    curl --location --request DELETE 'http://localhost:8080/api/person/3' \
    --header 'Content-Type: application/json'

