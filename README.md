# tinayiluo_IDS721_individualproject4

[![pipeline status](https://gitlab.com/ly178/tinayiluo_ids721_individualproject4/badges/main/pipeline.svg)](https://gitlab.com/ly178/tinayiluo_ids721_individualproject4/-/commits/main)

# Rust AWS Lambda and Step Functions Project

## Overview

This project demonstrates a robust serverless architecture using Rust, AWS Lambda, AWS DynamoDB, and AWS Step Functions. It is designed to showcase how serverless functions can be used to create scalable and efficient data processing pipelines in the cloud. The primary purpose of this project is to implement a series of interconnected AWS Lambda functions that handle different aspects of data management—from generating data to storing and aggregating it.

The architecture leverages the safety and performance of Rust to handle backend logic, AWS Lambda for serverless execution, DynamoDB for noSQL database interactions, and Step Functions for seamless orchestration of the workflow. This setup exemplifies a real-world scenario where different cloud components work together to process and manage data effectively.

## Goal

This project utilizes Rust to create AWS Lambda functions orchestrated by AWS Step Functions, forming a data processing pipeline that interacts with AWS DynamoDB. The pipeline consists of three primary components:

1. **Quote Generator (`step1`)**: Generates a random motivational quote.
2. **Database Handler (`step2`)**: Stores and retrieves quotes in/from DynamoDB.
3. **Quote Counter (`step3`)**: Counts the total quotes stored.

The functions are developed in Rust and managed through GitLab CI/CD for automated deployment.

## Technologies Used

- Rust
- AWS Lambda
- AWS DynamoDB
- AWS Step Functions
- GitLab CI/CD
- Cargo

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