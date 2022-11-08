import React, { useState } from 'react';

import ErrorBoundary from './ErrorBoundary';
import Nav from './Nav';
import Owners from './Owners';
import PetTypes from './PetTypes';
import Vets from './Vets';

function App() {
  const [page, setPage] = useState('owners')

  const renderPage = () => {
    switch (page) {
      case 'owners':
        return <Owners />;
      case 'vets':
        return <Vets />;
      case 'petTypes':
        return <PetTypes />;
      default:
        return <Owners />;
    }
  }

  return (
    <div>
      <Nav changePage={(page) => setPage(page)} />
      <div className="container mx-auto pt-8 relative">
        <ErrorBoundary>
          <img src="/images/cat.png" className="absolute top-0 sm:-left-[37%] -left-[40%] opacity-75 -z-10" />
          <img src="/images/dog.png" className="absolute sm:-right-[35%] top-1/2 -right-[38%] opacity-75 -z-10" />
          {renderPage()}
        </ErrorBoundary>
      </div>
    </div>
  );
}

export default App;
