# Communication API:

## Index:
- [Communication API:](#communication-api)
  - [Index:](#index)
  - [Introduction:](#introduction)
  - [Features:](#features)
    - [User:](#user)
  - [API:](#api)
    - [**User**:](#user-1)
      - [POST HTTP `/new/user`:](#post-http-newuser)
      - [DELETE HTTP `/user`:](#delete-http-user)
      - [GET WS `/user/info`:](#get-ws-userinfo)
      - [POST HTTP `/user/ws`:](#post-http-userws)
      - [WS `/user/ws/{ws_token}`:](#ws-userwsws_token)
    - [**Groups**:](#groups)
      - [POST WS `/group/new`:](#post-ws-groupnew)
      - [DELETE WS `/group/delete`:](#delete-ws-groupdelete)
      - [POST WS `/group/add_user`:](#post-ws-groupadd_user)
      - [DELETE WS `/group/delete_user`:](#delete-ws-groupdelete_user)
      - [GET WS `/group/info`:](#get-ws-groupinfo)
    - [**User group**:](#user-group)
      - [GET WS `/user/groups`:](#get-ws-usergroups)
      - [GET WS `/user/group/msgs`:](#get-ws-usergroupmsgs)
      - [POST WS `/new_msg`:](#post-ws-new_msg)
      - [Personal msgs:](#personal-msgs)
    - [Filters:](#filters)

## Introduction:
This API documentation details all the endpoints and ways to communicate with the server.

This communication protocol is divided into two parts: HTTP and Websocket.
The HTTP protocol is related with requests that do not require to be logged (example: create a new user). The Websocket protocol is related with the real-time communication between users and the server.
In both cases, the communication is done with JSON messages. 

## Features:
### User:
- **POST** Create new user.
- **DELETE** user.
- **POST** Login user.
- **POST** Logout user.
- **POST** Add profile picture.
- **GET** user info.


## API:
### **User**:
#### POST HTTP `/new/user`:
Creates a new user.

**Warning**: This endpoint may change with firebase authentication.
```json
// Body of the request.
{
  "username": "paco"
}
```

#### DELETE HTTP `/user`:
Deletes a user.

**Warning**: This endpoint may change with firebase authentication.
```json
// Body of the request.
{
  "username": "paco"
}
```

<!-- #### POST HTTP `/user/login`: -->
<!-- #### POST WS `/user/logout`: -->
<!-- #### POST WS `/user/profile_pic`: -->
#### GET WS `/user/info`:
Returns the information of a user.

```json
// Body of the request.
{
  "username": "paco"
}
```

#### POST HTTP `/user/ws`:
Returns a new websocket connection for the user.

```json
// Body of the request.
{
  "username": "paco"
}
```

#### WS `/user/ws/{ws_token}`:
Websocket connection for the user:  `ws://{ip}:{port}/ws/{uuid}`.

### **Groups**:
Groups allow users to communicate with multiple people at the same time. The groups are created by the users, and they can add or remove users from the group.
#### POST WS `/group/new`:
Creates a new group.

```json
// Body of the request.
{
  "admin": "paco",
  "groupName": "group1"
}
```

#### DELETE WS `/group/delete`:
Deletes a group.

```json
// Body of the request.
{
  "admin": "paco",
  "groupName": "group1"
}
```

#### POST WS `/group/add_user`:
Adds a user to a group.

```json
// Body of the request.
{
  "username": "user1",
  "groupName": "group1"
}
```

#### DELETE WS `/group/delete_user`:
Deletes a user from a group.

```json
// Body of the request.
{
  "username": "user1",
  "groupName": "group1"
}
```

#### GET WS `/group/info`:
Returns the information of a group.

```json
// Body of the request.
{
  "groupName": "group1"
}

// Response.
{
  "groupName": "group1",
  "users": [
    "user1",
    "user2"
  ]
}
```

<!-- #### POST WS `/group/profile_pic`: -->

### **User group**:
#### GET WS `/user/groups`:
Returns the groups of a user.

```json
// Body of the request.
{
  "username": "paco"
}

// Response.
{
  "username": "paco",
  "groups": [
    "group1",
    "group2"
  ]
}
```

#### GET WS `/user/group/msgs`:
Returns the previous messages of a group.

```json
// Body of the request.
{
  "groupName": "group1",
  "filters": {
    "from": "2023-01-01T00:00:00Z",
    "to": "2023-01-31T00:00:00Z"
  }
}
```

#### POST WS `/new_msg`:
Sends a new message to a group.

```json
// Body of the request.
{
  "username": "paco",
  "groupName": "group1",
  "msg": "Hello world!",
  "info": {}
}
```

#### Personal msgs:
Each user can send a message to another user.
- Works in with "private" group. Use endpoint [/new_msg](#post-ws-new_msg).
- This group can not be modified.
- No other user can be added to this group.

```json
// Body of the request.
{
  "username": "paco",
  "groupName": "private",
  "msg": "Hello Pepe!",
  "info": {
    "to": "pepe"
  }
}
```

### Filters:
All filters are optional.
- `before` = **date**: Returns the messages before the timestamp.
- `after` = **date**: Returns the messages after the timestamp.
