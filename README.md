
# Google Workspace Mailer

Send emails from a Google Workspace email address.  

This functionality requires that "Less secure apps" are enabled for each Google Workspace account, which is not recommended for a production environment. Additionally, the "Less secure apps" feature is [being phased out for Google Workspace accounts in 2024](https://support.google.com/a/answer/14114704).  

## Usage

```
cargo run
```

## Configuration

- `EMAIL`: The email address for the Google Workspace account.  
- `PASSWORD`: The password for the Google Workspace account.  
- `RECIPIENT`: The email address to send to.  
- `SUBJECT`: The subject of the email.  
- `BODY`: The body of the email.  
