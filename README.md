# 💣 SQLand

SQLand is a tool for detecting SQL Injection vulnerabilities in web applications. It supports both time-based and error-based blind SQL injection detection techniques.

## ⚡ Features

- ⚡  High-efficient and multi-threading.
- 📋 Comprehensive logging.
- 🌈 Colored output for better readability.
- 🕔 Time-based Blind SQL Injection detection.
- 🔍 Error-based Blind SQL Injection detection.
- ❌ Smart DOM sql-like errors detection.
- 🍪 Custom cookies, headers and static params support.

## 📋 Usage

To use SQLand, clone the repository and run the tool with the appropriate arguments.

### Arguments

| Argument                 | Description                               | Type                                | Multi |
|--------------------------|-------------------------------------------|-------------------------------------|:-----:|
| `-x` `--method`          | HTTP method to use                        | `GET` `POST` `PUT` `PATCH` `DELETE` | ❌   |
| `-H` `--header`          | Append a header to the request            | `"string: string"`                  | ✅   |
| `-c` `--cookie`          | Append a cookie to the request            | `"string: string"`                  | ✅   |
| `-p` `--param`           | Add a query/body param to fuzz payloads   | `string`                            | ✅   |
| `-d` `--data`            | Append a query/body param without fuzz    | `"string: string"`                  | ✅   |
| `-j` `--json`            | Post param and data as JSON               | `boolean`                           | ❌   |
| `-f` `--form`            | Post param and data as Form Data          | `boolean`                           | ❌   |
| `-w` `--workers`         | Number of simultaneous payload requests   | `number (Default 4)`                | ❌   |
| `-n` `--no_filtering`    | Don't use vanilla request for filtering   | `boolean`                           | ❌   |
| `-s` `--offset_samples`  | Samples to calculate avg response time    | `number (Default 0)`                | ❌   |
| `-o` `--offset`          | Time based attack latency offset          | `number (Default 0)`                | ❌   |

### Example

```bash
# Command syntax
sqland <Optional Arguments> [URL] # This is valid.
sqland [URL] <Optional Arguments> # This is valid too.

# Example for http://example.com/search?query=<payload>
sqland http://example.com/search -p query

# Example for http://example.com/search?query=<payload>&foo=bar
sqland http://example.com/search -p query -d foo=bar
```

## 📦 Build from source

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/sammwyy/sqland

# Build
cd sqland && cargo build --release
```

## 🤝 Contributing

We welcome contributions! If you'd like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch (git checkout -b feature-branch).
3. Make your changes.
4. Commit your changes (git commit -am 'prefix: 😀 describe your commit here').
5. Push to the branch (git push origin feature-branch).
6. Create a new Pull Request.

Please ensure your code adheres to the existing style, and includes tests where applicable. Feel free to check [issues page](https://github.com/sammwyy/sqland/issues).

## ❤️ Show your support

Give a ⭐️ if this project helped you! Or buy me a coffeelatte 🙌 on [Ko-fi](https://ko-fi.com/sammwy)

## 📝 License

Copyright © 2023 [Sammwy](https://github.com/sammwyy). This project is [MIT](LICENSE) licensed.
