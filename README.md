<h2>Summary</h2>
A simple interpreted programming language made with rust.

<h2>VARIABLE DECLARATION</h2>
```
bag foo = 4;
```
NOTE: Variables can only be declared once. future changes to a variable can be made if u ignore bag
EG

```
bag foo = 4;
foo = 5;
```
variables can NOT be declared twice EG

```
bag foo = 4;
bag foo = 5;
```

is INVALID

<h2>PRINTING</h1>
```
bag foo = 4;
yap(foo + (5 * 5));
```
NOTE: a semicolon represents end of a line
<H2>IF/ELSE</h2>
```
bag a = -5;
a = a + 10;

if a == 5 {
  yap("YAY");
}

else {
  yap("aww");
}
```
expected output:

YAY

while loops are in this language as well. So as string concatenation
In this language, true is fax, and false is cap. "!" (or not) is no.

So if you want to say !false, you say, no cap, which is the same as true, which is false
EG:
```
yap(no cap);
```
Expected output:

fax.

There's also: while loops

<h2>OTHER EXAMPLES</h2>
```
bag a = 5;
yap(a == 5 and 5 != 4);
yap(a > 3 or a > 10);
yap(no no no cap);
```

OUTPUT:<br>
fax<br>
fax<br>
fax




