### Cipherize - CLI Tool

#### Usage
```bash
# Encode a text
cipherize encode --cipher rot13 --message "this is some message"

# Decode a text
cipherize decode --cipher rot13 --message "this is encrypted data"

# Encode a text and sent to an outputfile
cipherize encode --cipher rot13 --message "this is encrypted data" --output-file encode.txt

# Encode a text from an inputfile
cipherize encode --cipher rot13 --input-file example.txt

```