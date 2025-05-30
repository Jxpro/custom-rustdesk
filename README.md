# RustDesk Custom ID Tool

## Introduction

>   Note that if a certain ID is invalid and reset, it may be that the ID is too short or occupied, please try a different ID.

中文文档: [README_CN.md](https://github.com/Jxpro/custom-rustdesk/blob/main/README_CN.md)

This project aims to provide a custom ID generation function for `RustDesk`, making it easier for users to remember and manage devices. The primary function is to simulate the official encryption algorithm, encrypting user-entered custom IDs and outputting the encrypted ID. By replacing the `enc_id` field in the configuration file with this output, users can complete the setup.

For MacOS, the configuration file is located at:

-   `~/Library/Preferences/com.carriez.RustDesk/RustDesk.toml`

For Windows, the configuration file is located at:

-   `C:\Users\username\AppData\Roaming\RustDesk\config\RustDesk.toml`

When running in service mode in Windows, specified by `--service` :

-   `C:\Windows\ServiceProfiles\LocalService\AppData\Roaming\RustDesk\config\RustDesk.toml`

## Features
-   Generate encrypted custom ID
-   Decrypt and validate encrypted ID
-   Use UUID as the encryption and decryption key

## Usage

### Get UUID

>   You can get more information at the official tool [machine-uid](https://github.com/rustdesk-org/machine-uid)

1.  **Windows:**

    -   Press `Win + R` to open the Run dialog box.
    -   Type `regedit` and press Enter to open the Registry Editor.
    -   Navigate to `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Cryptography`.
    -   Copy the `MachineGuid` value as the `uuid` parameter.

2.  **MacOS:**

    -   Open Terminal.
    -   Enter the following command: `ioreg -rd1 -c IOPlatformExpertDevice | grep IOPlatformUUID`
    -   Copy the UUID from the output as the `uuid` parameter.

### Validate UUID

1.  Clone the code locally.
2.  Find the `enc_id` field in the respective configuration file.
3.  Run the command `cargo run -- --eid $enc_id --uuid $uuid`.
4.  The program will output the decrypted ID, compare it with the current ID to check consistency.

### Customize ID

1.  Clone the code locally.
2.  Run the command `cargo run -- --id $custom_id --uuid $uuid`.
3.  The program will output the encrypted ID, copy and replace it in the `enc_id` field of the configuration file.

Example of program execution:

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
