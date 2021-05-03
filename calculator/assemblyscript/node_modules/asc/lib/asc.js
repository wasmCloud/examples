'use strict';

const __ = require( 'doublescore' );
const async = require( 'async' );
const Memory = require( './memory' );
const util = require( './util' );

class ASC {

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
  constructor( params ) {

    this._init( params );
    this._setup();

  }

  _init( params ) {

    // clamp to defaults
    params = typeof params === 'function' ? { // if params is just the get function, convert it to shortcut get
      get: params
    } : params;
    params = __.isArray( params ) ? { // if params is just the array of layers, convert it to a shortcut layers
      layers: params
    } : params;
    params = __.isObject( params ) ? params : {};
    this._layers = __.isArray( params.layers ) ? params.layers : [];

    // shortcut for cache with no middle layers
    if ( typeof params.get === 'function' ) {
      this._layers.push( {
        get: params.get
      } );
    }

    this._memoryParams = __( {
      disabled: false
    } ).mixin( params.memory || {} );

    if ( this._layers.length < 1 ) {
      throw new Error( 'no caching layers provided' );
    }

  }

  _setup() {

    this._serviceQueues = {};

    this._getLayers = [];
    this._setLayers = [];
    this._clearLayers = [];

    // if memory cache enabled, make it first
    if ( !this._memoryParams.disabled ) {

      delete this._memoryParams.disabled;

      const memory = new Memory( this._memoryParams );

      // prefix memory handler to layers
      this._layers.unshift( {
        get: ( key, done ) => {
          return memory.get( key, done );
        },
        set: ( key, data, done ) => {
          return memory.set( key, data, done );
        },
        clear: ( key, done ) => {
          return memory.clear( key, done );
        }
      } );

    }

    // this handler ignores errors and data returned by the layer
    // this is used for set calls
    const generateSetHandler = ( handler ) => {

      return ( key, data, done ) => {

        handler( key, data, () => {
          // ignore any errors or data the handler returns
          done();
        } );

      };

    };

    // this handler ignores errors and data returned by the layer
    // this is used for clear calls
    const generateClearHandler = ( handler ) => {
      return ( key, done ) => {
        handler( key, () => {
          // ignore any errors or data the handler returns
          done();
        } );
      };
    };

    this._layers.forEach( ( layer, i ) => {

      if ( !__.isObject( layer ) ) {
        throw new Error( 'layer ' + i + ' is not an object' );
      }

      // get function is required
      if ( typeof layer.get !== 'function' ) {
        throw new Error( 'layer ' + i + ' is missing get function' );
      }

      // get is required on each layer
      this._getLayers.push( layer.get );

      // set is not required on any layer, but it makes sense to want to populate each layers cache on a miss.
      // due to other logic, we set a no-op method if the layer doesn't provide one
      let layerSet = layer.set;
      if ( typeof layerSet !== 'function' ) {
        layerSet = ( key, data, done ) => {
          // NO-OP
          done();
        };
      }
      this._setLayers.push( generateSetHandler( layerSet ) );

      // clear is not required on any layer, but also makes sense to want to
      // be able to propagate clears to all layers
      if ( typeof layer.clear === 'function' ) {
        this._clearLayers.push( generateClearHandler( layer.clear ) );
      }

    } );

  }

  /**
   * Gets the corresponding key from the first layer to have the data.
   *
   * @param { any } key Can be any object or scalar, but must be serializable as JSON.
   * @param { any } data Can be anything. The in-memory layer built in to ASC can store anything, including resource handles, but it is up to your layers to be able to handle storage of whatever can be passed here.
   * @param { ErrorDataCallback } done Will call back with no arguments, or first argument will be instance of Error if any of the layers errors out.
   */
  get( key, done ) {

    // only use the marshalled key for ASC callback queues
    // pass original key to all cache layer handlers
    const marshalledKey = util.marshallKey( key );

    if ( !this._serviceQueues.hasOwnProperty( marshalledKey ) ) {
      this._serviceQueues[ marshalledKey ] = [];
    }

    this._serviceQueues[ marshalledKey ].push( done );

    if ( this._serviceQueues[ marshalledKey ].length > 1 ) {
      return;
    }

    let returnData;
    let hasData = false;
    let returnErr = null;
    let currentIndex = 0;

    async.whilst(
      () => !hasData && returnErr === null && currentIndex < this._getLayers.length,
      ( done ) => {

        const handler = this._getLayers[ currentIndex ];

        handler( key, ( err, data ) => {

          // this layer failed to return data, either not found or any other issue
          if ( err instanceof Error ) {

            if ( currentIndex === this._getLayers.length - 1 ) {
              // this was the last layer, take the error
              returnErr = err;
            } else {
              // more layers to try
              currentIndex++;
            }

          } else if ( err ) {

            // this layer returned an invalid value for error
            returnErr = new Error( 'layer ' + currentIndex + ' failed to return an instance of Error, returned: ' +
                                   __.getType( err ) );

          } else {

            // assume this layer had data
            returnData = data;
            hasData = true;

          }

          done();

        } );

      },
      () => {

        const finish = () => {

          // clear out the queue and store it in a local variable so that
          // the callbacks we are about to fire don't create an endless loop if
          // they trigger another lookup on the same key
          const callbacks = this._serviceQueues[ marshalledKey ];
          delete this._serviceQueues[ marshalledKey ];

          // fire all callbacks synchronously, in series
          callbacks.forEach( ( callback ) => {

            // wrap this in a try/cache in case the external code is buggy
            try {

              if ( hasData ) {
                callback( null, returnData );
              } else {
                callback( returnErr );
              }

            } catch ( e ) {
              // NO-OP
            }

          } );

        };

        // if we have data, back-populate up the layers
        // otherwise just start the callbacks
        if ( hasData ) {
          this._populateMisses( key, returnData, currentIndex, finish );
        } else {
          finish();
        }

      }
    );

  }

  _populateMisses( key, data, index, done ) {

    async.timesSeries( index, ( i, done ) => {

      this._setLayers[ i ]( key, data, done );

    }, ( err ) => {

      if ( err ) {
        return done( err );
      }

      done();

    } );


  }

  /**
   * Sets the corresponding key to store the passed data.
   *
   *
   * @param { any } key Can be any object or scalar, but must be serializable as JSON.
   * @param { any } data Can be anything. The in-memory layer built in to ASC can store anything, including resource handles, but it is up to your layers to be able to handle storage of whatever can be passed here.
   * @param { ErrorCallback } done Will call back with no arguments, or first argument will be instance of Error if any of the layers errors out.
   */
  set( key, data, done ) {

    async.applyEachSeries( this._setLayers, key, data, ( err ) => {

      if ( err ) {
        return done( err );
      }

      done();

    } );

  }

  /**
   * Clears the corresponding key.
   *
   * @param { any } key Can be any object or scalar, but must be serializable as JSON.
   * @param { function({Error} err?) } done Will call back with no arguments, or first argument will be instance of Error if any of the layers errors out.
   */
  clear( key, done ) {

    async.applyEachSeries( this._clearLayers, key, ( err ) => {

      if ( err ) {
        return done( err );
      }

      done();

    } );

  }

}

/**
 * This callback only accepts an optional Error, no data
 *
 * @callback ErrorCallback
 *
 * @param {Error} [err]
 */

/**
 * This callback only accepts an optional Error, no data
 *
 * @callback ErrorDataCallback
 *
 * @param {Error} [err]
 * @param {any} data
 */

module.exports = ASC;
