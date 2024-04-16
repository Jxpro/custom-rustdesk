# Rustdesk Custom ID Tool

## Introduction

>   Note that if a certain ID is invalid and reset, please try more IDs. For example, on my Mac computer, IDs starting with 'mac' are all reset to other random ID.

This project aims to provide a custom ID generation function for `Rustdesk`, making it easier for users to remember and manage devices. The main function is to simulate the official encryption algorithm, encrypt the user-entered custom ID, and output the encrypted ID.

中文文档: [README-CN.md](https://github.com/Jxpro/custom-rustdesk/README–CN.md) 

## Features
-   Generate encrypted custom ID
-   Use UUID as the encryption key

## Usage

### Get UUID

1.  **Windows:**

    -   Press `Win + R` to open the Run dialog box.
    -   Type `regedit` and press Enter to open the Registry Editor.
    -   Navigate to `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Cryptography`.
    -   Copy the `MachineGuid` value as the `uuid` parameter.

2.  **MacOS:**

    -   Open Terminal.
    -   Enter the following command and press Enter:

    ```shell
    ioreg -rd1 -c IOPlatformExpertDevice | grep IOPlatformUUID
    ```

    -   Copy the UUID from the output as the `uuid` parameter.

### Run the program

1. Clone the code to your local machine
2. Run  `cargo run -- --id $id --uuid $uuid`
3. The program will output the encrypted ID

Example:

```shell
cargo run -- --id 123456 --uuid 12345678-1234-1234-1234-123456789012
```

## Encryption Process

This program uses the `crypto::secretbox` module in the `sodiumoxide` library for symmetric encryption. The encryption key comes from the provided UUID string.

1.  Convert the custom ID string into a byte array.
2.  Convert the UUID string into a byte array and adjust its size to match the key length requirement.
3.  Use the `sodiumoxide::crypto::secretbox` module to create a key and `nonce`.
4.  Choose encryption or decryption operation based on the `encrypt` parameter.
5.  Use the `secretbox::seal` or `secretbox::open` function for encryption or decryption.
6.  Convert the encrypted byte array to a `base64` encoded string and output it to the console.

## Contributing

You are welcome to contribute to this project! You can participate in the following ways:

-   Submit code patches or issue reports
-   Provide feedback and suggestions
-   Help promote the project

## Contact

If you have any questions, feel free to discuss on [github issue](https://github.com/Jxpro/custom-rustdesk/issues), or send an email to [jxpro@qq.com](mailto:jxpro@qq.com) to contact me.