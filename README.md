# Word to number
![Hackatime badge](https://hackatime.hackclub.com/api/v1/badge/U08D22QNUVD/Spacexplorer11/word-to-number)  
This is my first ever API!! I'm very proud of myself for making it by myself, without AI-Assistance at all!!
The [Rust Book tutorial](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html) on this was very, very useful!

Check out my Scalar page: https://registry.scalar.com/@akaalroop/apis/word-to-number-api@latest  
[Watch Demo](https://github.com/user-attachments/assets/8b700070-60d7-46b5-8861-88bfda611c56)

## What is it?
This is an API where you send string numbers and receive integers. For example, you can exchange "sixty-seven" for 67.  
This API supports big numbers! The words we support go up to "billion". It can't understand trillion for example. However, you can chain numbers like "one thousand billion" to get 1,000,000,000,000!

## Usage
### The number itself
Hi! This gets a whole section to itself because (at least for now) my API is quite picky.
#### Valid examples
- "sixty-seven"
- "one hundred and forty-seven"
- "one hundred forty-two"
- "one"
- "two thousand and ninety"
- "two thousand five hundred and sixty-three"
- "five thousand"
- "fifty-seven thousand"
- "sixty-three thousand and five hundred twenty-three"
- "two million seven hundred thousand and sixty-four"

#### Invalid examples
- "sixty seven"
- "one hundred and fifty and nine"
- "two thousand and five hundred and sixty-three"
- "a million"
- "the million"
- "two million, five hundred and seventy-five" (commas are banned)

Hopefully the examples show everything, but to clarify:
#### Requirements:
- You must have a hyphen ("-") between two-digit numbers.
- You are not allowed to add commas.
- You may only have "and" once in your request.
- You must use English.
- No other numbers other than integers are allowed.
- You must use spaces between numbers

### Request
You must send a POST request to the server at https://word-to-number.akaalroop.com/.
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
The only 4 error codes you can receive from my server are (two others from Cloudflare are listed):
- `400` - Bad request. There are only 5 reasons you may get this:
- - Typo - You typed something wrong, or the number you sent isn't supported.
- - Non-UTF-8 Byte - You sent a byte which couldn't be encoded in UTF-8 and hence was rejected.
- - Content Length Zero - The content length header you sent, was zero. This is unacceptable as we require a body & a correct content length to process the body.
- - Content Length Malformed - The content length header you sent, was malformed. For example, could be if it had something NaN (Not a Number) in it.
- - Malformed Body - You didn't follow the rules set out [above](#request) and hence your request was rejected.
- `408` - Request Timed Out. Your request timed out. This may be due to a variety of reasons. For example, you may have sent fewer bytes than your Content-Length header said or just stopped sending bytes for more than 5 seconds.
- `411` - Length Required. You didn't provide a `Content-Length` header and without that my API rejects your request. You may also get this if you send no body which isn't acceptable.
- `429` - Too Many Requests. *(Cloudflare-Generated)* This may be returned by my Cloudflare Tunnel since I have a ratelimit of 15 requests / 10 seconds to protect my server.
- `500` - Internal Server Error. This is usually out of your control, but if you receive these a lot please email [akaal@akaalroop.com](mailto:akaal@akaalroop.com) and let me know!
- `502` - Bad Gateway. *(Cloudflare-Generated)* Since I use a Cloudflare Tunnel, you'll be able to connect but if you receive this then that means *my server* is down / not running the API. If you get this please email [akaal@akaalroop.com](mailto:akaal@akaalroop.com) and let me know!

## Self-hosting
1. Clone the repo
2. Run `cargo run`.
3. Send requests to `http://127.0.0.1:7878/`.

## Have you tested it?
Yes I have tested every, single, number up to 999 and they all work. You can test it yourself by downloading [test.json](/test.json) and running:
```bash
curl -s -X POST https://word-to-number.akaalroop.com/ -H "Content-Type: application/json" -d "$(cat test.json)"
```
**Warning:** Big requests like that may time out. (Command may not work on all systems)
