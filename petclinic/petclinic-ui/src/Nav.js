import React, { useState } from 'react';

export default function Nav(props) {

  const [active, setActive] = useState('owners')

  const changePage = (page) => {
    setActive(page);
    props.changePage(page);
  }

  const isActive = (link) => {
    return active === link ?
      'py-4 px-2 text-blue-500 border-b-4 border-blue-500 font-semibold cursor-pointer'
      :
      'py-4 px-2 text-gray-500 font-semibold hover:text-blue-500 transition duration-300 cursor-pointer'

  }

  return (
    <nav className="bg-white shadow-lg">
      <div className="container mx-auto">
        <div className="flex justify-between">
          <div className="flex space-x-7">
            <div className="hidden md:flex items-center space-x-1">
              <h1 className="font-semibold text-gray-500 pr-4">Pet Clinic</h1>
              <div onClick={() => changePage('owners')} className={isActive('owners')}>Owners</div>
              <div onClick={() => changePage('vets')} className={isActive('vets')}>Vets</div>
              <div onClick={() => changePage('petTypes')} className={isActive('petTypes')}>PetTypes</div>
            </div>
          </div>
        </div>
      </div>
    </nav>
  )
}