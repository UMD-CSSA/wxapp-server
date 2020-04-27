<h1 align="center">
  <a href="https://github.com/UMD-CSSA/wxapp-server.git/">UMD-CSSA WeApp Server API</a>
</h1>

![GitHub commit activity](https://img.shields.io/github/commit-activity/y/UMD-CSSA/wxapp-server.svg)
![GitHub contributors](https://img.shields.io/github/contributors/UMD-CSSA/wxapp-server)
![GitHub issues](https://img.shields.io/github/issues/UMD-CSSA/wxapp-server.svg)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/UMD-CSSA/wxapp-server.svg)
![GitHub license](https://img.shields.io/github/license/UMD-CSSA/wxapp-server.svg)

## Description

**wxapp-server** is a backend server that provides API for [UMD CSSA Miniapp](https://github.com/UMD-CSSA/CSSA-MiniApp).

*Last Modified on **Nov. 8th, 2019**.*

## Contents

- [**Important Versions**](#important-versions)
- [**API List**](#api-list)
  1.  [Freshman Handbook Image (IP)](#freshman-handbook-image)
  2.  [MiniApp Login (IP)](#miniapp-login)
- [Contributing](#contributing)
- [Authors](#authors)
- [License](#license)

## Important Constants

|   Item        | Version                                        |
| :-----------: | ---------------------------------------------- |
| Host URL      | `https://wxapp.umd-cssa.org/api/{API_VERSION}` |
| API Ver.      | `v0`                                           |
| Handbook Ver. | `v2018`                                        |

## API List

### 1. Freshman Handbook Image

#### Request

| Method | URL                            |
| :----: | ------------------------------ |
|  GET   | `/handbook/{HANDBOOK_VERSION}` |

Parameter(s):

| Key Name | Value Type     | Example | Description  |
| :------: | -------------- | ------- | ------------ |
|   `pg`   | `int [1, 142]` | `1`     | Page number. |

---

#### Respond

| Content-Type                | Content    |
| --------------------------- | ---------- |
| `image/png` OR `image/jpeg` | Raw image. |

#### Example

- `HTML` / `WXML`

```html
<image src="https://wxapp.umd-cssa.org/api/v0/handbook/v2018?pg=1" />
```

### 2. MiniApp Login

#### Request

| Method | URL              |
| :----: | ---------------- |
|  GET   | `/miniapp/login` |

Parameter(s):

| Key Name | Value Type | Example                            | Description                    |
| :------: | ---------- | ---------------------------------- | ------------------------------ |
|  `code`  | `String`   | `0818RlAt1MmAef07eAat1PYoata18lAa` | Code return from `wx.login()`. |

---

#### Respond

| Content-Type       | Content     |
| ------------------ | ----------- |
| `application/json` | _See Below_ |

Content:

| Key Name       | Key Type | Value Type | Example |
| -------------- | -------- | ---------- | ------- |
| `_3rd_session` | `String` | `String`   | //TODO  |

#### Example

- `Javascript`

```javascript
wx.login({
  success(res) {
    if (res.code) {
      wx.request({
        url: "https://wxapp.umd-cssa.org/api/v0/miniapp/login",
        data: {
          code: res.code
        },
        success(res) {
          if (statusCode === 200) {
            // Store this sesssion for future use.
            console.log(res.data._3rd_session);
          }
        }
      });
    }
  }
});
```

---

## Contributing

### Option 1: Submit via GitHub Issue (recommended)

It is strongly encouraged to submit bug reports and feature requests through
[GitHub Issue](https://github.com/UMD-CSSA/wxapp-server/issues)
page. It will help us organize and keep track of every issue reported.

### Option 2: Contact us via Email

Please email to [umdcssait@gmail.com](mailto:umdcssait@gmail.com), and make sure to includce `Wechat MiniApp` in subject.

## Authors

- **[Jerry C.](https://github.com/jerryc05)**
  - Member of CSSA IT department
  - Major in Computer Science

## License

This project is licensed under the GNU v3 License - see
[LICENSE.md](https://github.com/UMD-CSSA/wxapp-server.git/blob/master/LICENSE)
for details.
