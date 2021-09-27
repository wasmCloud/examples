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
      <div className="container mx-auto pt-8">
        <ErrorBoundary>
          {renderPage()}
        </ErrorBoundary>
      </div>
    </div>
  );
}

export default App;
