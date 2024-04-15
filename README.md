# tinayiluo_IDS721_individualproject4

[![pipeline status](https://gitlab.com/ly178/tinayiluo_ids721_individualproject4/badges/main/pipeline.svg)](https://gitlab.com/ly178/tinayiluo_ids721_individualproject4/-/commits/main)

# Rust AWS Lambda and Step Functions Project

## Overview

This project demonstrates a robust serverless architecture using Rust, AWS Lambda, AWS DynamoDB, and AWS Step Functions. It is designed to showcase how serverless functions can be used to create scalable and efficient data processing pipelines in the cloud. The primary purpose of this project is to implement a series of interconnected AWS Lambda functions that handle different aspects of data management—from generating data to storing and aggregating it.

The architecture leverages the safety and performance of Rust to handle backend logic, AWS Lambda for serverless execution, DynamoDB for noSQL database interactions, and Step Functions for seamless orchestration of the workflow. This setup exemplifies a real-world scenario where different cloud components work together to process and manage data effectively.

## Key Features

1. **Quote Generator (`step1`)**: Generates a random motivational quote.
2. **Database Handler (`step2`)**: Stores and retrieves quotes in/from DynamoDB.
3. **Quote Counter (`step3`)**: Counts the total quotes stored.

These components are carefully coordinated using AWS Step Functions, which illustrates how different serverless functions can be linked to form a coherent data processing pipeline. The project is also integrated with GitLab CI/CD to automate the deployment and management of these functions, ensuring a smooth development lifecycle and continuous delivery.

## Technologies Used

- Rust
- AWS Lambda
- AWS DynamoDB
- AWS Step Functions
- GitLab CI/CD
- Cargo

## Configuration Files

### `Cargo.toml`

This file configures all metadata needed by Cargo to compile the project:

- **Package details**: Includes the name, version, and edition of the Rust package.
- **Binary targets**: Specifies three separate binaries, each corresponding to a different step of the project and linked to specific source files.
- **Dependencies**: Lists external crates like `aws-sdk-dynamodb` for DynamoDB integration, `lambda_runtime` for AWS Lambda compatibility, and others essential for the project functionality.

### `Makefile`

The Makefile simplifies command execution for various phases of project development:

- **Compilation and Deployment**: Includes commands to build and deploy each step of the Lambda functions.
- **Utility Commands**: Provides quick access to format, lint, and test the entire Rust project.
- **Local and Remote Invocation**: Facilitates both local and AWS remote testing of Lambda functions using specified test commands.

## Environment Setup

### Local Environment Variables

Before deploying and testing the Lambda functions, set up the necessary AWS credentials and configuration on your local environment. Use an `.env` file to manage these variables securely.

1. **Create the .env File:**

    Create a file named `.env` in the root directory of your project and add the following environment variables:

    ```plaintext
    AWS_ACCESS_KEY_ID=your_access_key_id
    AWS_SECRET_ACCESS_KEY=your_secret_access_key
    AWS_DEFAULT_REGION=your_default_region
    ```

2. **Load Environment Variables:**

    Before running or deploying your functions, load the environment variables:

    ```bash
    set -a
    source .env
    set +a
    ```

### Adding Variables in GitLab CI/CD

To ensure that your CI/CD pipeline has access to the necessary AWS credentials, add these as environment variables in your GitLab project settings.

## Step1: Test Locally

### Description
The first step involves a Rust Lambda function (`main.rs`) that generates a random motivational quote using the `rand` crate. This function can respond with one of two hardcoded quotes.

### Local Testing
To test this Lambda function locally, use the following commands:

```bash
cargo lambda watch
cargo lambda invoke --data-ascii "{ \"command\": \"get\"}"
```

### Local Deployment
Build and deploy the function to AWS:

```bash
cargo lambda build --bin step1 --release
cargo lambda deploy step1
```

## Step2: Test on AWS

### Description
The second Lambda function (`db.rs`) interacts with DynamoDB to store the quote generated in Step1. It utilizes the AWS SDK for DynamoDB to put items into the table.

### AWS Deployment
Build and deploy the function to AWS, ensuring it has appropriate permissions to interact with DynamoDB:

```bash
cargo lambda build --bin step2 --release
cargo lambda deploy step2
```

### Testing on AWS
Test the functionality by invoking the Lambda function with a sample event:

```json
{
  "req_id": "tina",
  "msg": "yi"
}
```

## Step3: Test on AWS

### Description
The third Lambda function (`view.rs`) counts the total number of quotes stored in DynamoDB. It retrieves this data and outputs the count.

### AWS Deployment
Build and deploy this function to AWS:

```bash
cargo lambda build --bin step3 --release
cargo lambda deploy step3
```

### Testing on AWS
Invoke the function with any sample event to get the count of quotes:

```json
{
  "req_id": "String",
  "msg": "String"
}
```

## AWS Step Function

### Configuration
Set up AWS Step Functions through the AWS Management Console to orchestrate the sequence of invoking `step1`, `step2`, and `step3` based on your designed workflow. The final output will display the total number of quotes.

### Execution
Trigger the Step Function with the initial command to observe the entire flow:

```json
{
  "command": "get"
}
```

## CI/CD with GitLab

### Description
The project includes a `.gitlab-ci.yml` file for automating the build, test, and deployment process using GitLab CI/CD.

### Setup
Ensure that AWS environment variables are set in the GitLab project settings to allow the CI/CD pipeline to access AWS resources.

### CI/CD Pipeline Steps
The CI/CD pipeline is configured to perform the following tasks:
- Lint and format the code.
- Build each Lambda function.
- Deploy each function to AWS.

```yaml
stages:
  - build-test-deploy

build-test-deploy:
  stage: build-test-deploy
  script:
    - make lint 
    - make format 
    - make test 
    - make build1
    - make build2
    - make build3
    - make deploy1
    - make deploy2
    - make deploy3
```

## Deliveries 

# Step 1 Local Test 

![Screen_Shot_2024-04-13_at_10.51.48_PM](/uploads/72d69ecc93b540d9bd07c88dccc0ae1d/Screen_Shot_2024-04-13_at_10.51.48_PM.png)

# Step 2 AWS Lambda Test on DynamonDB Table 

![Screen_Shot_2024-04-14_at_1.48.20_PM](/uploads/1171d065592bab1718a16aa5b8d00c50/Screen_Shot_2024-04-14_at_1.48.20_PM.png)

# Step 3 AWS Lambda Test

![Screen_Shot_2024-04-14_at_1.53.44_PM](/uploads/a9d9abab76b214e17603f87060f264c8/Screen_Shot_2024-04-14_at_1.53.44_PM.png)

# AWS Step Function

![stepfunctions_graph](/uploads/f5d6c2989f85efbe7f39228fe1bf96d8/stepfunctions_graph.png)

# AWS Step Function Test

![Screen_Shot_2024-04-15_at_12.33.25_PM](/uploads/662fc2c0250fb8807556b126a2902ca5/Screen_Shot_2024-04-15_at_12.33.25_PM.png)
