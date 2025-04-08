# Experimental tool to trigger UNIX automounts or time out

## usage

```
./trymount <somepath>
if [ $? -ne 0 ]; then
    echo "Error"
fi
```

