import React, { useEffect, useState } from 'react';
import api from './Api';

function parseDateString(day, month, year) {
  return new Date(Date.parse(`${year}-${month}-${day}`)).toISOString().split('T')[0]
}

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
      <div>
        <div className="relative p-6 flex-auto">
          {props.children}
        </div>
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
            props.ownerCallback(owner);
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
  const [petTypes, setPetTypes] = useState([]);

  useEffect(() => {
    async function fetchPetTypes() {
      const response = await api.getPetTypes().catch((err) => { return err })
      setPetTypes(response);
    }
    fetchPetTypes();
  }, [])

  const onChange = (e) => {
    let val = e.target.value;
    if (e.target.id === 'birthdate') {
      const petBirthdate = new Date(e.target.value);
      val = {
        day: petBirthdate.getUTCDate(),
        month: petBirthdate.getUTCMonth() + 1,
        year: petBirthdate.getUTCFullYear()
      }
    }
    if (e.target.id === 'petType') {
      val = parseInt(e.target.value);
    }
    setPet({
      ...pet,
      [e.target.id]: val
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
          value={pet.birthdate ? parseDateString(pet.birthdate.day, pet.birthdate.month, pet.birthdate.year) : ''}
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
          value={pet.petType ? pet.petType.id : ''}
          onChange={(e) => onChange(e)}
          className="shadow appearance-none border w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
          id="petType">
          <option value=''>Choose pet type...</option>
          {petTypes.map((petType, idx) => {
            return (
              <option key={idx} value={petType.id}>{petType.name}</option>
            )
          })}
        </select>
      </div>
      <div className="mb-4">
        <button
          className="w-full bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4"
          type="button"
          onClick={() => {
            props.petCallback(pet);
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
  const [vets, setVets] = useState([]);

  const onChange = (e) => {
    let val = e.target.value;
    if (e.target.id === 'date') {
      const visitDate = new Date(e.target.value);
      val = {
        day: visitDate.getUTCDate(),
        month: visitDate.getUTCMonth() + 1,
        year: visitDate.getUTCFullYear()
      }
    }
    setVisit({
      ...visit,
      [e.target.id]: val
    })
  }

  useEffect(() => {
    async function fetchVisits() {
      const response = await api.getPetVisits(props.owner.id, props.pet.id).catch((err) => { return err })
      setVisits(response);
    }
    async function fetchVets() {
      const response = await api.getVets().catch((err) => { return err });
      setVets(response);
    }
    fetchVisits();
    fetchVets();
  }, [props.owner.id, props.pet.id]);


  const addVisit = async (visit) => {
    visit.petId = props.pet.id;
    visit.time = {
      hour: 12,
      minute: 0,
    }
    visit.vetId = 1;
    await api.createPetVisit(props.owner.id, props.pet.id, visit).catch((err) => { return err })
    setVisits([visit, ...visits]);
    setVisit({})
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
            value={visit.date ? parseDateString(visit.date.day, visit.date.month, visit.date.year) : ''}
            onChange={(e) => onChange(e)}
            className="shadow appearance-none border w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            id="date"
            type="date"
            placeholder="Date" />
        </div>
        <div className="mb-4">
          <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="petType">
            Vet
          </label>
          <select
            value={visit.vetId || ''}
            onChange={(e) => onChange(e)}
            className="shadow appearance-none border w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            id="vetId">
            <option value=''>Choose vet...</option>
            {vets.map((vet, idx) => {
              return (
                <option key={idx} value={vet.id}>{`${vet.firstName} ${vet.lastName}`}</option>
              )
            })}
          </select>
        </div>
        <div className="mb-4">
          <button
            className="w-full bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4"
            type="button"
            onClick={() => addVisit(visit)}
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
                  <span className="italic">{v.date.month}/{v.date.day}/{v.date.year}</span>
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