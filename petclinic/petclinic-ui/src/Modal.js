import React, { useState } from 'react';

export function Modal(props) {

  const renderModalHeader = () => {
    return (
      <div className="flex items-start justify-between p-5 border-b border-solid border-blueGray-200">
        <h3 className="text-3xl font-semibold">
          {props.modalTitle}
        </h3>
      </div>
    )
  }

  const renderModalContent = () => {
    return (
      <div className="relative p-6 flex-auto">
        {props.children}
      </div>
    )
  }

  const renderModalFooter = () => {
    return (
      <div className="flex items-center justify-end p-6 border-t border-solid border-blueGray-200">
        <button
          className="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 mb-2"
          type="button"
          onClick={() => props.setShowModal(false)}
        >
          Close
        </button>
      </div>
    )
  }

  return (
    <>
      <div
        className="justify-center items-center flex overflow-x-hidden overflow-y-auto fixed inset-0 z-50 outline-none focus:outline-none"
      >
        <div className="relative w-1/3 my-6 mx-auto ">
          {/*content*/}
          <div className="border-0 shadow-lg relative flex flex-col w-full bg-white outline-none focus:outline-none">
            {renderModalHeader()}
            {renderModalContent()}
            {renderModalFooter()}
          </div>
        </div>
      </div>
      <div className="opacity-25 fixed inset-0 z-40 bg-black"></div>
    </>
  )
}

export function OwnerModal(props) {
  const [owner, setOwner] = useState(props.owner || {})

  const onChange = (e) => {
    setOwner({
      ...owner,
      [e.target.id]: e.target.value
    })
  }

  return (
    <form className="px-8 pt-6 pb-8 mb-4">
      <div className="mb-4">
        <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="firstName">
          FirstName
        </label>
        <input
          value={owner.firstName || ''}
          onChange={(e) => onChange(e)}
          className="shadow appearance-none border w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
          id="firstName"
          type="text"
          placeholder="First Name" />
      </div>
      <div className="mb-4">
        <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="lastName">
          LastName
        </label>
        <input
          value={owner.lastName || ''}
          onChange={(e) => onChange(e)}
          className="shadow appearance-none border w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
          id="lastName"
          type="text"
          placeholder="Last Name" />
      </div>
      <div className="mb-4">
        <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="email">
          Email
        </label>
        <input
          value={owner.email || ''}
          onChange={(e) => onChange(e)}
          className="shadow appearance-none border w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
          id="email"
          type="text"
          placeholder="Email" />
      </div>
      <div className="mb-4">
        <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="address">
          Address
        </label>
        <input
          value={owner.address || ''}
          onChange={(e) => onChange(e)}
          className="shadow appearance-none border w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
          id="address"
          type="text"
          placeholder="Address" />
      </div>
      <div className="mb-4">
        <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="city">
          City
        </label>
        <input
          value={owner.city || ''}
          onChange={(e) => onChange(e)}
          className="shadow appearance-none border w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
          id="city"
          type="text"
          placeholder="City" />
      </div>
      <div className="mb-4">
        <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="telephone">
          Telephone
        </label>
        <input
          value={owner.telephone || ''}
          onChange={(e) => onChange(e)}
          className="shadow appearance-none border w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
          id="telephone"
          type="text"
          placeholder="Telephone" />
      </div>
      <div className="mb-4">
        <button
          className="w-full bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4"
          type="button"
          onClick={() => {
            props.ownerCallback({
              ...owner,
              id: owner.id || props.ownerLen + 1
            });
            setOwner({})
          }}
        >
          {props.owner ? 'Edit Owner' : 'Add Owner'}
        </button>
      </div>
    </form>
  )
}

export function PetModal(props) {
  const [pet, setPet] = useState(props.pet || {})

  const onChange = (e) => {
    setPet({
      ...pet,
      [e.target.id]: e.target.value
    })
  }

  return (
    <form className="px-8 pt-6 pb-8 mb-4">
      <div className="mb-4">
        <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="name">
          Name
        </label>
        <input
          value={pet.name || ''}
          onChange={(e) => onChange(e)}
          className="shadow appearance-none border w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
          id="name"
          type="text"
          placeholder="Name" />
      </div>
      <div className="mb-4">
        <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="birthdate">
          Birthdate
        </label>
        <input
          value={pet.birthdate || ''}
          onChange={(e) => onChange(e)}
          className="shadow appearance-none border w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
          id="birthdate"
          type="date"
          placeholder="Birthdate" />
      </div>
      <div className="mb-4">
        <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="petType">
          Pet Type
        </label>
        <select
          value={pet.petType || ''}
          onChange={(e) => onChange(e)}
          className="shadow appearance-none border w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
          id="petType">
          <option value=''>Choose pet type...</option>
          <option value="Dog">Dog</option>
          <option value="Cat">Cat</option>
          <option value="Snake">Snake</option>
          <option value="Hamster">Hamster</option>
          <option value="Parrot">Parrot</option>
        </select>
      </div>
      <div className="mb-4">
        <button
          className="w-full bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4"
          type="button"
          onClick={() => {
            props.petCallback({
              ...pet,
              id: pet.id || props.petsLen + 1
            });
            setPet({})
          }}
        >
          {props.pet ? 'Edit Pet' : 'Add Pet'}
        </button>
      </div>
    </form>
  )
}

export function VisitsModal(props) {
  const [visits, setVisits] = useState([]);
  const [visit, setVisit] = useState({});

  const onChange = (e) => {
    setVisit({
      ...visit,
      [e.target.id]: e.target.value
    })
  }
  return (
    <div className="px-8 pt-6 pb-8">
      <form className="mb-4">
        <div className="mb-4">
          <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="description">
            Description
          </label>
          <input
            value={visit.description || ''}
            onChange={(e) => onChange(e)}
            className="shadow appearance-none border w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            id="description"
            type="text"
            placeholder="Description" />
        </div>
        <div className="mb-4">
          <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="date">
            Date
          </label>
          <input
            value={visit.date || ''}
            onChange={(e) => onChange(e)}
            className="shadow appearance-none border w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            id="date"
            type="date"
            placeholder="Date" />
        </div>
        <div className="mb-4">
          <button
            className="w-full bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4"
            type="button"
            onClick={() => {
              setVisits([{
                ...visit,
                id: visits.length + 1
              }, ...visits]);
              setVisit({})
            }}
          >
            Add Visit
          </button>
        </div>
      </form>
      <div>
        <div className="bg-white-lg shadow">
          <ul className="divide-y-2 divide-gray-100">
            {visits.map((v, idx) => {
              return (
                <li key={idx} className="p-3">
                  <span className="italic">{v.date}</span>
                  <p className="font-bold">{v.description}</p>
                </li>
              )
            })}

          </ul>
        </div>

      </div>
    </div >
  );
}