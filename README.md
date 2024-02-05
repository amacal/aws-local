# aws-local

A local HTTP server for proxying requests to AWS Lambda functions, facilitating the development and testing of Lambda functions locally.

## Introduction

`aws-local` is a Rust-based CLI tool that creates a local server mimicking AWS API Gateway, allowing developers to test and debug AWS Lambda functions locally. It routes incoming HTTP requests to your Lambda function and returns the function's response, streamlining the Lambda development cycle by providing immediate feedback and compatibility with `cargo lambda watch`.

## Installation

Install aws-local directly using Cargo:

```bash
cargo install aws-local
```

## Usage

1. Run aws-local to start intercepting and forwarding HTTP requests to your local AWS Lambda function.

    ``` bash
    aws-local --function-name your-function-name -vvv
    ```

3. Start your local lambda function runtime.

    ``` bash
    cargo lambda watch -a 127.0.0.1
    ```

2. Invoke Your Lambda Function Locally, by sending requests to http://localhost:3000/your-lambda-endpoint. `aws-local` will handle the requests, forwarding them to your Lambda function and returning the responses.

## Contributing

Contributions are welcome! If you have a feature request, bug report, or a pull request, please feel free to open an issue or a pull request on the project repository.

## License

`aws-local` is released under the MIT License. See the LICENSE file for more details.
