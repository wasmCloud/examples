import React, { useState } from 'react';

export default function Nav(props) {

  const [active, setActive] = useState('owners')

  const changePage = (page) => {
    setActive(page);
    props.changePage(page);
  }

  const isActive = (link) => {
    return active === link ?
      'py-4 px-2 text-[#00bc8e] border-b-4 border-[#00bc8e] font-semibold cursor-pointer'
      :
      'py-4 px-2 text-wasmcloudGray font-semibold hover:text-[#00bc8e] transition duration-300 cursor-pointer'

  }

  return (
    <nav className="bg-white shadow-lg sticky top-0 z-20">
      <div className="container mx-auto">
        <div className="flex">
          <img src="/images/wasmcloud_logo.svg" className="p-1" height="40%" width="15%" />
          <h1 className="font-semibold text-gray-500 text-2xl my-auto pl-4">Pet Clinic</h1>
          <div className="flex space-x-6 ml-[15%]">
            <div
              onClick={() => changePage('owners')}
              className={`${isActive('owners')} flex`}
            >
              <i className="fa fa-user text-wasmcloudGreen-light my-auto mr-2" />
              Owners
            </div>

            <div
              onClick={() => changePage('vets')}
              className={`${isActive('vets')} flex`}
            >
              <i className="fa fa-notes-medical text-wasmcloudGreen-light my-auto mr-2" />
              Vets
            </div>

            <div
              onClick={() => changePage('petTypes')}
              className={`${isActive('petTypes')} flex`}
            >
              <i className="fa fa-paw text-wasmcloudGreen-light my-auto mr-2" />
              Pet Types
            </div>
          </div>
        </div>
      </div>
    </nav>
  )
}