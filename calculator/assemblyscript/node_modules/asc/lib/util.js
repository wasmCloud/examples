'use strict';

module.exports = {
  marshallKey: function ( key ) {
    return JSON.stringify( key );
  },
  unmarshallKey: function ( key ) {
    return JSON.parse( key );
  }
};
