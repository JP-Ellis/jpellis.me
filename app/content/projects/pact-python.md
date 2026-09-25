---
github: pact-foundation/pact-python
slug: pact-python
tagline: Contract testing for Python services, built on a Rust FFI core
title: pact-python
---

[Pact](https://pact.io) is a contract testing framework. The consumer records the requests it makes and the responses it expects in a _pact file_. The provider then replays that file against itself and checks each response. Neither test needs the other service running.

`pact-python` is the Python library in the [Pact Foundation](https://github.com/pact-foundation) ecosystem. I rebuilt it from the ground up over a Rust FFI core ([`pact-reference`](https://github.com/pact-foundation/pact-reference)), replacing the previous approach of shelling out to a Ruby binary. The result is a library that installs as a plain Python wheel with no system dependencies, supports the full Pact specification (V1–V4), and runs on all major platforms.

## Usage

```python
from pathlib import Path

from pact import Pact, match


def test_get_user() -> None:
    pact = Pact("user-consumer", "user-provider")
    (
        pact
        .upon_receiving("a request for user 1")
        .given("user 1 exists")
        .with_request("GET", "/users/1")
        .will_respond_with(200)
        .with_body(
            {"id": match.int(1), "name": match.str("Alice")},
            content_type="application/json",
        )
    )

    with pact.serve() as srv:
        client = UserClient(str(srv.url))  # your actual client code
        assert client.get_user(1).name == "Alice"

    pact.write_file(Path(__file__).parent / "pacts")
```

The library is published to [PyPI](https://pypi.org/project/pact-python/).
