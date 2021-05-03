# ASC (pronounced "ASK")

ASC is a middleware layer between a set of cacheable services and a client. It allows the developer to define caching tiers
by declaring layers which ASC will asynchronously update and query as needed. Hence, ASC stands for Asynchronous 
Self-Updating Cache. 

The services defined in each layer could be an in-process library, a file, an external web service, anything. Any time 
the results from a resource call can be cached, you can use ASC as a proxy to that resource.


# New 2.x

This is a major re-write to the previous version. The previous version is 1.x

Here are the changes to ASC since 1.x 

* Tiered cache support: Add multiple cache layers. In addition to the built-in, in-memory cache, add Redis, Memcached, 
DynamoDB, S3, whatever you want. Each caching layer you define has a get, set and clear methods. Do whatever you want 
with each layer. The first layer to return a hit will populate all cache layers above it.

* In-memory cache is optional: If you don't want to use the in-memory cache, it can be disabled. 

* Dropped batch key lookup. (We may bring it back if there is demand)

* Dropped the global factory. (We won't bring this back, its an anti-pattern to create global storage at the process 
level)


# Why use it?

Tiered storage of data is a key solution for scale. Typically data is stored at many levels between user access and 
permanent cold storage. Also, these tiers of storage are assembled at an infrastructure level, not an application level.
The impact is considerable overhead between developers and infrastructure admins to develop a solid system.

Developers have to inform admins how to cache data: which fields to cache on, what data to cache, how to cache it, how
long to cache it, etc. The aim of ASC is to move most of that conversation away from developers <-> admins, and just
between developers <-> developers.

ASC will handle: 

* Traversing cache layers until a tier returns a hit for a key get().
* Propagating a hit up the layers with a set().
* De-duplicating parallel get() requests for the same key. If a key is requested, and a request is already in flight for
that key, the first request will return data to all new, in-coming get() calls for the same key.
* Being really fast and predictable.

# When NOT to use it?

* You can't have your data cached, need latest data every time.
* Your data can't be serialized, because its an object containing resources, and not just arrays, objects and scalars.
* Your keys can't be serialized using JSON.stringify. 

# API

The ASC module exports a single class. Just instantiate it with ```new```. This is the full JSDoc for the constructor
function.

```js
	 /**
     * Constructor Data Types
     *
     * @typedef MemoryParams
     * @type {object}
     * @property {boolean} [disabled=false] If TRUE, the in-memory cache is disabled
     * @property {number} [ttl=60000] The TTL in milliseconds for the in-memory cache
     *
     * @callback DataCallback
     * @param {Error} err - An instance of Error if an error occurred, null otherwise
     * @param {any} [data] - Any data, MUST only be omitted if err is set.
     *
     * @callback ErrorOnlyCallback
     * @param {Error} [err] - An instance of Error if an error occurred, empty or null otherwise
     *
     * @callback GetCallback
     * @param {string} key - The key of the data to return
     * @param {DataCallback} - The done callback to pass an error or data to
     *
     * @callback SetCallback
     * @param {string} key - The key of the data to return
     * @param {any} data - The data to set for respective key
     * @param {ErrorOnlyCallback} - The done callback
     *
     * @callback ClearCallback
     * @param {string} key - The key of the data to return
     * @param {ErrorOnlyCallback} - The done callback
     *
     * @typedef CacheLayer
     * @type {object}
     * @property {DataCallback} get The get function
     * @property {SetCallback} [set] The set data function
     * @property {ClearCallback} [clear] The clear function
     *
     * @typedef ConstructorParams
     * @type {object}
     * @param {MemoryParams} [ConstructorParams.memory]
     * @param {[CacheLayer]} [ConstructorParams.layers]
     * @param {GetCallback} [ConstructorParams.get]
     *
     */
  
    /**
     * Return an instance of ASC class.
     *
     * @param {ConstructorParams|[CacheLayer]|GetCallback} params Can either be a configuration object, an array of just
     * layers, or just a single get function for handling in-memory misses.
     *
     */
	const ASC = require( 'asc' );
	const cache = new ASC( params );
```

# Layers, Diggity-Layers and All That

The new tiering logic defines each storage tier in layers. The potential layers are:

* The in-memory cache
* Any user-defined layers
* The user-defined get method (called if in-memory cache, and all user-defined layers miss on a key)

Layers are prioritized in the order they are defined. The in-memory cache is always top of the list, then all the 
user-defined layers are next, in the same order they are passed to the ASC constructor. Lets look at the most expressive
way to define several the layers.

Consider this code.

```js

const ASC = require( 'asc' );

const fancyCache = new ASC( {
  memory: { // in-memory layer
    disabled: false, // this is the default, but we set it explicitly for illustration purpose here
    ttl: 60000 // 60 seconds
  },
  layers: [
    { // layer 1
      get: ( key, done ) => {
        
        // call some service, maybe Redis, and get the data for key, then return it
        done( null, dataFromRedis ); // success
        
        // if there is no data, or any other error occurs, return an instance of Error
        done( new Error( 'not found' ) );
        
      },
      set: ( key, dataForRedis, done ) => { // optional, but its a nice way to propagate data up the chain on misses
        
        // call some service, maybe Redis, and store the data for key
        done(); // success
        
        // if there is an error, or for any reason that data can not be stored, return an instance of Error.
        done( new Error( 'some API failure' ) );
        
      },
      clear: ( key, data, done ) => { // also optional, but not if you want to force this layer to delete a key
        
        // call some service, maybe Redis, and clear the data for key
        done();
        
        // if there is an error, return an instance of Error. If the key doesn't exist in this layer, that is not 
        // considered an error. Anything that prevents a key from clearing when it does exist is an error.
        done( new Error( 'some API failure' ) );
                
      }
    },
    { // layer 2
      get: ( key, done ) => {
        
        // call some service, maybe DynamoDB, and get the data for key
        done( null, dataFromDynamoDB );
        
      },
      set: ( key, dataForDynamoDB, done ) => {
        
        done(); // success
        
      },
      clear: ( key, data, done ) => {
        
         done(); // success
         
      }
    },
    { // layer 3
      get: ( key, done ) => {
        
        // query some heavy process, maybe a SQL report, maybe a 3rd party API across the inter-webs, maybe do some
        // heavy number crunching, etc.
        done( null, someExpensiveDataToGenerate );
        
      }
      // no set or clear, because this is layer is ground truth. If it can't return data, it doesn't exist, or there 
      // is an error.
    }
  ]
} );


```

Here, the priority of the layers are:

* In-Memory
* Layer 1
* Layer 2
* Layer 3

When ```fancyCache.get( key... )``` is called, ASC will first check memory, then call get() on layer 1, then on layer 2,
finally on layer 3. As soon as one of the layers fires the callback with ```null``` as the error argument will end the
waterfall. Each layer above the layer that finally returned data will have it's set() method called, to populate that
layer. If you don't want a layer to have data set() after a cache miss, simply don't provide a set() method. Or, provide
a set() method and then ignore calls depending on the key or any other state your layer cares about. If you do provide
a set() method, you MUST call done to prevent hanging.

Here is a more concrete example relative to the code above. Let's say a call for data on a key is made. That key is 
missing from memory layer and layer 1. But, the key does have data in layer 2. The following methods will be called:

fancyCache.get() // called
memory.get() // miss
layer1.get() // miss
layer2.get() // hit
layer1.set() // propagate data
memory.set() // propagate data
fancyCache.get() // returned data to callback

The advantage to this approach is that it allows the application to determine how to migrate data between cache layers,
with priority tiering. 

In this example we suggested that layer 1 is a Redis cluster, and layer 2 is DynamoDB global tables cluster, and layer 3
is ground truth/source for the data. In a geographically distributed deployment this logic allows for seamless data 
migration and caching throughout your architecture with little thought as to where your code is running. With this setup
you could easily have a global deployment like this:

```text

- Ground Truth of Expensive Data Origination // Infinite cache, ground truth, never expires, etc
  - DynamoDB Global Table // Set the longest possible TTL on DynamoDB records that makes sense for your data
  
    - Redis Cluster in Tokyo DataCenter // Set a TTL that is much shorter than the DynamoDB TTL, probably minutes
      - Your application instances in Tokyo // Set a TTL that is much shorter than the Redis TTL, probably seconds
      
    - Redis Cluster in US East DataCenter
      - Your application instances in US East
      
    - Redis Cluster in US West DataCenter
      - Your application instances in US West
      
    - Redis Cluster in Ireland DataCenter
      - Your application instances in Ireland

```

With this globally deployed architecture your entire infrastructure has the same kind of caching logic and layers you find 
within a modern CPU and motherboard. CPUs have multiple layers of very fast, but very short TTL caches, then system 
memory which is larger, but slower, followed by disk (and sometimes network storage) which is very slow but is permanent
 ground truth.
 
There are of course other benefits to ASC which apply to other use cases that are much simpler than a globally deployed
application. See the section "Common Use Cases with Examples" for additional examples.


# Warning on the In-Memory Cache

The built-in memory cache is not a magical box of unlimited storage. This cache will consume the primary memory in your
process. Node.js running on 32-bit systems has a maximum stack of 512MB and on 64-bit systems it is 1GB.

It is up to you to determine a good TTL too prevent your process from running out of memory. If you don't want to use 
any memory, no worries, just disable it.

```js

const ASC = require( 'asc' );

const fancyCache = new ASC( {
  memory: {
    disabled: true
  }, ...
} );

``` 

## Common Use Cases with Examples

### Simple In Memory Caching Only

This is the base case. It is just use the built-in memory cache which stores data in memory. The default TTL is used 
here and it is 60,000ms. 

```js

const ASC = require( 'asc' );

const cache = new ASC( ( key, done ) => {
  // call service, or load file, or do whatever, then return data
  done( null, 'some data based on key' );
} );

// If the key is in memory, data will be returned from memory, otherwise the handler function will be called
cache.get( 'some key', ( err, data ) => {
  // do something with the data
} );

```

#### Config shortcuts

The above example is shorthand for defining an in-memory cache with a single layer. Here are 3 other ways to create the
exact same cache.

One
```js
const ASC = require( 'asc' );

const cache = new ASC( {
  get: ( key, done ) => {
    // call service, or load file, or do whatever, then return data
    done( null, 'some data based on key' );
  }
} );
```

Two
```js
const ASC = require( 'asc' );

const cache = new ASC( {
  layers: [
    {
      get: ( key, done ) => {
        // call service, or load file, or do whatever, then return data
        done( null, 'some data based on key' );
      }
    }
  ]
} );
```

Three
```js
const ASC = require( 'asc' );

const cache = new ASC( {
  layers: [
    {
      get: ( key, done ) => {
        // call service, or load file, or do whatever, then return data
        done( null, 'some data based on key' );
      },
      set: ( key, data, done ) => {
        // NO-OP
        done();
      },
      clear: ( key, done ) => {
       // NO-OP
       done();
      }
    }
  ]
} );
```
 
You can use most of the shortcuts with multiple layers too. These two examples are also equivalent caches.

One
```js
const ASC = require( 'asc' );

const cache = new ASC( {
  layers: [
    {
      get: ( key, done ) => {
        done( null, 'layer 1 cache data' );
      }
    }
  ],
  get: ( key, done ) => {
    // call service, or load file, or do whatever, then return data
    done( null, 'some data based on key' );
  }
} );
```

Two
```js
const ASC = require( 'asc' );

const cache = new ASC( {
  layers: [
    {
      get: ( key, done ) => {
        done( null, 'layer 1 cache data' );
      }
    },
    {
      get: ( key, done ) => {
        // call service, or load file, or do whatever, then return data
        done( null, 'some data based on key' );
      }
    }
  ]
} );
```

### Redis Shared Cache

Let's say you use a beefy, well tuned Redis cluster and prefer to use that for caching. Also, lets say you don't want to
use in-memory caching because you are deploying code on low-memory containers. This is a good way to use Redis to cache
data and make it available to all containers in your cluster.  

```js

const ASC = require( 'asc' );
const async = require( 'async' );
const redis = require( 'redis' );

const redisClient =  redis.createClient();

const fancyCache = new ASC( {
  memory: { // disable in-memory layer
    disabled: true
  },
  layers: [
    { // Redis tier 
      get: ( key, done ) => { // see if redis has the key
        
        async.waterfall([
          ( done ) => {
          
            // Redis doesn't support fancy objects as keys, so we just JSON encode the key object
            redisClient.get( JSON.stringify( key ), done );
          
          },
          ( strData, done ) => {
          
            let data = null;
            
            // wrap in try catch in case invalid JSON comes back and causes JSON.parse() to throw an Error.
            try {
              data = JSON.parse( strData ); // Redis doesn't understand JSON objects, so we store data as a string.
            } catch (e) {
              return done( e );
            }
            
            done( null, data );
            
          }
        ], done);
        
        
      },
      set: ( key, dataForRedis, done ) => {
        
        // store data in redis cache for 300 seconds
        async.waterfall( [
          ( done ) => {
          
            // store the data with a timeout of 300 seconds
            redisClient.set( JSON.stringify(key), JSON.stringify(dataForRedis), 'EX', 300, done );
          },
          ( OK, done ) => {
            
            if ( OK !== 'OK' ) {
             return done( new Error('unknown error' ) );
            }
            
            done();
            
          }
        ], done );
        
      },
      clear: ( key, data, done ) => {
        
        // delete the key from redis even if the TTL has not expired
        redisClient.del( JSON.stringify( key ), done );
                
      }
    },
    { // Final tier
      get: ( key, done ) => {
        
        // query some heavy process, maybe a SQL report, maybe a 3rd party API across the inter-webs, maybe do some
        // heavy number crunching, etc.
        done( null, someExpensiveDataToGenerate );
        
      }
      // no set or clear, because this is layer is ground truth. If it can't return data, it doesn't exist, or there 
      // is an error.
    }
  ]
} );


```

# Contributing

If you would like to help with this project, please do the following:

* Fork the project on GitHub.
* Branch your changes from develop to a new branch called `feature/<some unique name for your change>`.
* Ensure you validate your code using the .eslintrc file in this repo.
* Ensure your changes are covered by at least one test in test/.
* Ensure `npm test` passes.
* Issue a pull request from `feature/<some unique name for your change>` back to develop in the main repo.
* Eat a sandwich. 


License
========

(The MIT License)

Copyright (c) 2019 BlueRival Software <support@bluerival.com>

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the 'Software'), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

