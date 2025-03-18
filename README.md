# diffwrite

writes stdin to a file, while also printing a diff

```bash
# tired, dusty
echo "abc" > results.txt
# (no output)

# amazing, really good
echo "def" | diffwrite results.txt
# --- a/results.txt
# +++ b/results.txt
# @@ -1,1 +1,1 @@
# -abc
# +def
```

| redirection                       | diffwrite                         |
| --------------------------------- | --------------------------------- |
| prints nothing, how unixy         | prints a diff                     |
| streams, wow, unix philosophy     | reads the whole thing into memory |
| cant even pipe a file into itself | like a snake eating its tail      |
| posix                             | live a little                     |
