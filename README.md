# Beacon chain slots reader.

### Fetches slots from beacon node & dumbs it to postgres db. API cal. average attestation percent over db slots & exposes it.
- There are two bin files.
    - Fetcher
    - Rest Api


## Running Fetcher

- Create env files & set `BEACON_CHAIN_URL` & `POSTGRES_URL`.
- Run with `cargo run --bin fetcher auto`


## Running API
 
- Env file with atleast `POSTGRES_URL`.
- Run with `cargo run --bin api`


## Interacting with api endpoints.

### With curl 

```
curl localhost:8082/attestation_percentage
```

### With postman

```
 Get -  localhost:8082/attestation_percentage
```

