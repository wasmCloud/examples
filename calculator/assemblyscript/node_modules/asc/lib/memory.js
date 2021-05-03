'use strict';

const __ = require( 'doublescore' );
const async = require( 'async' );
const util = require( './util' );

class Memory {

  constructor( params ) {

    this._init( params );
    this._setup();

  }

  _init( params ) {

    this._params = __( {
      ttl: 60000
    } ).mixin( params || {} );

    if ( !__.isNumber( this._params.ttl ) ) {
      throw new Error( 'memory.ttl must be a number' );
    }

    if ( this._params.ttl < 0 ) {
      throw new Error( 'memory.ttl must be >= 0' );
    }

  }

  _setup() {

    this._memoryCache = {};
    this._memoryCacheTimeouts = {};

  }

  get( key, done ) {

    key = util.marshallKey( key );

    if ( this._memoryCache.hasOwnProperty( key ) ) {
      return done( null, this._memoryCache[ key ] );
    }

    done( new Error( 'not found' ) );

  }

  set( key, value, done ) {

    key = util.marshallKey( key );

    async.waterfall( [
      ( done ) => {
        this._memoryClear( key, done );
      },
      ( done ) => {
        this._memoryCache[ key ] = value;

        setTimeout( () => {
          this._memoryClear( key, () => {
            // NO-OP
          } );
        }, this._params.ttl );

        done();
      }
    ], done );

  }

  clear( key, done ) {

    if ( this._memoryCacheTimeouts.hasOwnProperty( key ) ) {
      clearTimeout( this._memoryCacheTimeouts[ key ] );
    }

    delete this._memoryCacheTimeouts[ key ];
    delete this._memoryCache[ key ];

    done();

  }

  _memoryClear( key, done ) {

    if ( this._memoryCacheTimeouts.hasOwnProperty( key ) ) {
      clearTimeout( this._memoryCacheTimeouts[ key ] );
    }

    delete this._memoryCacheTimeouts[ key ];
    delete this._memoryCache[ key ];

    done();

  }

}

module.exports = Memory;
