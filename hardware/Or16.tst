load Or16.hdl,
output-file Or16.out,
compare-to Or16.cmp,
output-list a%B1.16.1 b%B1.16.1 out%B1.16.1;

set a %B0000000000000000,
set b %B0000000000000000, eval, output;

set a %B0000000000000000,
set b %B1111111111111111, eval, output;
