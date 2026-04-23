# Word to number
![](https://hackatime.hackclub.com/api/v1/badge/U08D22QNUVD/Spacexplorer11/word-to-number)  
This is my first ever API!! I'm very proud of myself for making it by myself, without AI-Assistance at all!!
The [Rust Book tutorial](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html) on this was very, very useful!


## What is it?
This is an API where you send string numbers and receive integers. For example, you can exchange "sixty-seven" for 67.  
>[!Important]
> This API only supports numbers up to 999 at the moment. This will change within the next 7 days.

## Usage
### Request
You must send a POST request to the server at https://word-to-number.akaalroop.com.  
The body of the request must be JSON formatted like this:  
```json
{
    "word": "six hundred and seventy-six"
}
```
You **may** include multiple numbers like this:  
```json
{
    "word": "six hundred and seventy-six",
    "word-1": "seven hundred and sixty-seven"
}
```  
The above is the idiomatic way however you may replace the keys with anything that includes "word". For example:  
```json
{
    "word-a": "six hundred and seventy-six",
    "word-b": "seven hundred and sixty-seven"
}
```  
```json
{
    "Aword": "six hundred and seventy-six",
    "Bword": "seven hundred and sixty-seven"
}
```  
As long as it includes "word", it's acceptable.

### Response
The response of the API will be cacheable for 7 days.  
It will be like:  
```json
{
    "number": 676
}
```  
For multiple numbers the response will always be like this:  
```json
{
"number": 676,
"number-1": 767,
"number-2": 67
}
```  
The numbers' index will go up to a reasonable amount but does depend on how the server is feeling at that moment.

### Errors
The only 3 error codes you can receive are:
- `400` - Bad request. Make sure your request is formatted to the requirements in request section. This will also be returned if the request has a typo or if the number is unsupported. **Currently only numbers up to 999 are supported. This will change soon**  
- `408` - Request Timed Out. Your request timed out. This may be due to a variety of reasons. One reason may be no ending `}`. The code requires that to understand when the request has finished.  
- `500` - Internal Server Error. This is usually out of your control, but if you receive these a lot please email [akaal@akaalroop.com](mailto:akaal@akaalroop.com) and let me know!

## Self-hosting
1. Clone the repo
2. Run `cargo run`.
3. Send requests to http://127.0.0.1:7878/.
