AUTHOR_UUID="8c96d36f-4788-4ebc-bbf8-fd4f5d602345"
TYPE_UUID="9cb7d657-736b-4ba4-988f-60b16194a0d7"
TITLE="PruebaTitulo"
CONTENT="This content has commas, and will be fine."
TAGS="ec3445f8-bb02-4335-aa5e-ddd61b9adb93,2565b896-63de-435e-a70b-60bba191bf18"
SEPARATOR="---FILE_SEPARATOR---"
IMAGE_PATH="./manzana.png"

METADATA_STRING="${AUTHOR_UUID}###${TYPE_UUID}###${TITLE}###${CONTENT}###${TAGS}"
echo -n "$METADATA_STRING" > payload.bin

echo -n "$SEPARATOR" >> payload.bin

cat "$IMAGE_PATH" >> payload.bin

PAYLOAD_SIZE=$(wc -c < payload.bin)

# Send the request
curl -v -X POST 'http://localhost:7879/writings' \
  -H 'Content-Type: application/octet-stream' \
  -H "Content-Length: $PAYLOAD_SIZE" \
  --data-binary "@payload.bin"