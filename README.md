<div align=right>Table of Contents↗️</div>

<h1 align=center><code>fake-rs</code></h1>

<p align=center>📦 A fake data generator for Rust.</p>

<div align=center>
  <a href="https://crates.io/crates/fake-rs">
    <img src="https://img.shields.io/crates/v/fake-rs.svg" alt="crates.io version">
  </a>
  <a href="https://crates.io/crates/fake-rs">
    <img src="https://img.shields.io/github/repo-size/lvillis/fake-rs?style=flat-square&color=328657" alt="crates.io version">
  </a>
  <a href="https://github.com/lvillis/fake-rs/actions">
    <img src="https://github.com/lvillis/fake-rs/actions/workflows/ci.yaml/badge.svg" alt="build status">
  </a>
  <a href="mailto:lvillis@outlook.com?subject=Thanks%20for%fake-rs!">
    <img src="https://img.shields.io/badge/Say%20Thanks-!-1EAEDB.svg" alt="say thanks">
  </a>
</div>

---

`fake-rs` is a Rust library for generating fake data such as random user agents, names, phone numbers, and IP addresses.
It's designed to provide a simple and easy-to-use interface for developers to quickly generate mock data for testing,
simulation, and other scenarios where random data is needed.

## Features

- **User Agent Generation**: Randomly select a user agent from a predefined list.
- **Name Generation**: Randomly generate names.
- **Phone Number Generation**: Generate properly formatted random phone numbers.
- **IP Address Generation**: Generate random IPv4 addresses.

## Usage

```toml
[dependencies]
fake-rs = { version = "0.1.0", features = ["user-agent", "name", "phone", "ip"] }
```

Each module provides one or more functions to generate specific types of data. Here are some usage examples:

### User Agent Generation

```
use fake_rs::get_random_user_agent;
let user_agent = get_random_user_agent();
let user_agent_chrome = get_chrome_user_agent();
```

### Name Generation

```
use fake_rs::generate_name;
let name = generate_name();
```

### Phone Number Generation

```
use fake_rs::generate_phone;
let phone_number = generate_phone();
```

### IP Address Generation

```
use fake_rs::generate_ip;
let ip_address = generate_ip();
```