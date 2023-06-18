HTTP Server that allows the upload of binary data over http and returns the IPFS hash of that content.
Requires Basic Auth.

that should work:
```
curl -X POST https://xxx.example.com:443/api/storage/upload -H "Content-Type:application/octet-stream" --data-binary @file_name.file
```
