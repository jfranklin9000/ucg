+++
title = "Expressions"
weight = 3
sort_by = "weight"
in_search_index = true
+++
Ucg expressions can reference a bound name, do math, concatenate lists or
strings, copy and modify a struct, or format a string.

Symbols
-------

Many UCG expressions or statements use a symbol. A symbol might be used as
either a name for a binding or a name for a field. Symbols must start with an
ascii letter and can contain any ascii letter, number, `_`, or `-` characters.

### The environment symbol

There is a special symbol in UCG for obtaining a value from the environment.
The `env` symbol references the environment variables in environment at the
time of the build. You reference an environment variable just like it was in a
tuple. By default, attempting to reference a variable that doesn't exist will
be a compile error. You can turn this behavior off with the `--nostrict`
argument to the compiler. When in nostrict mode nonexistent variables will
result in a warning and be set to the NULL empty value.

```
let env_name = env.DEPLOY_ENV;
```

Binary Operators
----------

UCG has a number of binary infix operators. Some work only on numeric values and others
work on more than one type.

### Selector operators

The UCG selector operator `.` selects a field or index from tuples or lists.
They can descend arbitrarily deep into data structures. 

You can reference a field in a tuple by putting the field name after a dot. You
can index into a list by referencing the index after the `.`. Lists are always
0 indexed.

```
let tuple = {
    inner = {
        field = "value",
    },
    list = [1, 2, 3],
    "quoted field" = "quoted value",
};

// reference the field in the inner tuple in our tuple defined above.
tuple.inner.field;

// reference the field in the list contained in our tuple defined above.
tuple.list.0;
```

Selectors can quote fields if there are quoted fields with spaces in the tuple.

```
tuple."quoted field";
```

### Numeric Operators

UCG supports the following numeric operators, `+`, `-`, `*`, `/` Each one is type safe 
and infers the types from the values they operate on. The operators expect both the 
left and right operands to be of the same type.

```
1 + 1;
1.0 - 1.0;
```

### Concatenation

The `+` operator can also do concatenation on strings and lists. As with the numeric
version both sides must be the same type, either string or list.

```
"Hello " + "World"; // "Hello World"
[1, 2] + [3]; // [1, 2, 3]
```

### Comparison Operators

UCG supports the comparison operators `==`, `!=`, `>=`, `<=`, `<`, `>`, and `in`.
They all expect both sides to be of the same type.

The `>`, `<`, `>=`, and `>=` operators are only supported on numeric types
(i.e. int, and float).

```
1 > 2; // result is false
2 < 3; // result is true
10 > "9"; // This is a compile error.
(1+2) == 3;
```

The equality operators `==` and `!=` are supported for all types and will
perform deep equal comparisons on complex types.

```
let tpl1 = {
  foo = "bar",
  one = 1
};
let tpl2 = {
  foo = "bar",
  one = 1
};
tpl1 == tpl2; // returns true
let tpl2 = {
  foo = "bar",
  one = 1
  duck = "quack",
};
tpl1 == tpl3; // returns false
```

Because tuples are an ordered set both tuples in a comparison must have their
fields in the same order to compare as equal.

The `in` operator tests for the existence of a field in a tuple or an element in a
list.

```
let tpl = { foo = "bar" };
foo in tpl; // evaluates to true
"foo" in tpl; // also evaluates to true.
```

Lists do a deep equal comparison when testing for the existence of an element.

```
let lst = [1, "two", {three = 3}];
1 in lst; // evaluates to true;
{three = 3} in lst; // evaluates to true
{three = "3"} in lst; // evaluates to false
{three = 3, two = 2} in lst // evaluates to false
```

### Boolean Operators

UCG has the standard boolean operators: `&&` and `||`. Both of them short
circuit and they require the expressions on each side to be boolean.

```
true && false == false;
false || true == true;
```

In addition to the binary operators `&&` and `||` UCG also has the unary
operator `not` which reverses a boolean value.

```
not true == false;
not false == true;
```

#### Operator Precedence

UCG binary operators follow the typical operator precedence for math. `*` and
`/` are higher precendence than `+` and `-` which are higher precedence than
any of the comparison operators.

**Precedence table**

Higher values bind tighter than lower values.

<table>
<thead><th>Operator</th><th>Precedence</th><th>Description</th></thead>
<tr><td>==</td><td>1</td><td>Equality Comparison</td></tr>
<tr><td>!=</td><td>1</td><td>Inequality Comparison</td></tr>
<tr><td>>=</td><td>1</td><td>Greater Than or Equal</td></tr>
<tr><td><=</td><td>1</td><td>Less Than or Equal</td></tr>
<tr><td><</td><td>1</td><td>Greater Than</td></tr>
<tr><td>></td><td>1</td><td>Less Than</td></tr>
<tr><td>=~</td><td>1</td><td>Regex Match</td></tr>
<tr><td>!~</td><td>1</td><td>Negated Regex Match</td></tr>
<tr><td>in</td><td>2</td><td>Contains field or item</td></tr>
<tr><td>is</td><td>2</td><td>Type check</td></tr>
<tr><td>+</td><td>3</td><td>Sum or concatenation</td></tr>
<tr><td>-</td><td>3</td><td>Subtraction</td></tr>
<tr><td>*</td><td>4</td><td>Product</td></tr>
<tr><td>/</td><td>4</td><td>Division</td></tr>
<tr><td>%%</td><td>4</td><td>Modulus</td></tr>
<tr><td>&&</td><td>5</td><td>And</td></tr>
<tr><td>||</td><td>5</td><td>Or</td></tr>
<tr><td>.</td><td>6</td><td>Dot Selector</td></tr>
</table>

Type test expressions
---------------------

ucg has the `is` operator for testing that something is of a given base type.
The type must be a string literal matching one of:

* `"null"`
* `"str"`
* `"int"`
* `"float"`
* `"tuple"`
* `"list"`
* `"func"`
* `"module"`

```
("foo" is "str") == true;
```

Copy Expressions
----------------

UCG expressions have a special copy expression for tuples. These faciliate a
form of data reuse as well as a way to get a modified version of a tuple. Copy
expressions start with a selector referencing a tuple followed by braces `{}`
with `name = value` pairs separated by commas. Trailing commas are allowed.

Copied expressions can change base fields in the copied tuple or add new
fields. If you are changing the value of a base field in the copy then the new
value must be of the same type as the base field's value. This allows you to
define a base "type" of sorts and ensure that any modified fields stay the
same.

```
let base = {
    field1 = "value1",
    field2 = 100,
    field3 = 5.6,
};

let overridden = base{
    field1 = "new value"
};

let expanded = base{
    field2 = 200,
    field3 = "look ma a new field",
};

let bad = base{
    field1 = 300, // Error!!! must be a string.
};

```

There is a special selector that can be used in a copy expression to refer to
the base tuple in a copy called `self`. `self` can only be used in the body of
the copy.

```
let nestedtpl = {
    field1 = "value1",
    inner = {
        field2 = 2
        inner = {
            field3 = "three",
        },
    },
};

let copiedtpl = nestedtpl{
    inner = self.inner{
        inner = self.inner{
            field4 = 4,
        },
    },
};
```

Import Expressions
------------------

Import expressions bring in a ucg file and expose their bound values as a tuple
in the current file. Import expressions are idempotent and cached so you can
use them more than once in a file safely. Import expressions start with the `import`
keyword and are followed by a string containing the path of the file to import.

```
// You can import an entire file into the namespace.
let imported = import "some_file.ucg";

// Or you can just import a single value from that file.
let imported_val = (import "some_file.ucg").val;
```

Format Expressions
----------

UCG has a format expression that has a limited form of string templating. Format expressions come in two forms.

The simplest form starts with a string followed by the `%` operator and a list
of arguments in parentheses separated by commas. Trailing commas are allowed.
The format string should have `@` characters in each location where a value
should be placed. Any primitive value can be used as an argument.

```
"https://@:@/" % (host, port)
```

A slightly more complex form starts with a string followed by the `%` operator
and an unparenthesized expression. the template string can then reference the
result of the expression in expressions embedded within the format string. The
expressions result can be referenced using the special name `item` in the
embedded expression. The result of the expression will be rendered as the
default string representation in the resulting string.

```
let tpl = {
    foo = {
        bar = [0, 1, 2],
    },
};

"foo.bar.1 == @{item.foo.bar.1}" % tpl;
```

If the `%` operator is followed by a parenthesized expression it will be treated
as the first form with one item.

Range Expression
----------------

UCG can generate lists from a range with an optional step.

```
1:10 == [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
0:2:10 == [0, 2, 4, 6, 8, 10];
```

Functions
-----

Functions close over the environment up to the point where they are declared in
the file. One consequence of this is that they can not call themselves so
recursive functions are not possible. This is probably a feature. They are
useful for constructing tuples of a certain shape or otherwise promoting data
reuse. You define a function with the `function` keyword followed by the
arguments in parentheses, a `=>`, and then a valid expression.

```
let myfunc = func (arg1, arg2) => {
    host = arg1,
    port = arg2,
    connstr = "couchdb://@:@" % (arg1, arg2),
};

let my_dbconf = myfunc("couchdb.example.org", "9090");

let my_dbhost = dbconf.host;

let add = func (arg1, arg2) => arg1 + arg2;
add(1, 1) == 2;
```

Functional processing expressions
---------------------------------

UCG has a few functional processing expressions called `map`, `filter`, and
`reduce`. All of them can process a string, list, or tuple.

Their syntax starts with either `map` `filter`, or `reduce` followed by a symbol
that references a valid func and finally an expression that resolves to either
a list or a tuple.

### Map expressions

Map functions should produce either a valid value or a list of [field, value] that
will replace the element or field it is curently processing.

**Lists**

When mapping a function across a list the result field can be any valid value. The
function is expected to take a single argument.

```
let list1 = [1, 2, 3, 4];

let mapper = func (item) =>  item + 1;
map(mapper, list1) == [2, 3, 4, 5];
```

**Tuples**

Functions for mapping across a tuple are expected to take two arguments. The first
argument is the name of the field. The second argument is the value in that
field. The result should be a two item list with the first item being the new
field name and the second item being the new value.

```
let test_tpl = {
    foo = "bar",
    quux = "baz",
};
let tpl_mapper = func (name, val) =>  select name, [name, val], {
    "foo" = ["foo", "barbar"],
    quux = ["cute", "pygmy"],
};
map(tpl_mapper, test_tpl) == {foo = "barbar", cute = "pygmy"};
```

**Strings**

```
let string = "foo";
let string_mapper = func (item) =>  item + item;

map(string_reducer, 0, string) == "ffoooo";
```

### Filter expressions

Filter expressions should return a field with false or NULL for items to
filter out of the list or tuple. Any other value in the return field results in
the item or field staying in the resulting list or tuple.

**Lists**

```
let list2 = ["foo", "bar", "foo", "bar"];
let filtrator = func (item) => select item, NULL, {
    foo = item,
};

filter(filtrator, list2) == ["foo", "foo"];
```

**Tuples**

```
let test_tpl = {
    foo = "bar",
    quux = "baz",
};
let tpl_filter = func (name, val) =>  name != "foo";
filter(tpl_filter, test_tpl) == { quux = "baz" };
```

**Strings**

```
let string = "foo";
let string_filter = func (item) =>  item != "f";

filter(string_reducer, 0, string) == "oo";
```

### Reduce expressions

Reduce expressions start with the reduce keyword followed by a symbol
referencing a func, an expression for the accumulator, and finally the tuple,
list, or string to process.

**Tuples**

```
let test_tpl = {
    foo = "bar",
    quux = "baz",
};
let tpl_reducer = func (acc, name, val) =>  acc{
    keys = self.keys + [name],
    vals = self.vals + [val],
};

reduce(tpl_reducer, {keys = [], vals = []}, test_tpl) == {keys = ["foo", "quux"], vals = ["bar", "baz"]};
```

**Lists**

```
let list1 = [1, 2, 3, 4];
let list_reducer = func (acc, item) =>  acc + item;

reduce(list_reducer, 0, list1) == 0 + 1 + 2 + 3 + 4;
```

**Strings**

```
let string = "foo";
let string_reducer = func (acc, item) =>  acc + [item];

reduce(string_reducer, 0, string) == ["f", "o", "o"];
```

Include expressions
-------------------

UCG can include the contents of other files as an expression. Currently we only
support strings and base64 encoding but we plan to support yaml, and json in
the future. include expressions start with the `include` keyword a type
(currently only `str`), and a path. Relative paths are calculated relative to
the including file.

```
let script = include str "./script.sh";
```

Conditionals
----------

UCG supports a limited conditional expression called a select. A select
expression starts with the `select` keyword and is followed by a an expression
resolving to a string or boolean naming the field to select, an optional
expression resolving to the default value, and finally a tuple literal to
select the field from. If the field selected is not in the tuple then the
default value will be used. If no default is specified then select will
throw a compile failure for the unhandled case.

```
let want = "baz";

//     field  default
select want, "quux", {
    baz = "foo",
    fuzz = "bang",
}; // result will be "foo"

//     field    default
select "quack", "quux", {
    baz = "foo",
    fuzz = "bang",
}; // result will be "quux"

let ifresult = select true, {
    true = "true result",
    false = "false result",
}; // result will be "true result"
```

Modules
-------

UCG has another form of reusable execution that is a little more robust than
functions are. Modules allow you to parameterize a set of statements and build
the statements later. Modules are an expression. They can be bound to a value
and then reused later. Modules do not close over their environment but they can
import other UCG files into the module using import statements including the
file they are located themselves. This works since the statements in a module
are not evaluated until you attempt to call the module with a copy expression.

Module expressions start with the module keyword followed by a tuple
representing their parameters with any associated default values. The body of
the module is separated from the parameter tuple by the `=>` symbol an optional
`(expr)` defining the out expression, and a series of statements delimited by
`{` and `}` respectively. The body of the module can contain any valid UCG
statement. You instantiate a module via the copy expression. By default if
there is no out expression then the module will export all of the named
bindings in the statements.

```
let top_mod = module {
    deep_value = "None",
} => {
    let shared_funcs = import "shared.UCG";

    let embedded_def = module {
        deep_value = "None",
    } => {
        let value = mod.deep_value;
    };

    let embedded = embedded_def{deep_value = mod.deep_value};
};

let embedded_with_params = top_mod{deep_value = "Some"};

// By default all of the named bindings in a module are exported so we can
// get the embedded tuple out via a selector.
embedded_with_params.embedded.value == "Some";
```

### Return Expressions

If there is a return expression then the module will only export the result of
that expression. The out expression is computed after the last statement in the
module has been evaluated.

```
let top_mod_out_expr = module {
    deep_value = "None",
} => (embedded) { // we will only expose the embedded binding in our module
    let shared_funcs = import "shared.UCG";

    let embedded_def = module {
        deep_value = "None",
    } => {
        let value = mod.deep_value;
    };

    let embedded = embedded_def{deep_value = mod.deep_value};
};

let embedded_default_params = top_mod_out_expr{};

// We don't have to dereference the embedded binding since the out expression
// exported it for us.
embedded_default_params.value == "None";
```

### Module builtin bindings

One consequence of a module being able to import the same file they are located in
is that modules can be called recursively. They are the only expression that is
capable of recursion in UCG. Recursion can be done by importing the module's file
inside the module's definition and using it as normal.

Modules have ia recursive reference to the current module `mod.this` that can
be used for recursive modules.

```
let recursive = module {
    counter=1,
    stop=10,
} => (result) {
    let result = select mod.counter != mod.stop, {
        true = [mod.start] + mod.this{counter=mod.counter+1},
        false = [mod.start],
    };
};

recursive{} == [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
```

There is also a convenience function `mod.pkg` in the mod binding for a module.
That imports the package/file the module was declared in. This binding is only
present if the module was declared in a file. Modules created as part of an
eval will not have it.

```
let self_importer = module {
    item=NULL,
} => () {
    let pkg = mod.pkg();

    let result = pkg.recursive{};
};

self_importer{} == [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
```


Fail Expression
---------------

UCG has a way to declaratively trigger a build failure using the `fail` expression.

Fail expressions start with the `fail` keyword and are followed an expression
that must resolve to a string with the build failure message.

```
fail "Oh No This was not what we wanted!";

fail "Expected foo but got @" % ("bar");
```

Trace Expression
----------------

UCG has a debugging expression that can be helpful to trace values while developing called the trace expression.

Trace expression are any valid expression preceded by the `TRACE` keyword. Trace
expression return the result of the expression unchanged but they also output a
trace statement to stderr printing the result of the expression as well as the
file, line and column where the expression was.

```
let mk_list = func(a, b) => TRACE [a, b];
mk_list(1, 2);
```

This will output a line to stderr something like the below:

    TRACE: [1, 2] at file: <file name> line: 1 column: 29

This is helpful when developing shared modules or ucg libraries.

Next: <a href="/reference/statements">Statements</a>