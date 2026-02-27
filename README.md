# bfint
Brainfukk interpreter using the power of rust

To run this you need docker compose installed.

### Examples:

```shell
# Hello World!
./run.sh run '++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.'


# a simple addition (2 + 3)
./run.sh run examples/add.bf
```
