
## ⚠️ Base API in Rust in construction ⚠️


## API Reference

#### Root path
```http
  GET /api/
```
| Parameter | Type     | Description                |
| :-------- | :------- | :------------------------- |
| `null` | `null` | Add a private cookie to the connection to the server |

#### Auth path
```http
  POST /api/auth
```
| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `null`      | `null` | For the login |

#### Users path
```http
  GET /api/users
```
| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `null`      | `null` | Get all users |

```http
  POST /api/users
```
| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `null`      | `null` | Create new user |

```http
  GET /api/users/<user_id>
```
| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| **required** `id`      | `string` |  Get one user by id |