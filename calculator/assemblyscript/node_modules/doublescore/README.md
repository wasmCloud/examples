# doublescore

A natively (TypeScript native at least) written, no external dependency utility library.

# Version 1.0.0 Changes

The native code for doublescore has moved to TypeScript. JavaScript libraries can still use this library as always. But, 
TypeScript based libraries will now have a much easier time using it. With the exception of one removed feature set, 
DoubleScore should work in place for both JS and TS based projects. Nevertheless, TypeScript transpiling is a big change
so we bumped the major version to 1, and also officially pushed it over the line to full blown production grade.

### Removed Features 
 
The following features were removed in 1.0.0:

- Close: All close functionality for managing callback SLAs have been removed. This was a non-used feature, and is 
slightly too specific for the overall purposes of DoubleScore. DoubleScore aims to be a utility for managing data 
structures. Managing callbacks is not within that scope.


# Usage 

## Including the Library

### TypeScript
```typescript
import * as  __  from 'doublescore';

```

### JavaScript
```javascript
const __ = require( 'doublescore' );
```

These are the available utility functions.



## iterate()

Iterate enumerates the leaves of an n-dimensional data structure in depth first order. The first parameter will be the 
value of the leaf, the second parameter will be the index path of the leaf. This is very similar to Array.forEach(). The
only difference is that iterate passes an array for index because the input can be n-dimensional, with nested objects.

Note that you can pass multiple objects to the method so iterate is actually traversing the arguments "array", and the 
index to that is the first index field.

```typescript

__( [ 0, { one: 1 }, [ 2, 3 ] ], [ 4, 5 ] ).iterate( ( value, path ) => {
	/** is called 4 times with the params value/path:
	 * 0/[0, 0]
	 * 1/[0, 1,'one']
	 * 2/[0, 2, 0]
	 * 3/[0, 2, 1]
	 * 4/[1, 0]
	 * 5/[1, 1]
	 */
} );

__( ['one', 'two', 'three' ], [ 'four', 'five' ] ).iterate( ( value, path ) => {
	/** is called 5 times with the params value/path:
	 * 'one'/[0, 0]
	 * 'two'/[0, 1]
	 * 'three'/[0, 2]
	 * 'four'/[1, 0]
	 * 'five'/[1, 1]
	 */
} );

```

## iterate.flatten()

Flatten is kin to Iterate, hence the chaining. Flatten is basically the result of listing all values returned by iterate 
and ignoring path. For example, consider the output of the following code.

```typescript

const values = __.iterate.flatten(
            [ 0, 1, 2, 3 ],
            [ 'a', 'b', 'c' ],
            [ { one: 'two', three: -3, four: false }, 'b', 'c' ],
            [],
            [ -1, null ],
            [ 'a', [ false, true, -1, null ], 'c' ]
          );

console.log( values );

/**
  * Outputs:
  * [ 0, 1, 2, 3, 'a', 'b', 'c', 'two', -3, false, 'b', 'c', -1, null, 'a', false, true, -1, null, 'c' ]
*/

```



## isObject() 

Will return TRUE for anything that is typeof object, is not an Array, and is not NULL.


```typescript
__.isObject( new Date() ); // true
__.isObject( {} ); // true
__.isObject( null ); // false
__.isObject( "hello" ); // false
__.isObject( Infinity ); // false
__.isObject( 5 ); // false
__.isObject( true ); // false
__.isObject( false ); // false
__.isObject( [] ); // false

```


## getType() 

This expands on the resolution of the native ```typeof <var>``` operation. 

```typescript

__.getType( null ); // 'null'
__.getType( undefined ); // 'undefined'
__.getType( [] ); // 'array'
__.getType( new Array() ); // 'array'
__.getType( new Date() ); // 'date'
__.getType( new RegExp( 'anything' ) ); // 'regex'
__.getType( parseInt('not-a-number') ); // 'not-a-number'
__.getType( Math.pow( 10, 1000 ) ); // 'infinity'
__.getType( 1.1 ); // 'float'
__.getType( -1.1 ); // 'float'
__.getType( 1 ); // 'integer'
__.getType( 0 ); // 'integer'
__.getType( 0.0 ); // 'integer' What?! I know, this doesn't make sense but after compiling Javascript sees this the same as 0
__.getType( -1 ); // 'integer'
__.getType( 1n ); // 'integer' Technically a BigInt, but that's not official, so not supporting it yet

// All other values not covered by one of the cases above and falls back to returning typeof <var>

```



clone() 

Will return a distinct copy of the object it was called from. Depending on the value type it is handled differently.

Primitives are natively copied/cloned anytime you assign them or pass them to a method/function. This is the case for 
strings, numbers, boolean, null, etc.

Simple objects and arrays are recursively cloned. 

Date objects are cloned. 

Everything else is passed through directly. If is is natively copied/cloned by assignment, it will be cloned, otherwise
it will be carried over to the destination. Functions, for example, will be copied to the destination.

The performance of this is better than JSON.parse( JSON.stringify( thing ) ).

```typescript

function goodFunctionalProgramming( obj: SomeType ) {
  
  obj = __.clone( obj );
  
  // modify obj
  
  return obj;
  
}

```

mixin() 

Applies clone() to params and then recursively mixes all values into default. If types do not match, params type is 
taken. mixin() accepts an arbitrary number of arguments, they will be mixed in from left to right. That is, if the same
path exists in two arguments, the furthest right argument will have its value kept. 

This method is an excellent way to ensure defaults or overrides recursively.

```typescript

function doSomething  ( params ) {

  const defaults = {
    deep: [
      {
        merge: 'on this default object of'
      }
    ],
    strings: {
      objects: 'and',
      numbers: 1.5
    },
    and: null,
    etc: [ false, false ],
    from: 'params'
  };
  
  const overrides = {
    boolean: false
  };

  // sets defaults and overrides recursively
	params = __( defaults ).mixin( params, overrides );
	

	// handle params... 

}

```

timer() 

Will return a function that will track time deltas with 1ms resolution and < 0.5% error on average. The returned 
function accepts a single parameter: reset, which if true will reset the timer, returning the last interval.

Note: For intervals of < 10ms, timer() is likely not accurate enough. If you want to experimentally measure the time of 
something that takes <10ms to run once, you should run it N times, measuring total execution time, then divide by N. N
should be something large like 100.
 
```typescript

const timer = __.timer();

// wait 100ms

timer(); // returns 100 +- 0.5%

// wait 50ms

timer(); // returns 150 +- 0.5%

// wait 300ms

timer( true ); // returns 450 +- 0.5%

// wait 200ms

timer(); // returns 200 +- 0.5%

// wait 175ms

timer(); // returns 375 +- 0.5%


```

# More examples

Please see the test cases in test/* for extensive examples of all supported use cases. Test cases are available in the
Git repo for this code base.


# License

(The MIT License)

Copyright (c) 2019 BlueRival Software <support@bluerival.com>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation
files (the 'Software'), to deal in the Software without restriction, including without limitation the rights to use, copy,
modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
