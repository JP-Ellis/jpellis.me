---
github: pact-foundation/pact-python
slug: pact-python
tagline: Contract testing for Python services, built on a Rust FFI core
title: pact-python
---

[Pact](https://pact.io) is a contract testing framework: rather than spinning up a full integration environment, each consumer records the interactions it expects from a provider into a _pact file_. The provider then verifies it can satisfy those interactions independently. This eliminates an entire class of integration test failures without the cost of a shared environment.

`pact-python` is the Python library in the [Pact Foundation](https://github.com/pact-foundation) ecosystem. I rebuilt it from the ground up over a Rust FFI core ([`pact-reference`](https://github.com/pact-foundation/pact-reference)), replacing the previous approach of shelling out to a Ruby binary. The result is a library that installs as a plain Python wheel with no system dependencies, supports the full Pact specification (V1–V4), and runs on all major platforms.

## Usage

```python
import pytest
from pact import Consumer, Provider

pact = Consumer("Consumer").has_pact_with(Provider("Provider"))

def test_get_user():
    (pact
     .given("user 1 exists")
     .upon_receiving("a request for user 1")
     .with_request("GET", "/users/1")
     .will_respond_with(200, body={"id": 1, "name": "Alice"}))

    with pact:
        result = get_user(1)          # call your actual client code
        assert result["name"] == "Alice"
```

The library is published to [PyPI](https://pypi.org/project/pact-python/).
