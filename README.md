
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
  Performance: 100%
  ```

### Network Performance API

- **Endpoint:** `/network_performance`
- **Description:** Retrieves the percentage of active validators in a committee.
- **HTTP Method:** GET
- **Path Parameter:**
  - none
- **Example Request:**
  ```bash
  curl -X GET http://localhost:8080/network_performance
  ```
- **Example Response:**
  ```bash
  Network Performance: 100%
  ```

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

   - For the Network Performance API, use the following command:
     ```bash
     curl -X GET http://localhost:8080/network_performance
     ```

3. The API will respond with the performance value as a percentage.
