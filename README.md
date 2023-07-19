
```markdown
# Rust API Documentation

This Rust application provides three APIs for retrieving performance-related data. Follow the instructions below to call these APIs.

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

### Validator Committee Performance API

- **Endpoint:** `/validator_committee/{comm_id}`
- **Description:** Retrieves the percentage of active validators in a committee.
- **HTTP Method:** GET
- **Path Parameter:**
  - `committee` (String): The index of the committee in the latest slot.
- **Example Request:**
  ```bash
  curl -X GET http://localhost:8080/validator_committee/32
  ```
- **Example Response:**
  ```bash
  Percentage of active validators is: 100%

  epoch: 215973
  slot: 6911173
  index: 32
  ```

### Validator Network Performance API

- **Endpoint:** `/validator_network`
- **Description:** Retrieves the overall network performance of the latest slot as a percentage.
- **HTTP Method:** GET
- **Example Request:**
  ```bash
  curl -X GET http://localhost:8080/validator_network
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

   - For the Validator Committee Performance API, use the following command:
     ```bash
     curl -X GET http://localhost:8080/validator_committee/{comm_id}
     ```
     Replace `{comm_id}` with the committee ID in the format "epoch_slot_index" you want to retrieve the performance for.

   - For the Validator Network Performance API, use the following command:
     ```bash
     curl -X GET http://localhost:8080/validator_network
     ```

3. The API will respond with the corresponding performance value as a percentage.



If you encounter any errors or issues, refer to the error messages displayed in the console for troubleshooting.
```

