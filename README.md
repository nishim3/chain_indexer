
# Rust API Documentation

This Rust application provides two APIs for retrieving performance-related data. Follow the instructions below to call these APIs.

## API Endpoints

### Validator Performance API

- **Endpoint:** `/validator/{validator_id}`
- **Description:** Retrieves the performance of a specific validator as a percentage.
- **HTTP Method:** GET
- **Path Parameter:**
  - `validator_id` (String): The ID of the validator.
- **Example Request:**
  ```bash
  curl -X GET http://localhost:8080/validator/25
  ```
- **Example Response:**
  ```bash
  Performance:100%
  ```

### Validator Committee Performance API

- **Endpoint:** `/validator_committee/{comm_id}`
- **Description:** Retrieves the performance of a validator committee as a percentage.
- **HTTP Method:** GET
- **Path Parameter:**
  - `comm_id` (String): The committee ID in the format "epoch_slot_index".
- **Example Request:**
  ```bash
  curl -X GET http://localhost:8080/validator_committee/2_100_3
  ```
- **Example Response:**
  ```bash
  Performance:100%
  ```
**Note** This API can take upto 10 minutes for execution because of the rate of API calls is limited.

## How to Use

1. Start the Rust application by running the following command:
   ```bash
   cargo run
   ```

2. Once the application is running, you can make API requests using `curl` or any other HTTP client of your choice.

   - For the Validator Performance API, use the following command:
     ```bash
     curl -X GET http://localhost:8080/validator/{validator_id}
     ```
     Replace `{validator_id}` with the ID of the validator you want to retrieve the performance for.

   - For the Validator Committee Performance API, use the following command:
     ```bash
     curl -X GET http://localhost:8080/validator_committee/{comm_id}
     ```
     Replace `{comm_id}` with the committee ID in the format "epoch_slot_index" you want to retrieve the performance for.

3. The API will respond with the performance value as a decimal between 0 and 1.

Make sure to replace `http://localhost:8080` with the appropriate hostname and port where your API is running.

