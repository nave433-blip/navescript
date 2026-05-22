# NaveScript API Reference

## Standard Library
The NaveScript standard library (`@std`) provides essential utilities for file I/O, networking, and system operations.

### File System (`fs`)
| Function | Description |
| :--- | :--- |
| `fs.read(path)` | Reads a file from the disk. |
| `fs.write(path, content)` | Writes data to a file. |
| `fs.stat(path)` | Returns file metadata. |

### Network (`http`)
| Function | Description |
| :--- | :--- |
| `http.get(url)` | Performs a GET request. |
| `http.post(url, body)` | Performs a POST request. |

### Cryptography (`crypto`)
| Function | Description |
| :--- | :--- |
| `crypto.sha256(data)` | Returns the SHA-256 hash of a string. |

### JSON (`json`)
| Function | Description |
| :--- | :--- |
| `json.marshal(v)` | Serializes a value to JSON. |
| `json.unmarshal(data, v)` | Deserializes JSON data. |
