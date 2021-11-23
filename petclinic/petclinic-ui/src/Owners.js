import React, { useEffect, useState } from 'react';
import api from './Api';
import { Modal, OwnerModal } from './Modal';
import Owner from './Owner';
/*
{
  id: 1,
  firstName: '',
  lastName: '',
  address: '',
  city: '',
  telephone: '',
  email: ''
}
*/

export default function Owners() {
  const [owners, setOwners] = useState([]);
  const [owner, setOwner] = useState(false);

  const [showModal, setShowModal] = useState(false);

  useEffect(() => {
    async function fetchOwners() {
      const owners = await api.getOwners().catch((err) => { return err })
      setOwners(owners);
    }
    fetchOwners();
  }, [])

  const addOwner = async (owner) => {
    owner.id = owners.length + 1;
    await api.createOwner(owner).catch((err) => { return err })
    setOwners([owner, ...owners])
  }

  const renderModal = () => {
    return showModal ? (
      <Modal modalTitle={'Add Owner'} setShowModal={(val) => setShowModal(val)}>
        <OwnerModal
          ownerCallback={(owner) => {
            addOwner(owner);
            setShowModal(false);
          }}
          ownerLen={owners.length}
        />
      </Modal>
    )
      :
      null;
  }

  const renderTable = () => {
    return (
      <div>
        {renderModal()}
        <button onClick={() => setShowModal(true)} className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 mb-2">
          Add Owner
        </button>
        <table className="w-full shadow-lg">
          <thead className="bg-blue-500 border">
            <tr>
              <th className="text-left px-8 py-4">Name</th>
              {/* <th className="text-left px-8 py-4">Address</th>
              <th className="text-left px-8 py-4">Telephone</th> */}
              <th className="text-left px-8 py-4">Email</th>
            </tr>
          </thead>
          <tbody>
            {owners.map((owner, idx) => {
              return (
                <tr onClick={() => setOwner(owner)} key={idx} className="border cursor-pointer">
                  <td className="px-8 py-4">{`${owner.firstName} ${owner.lastName}`}</td>
                  {/* <td className="px-8 py-4">{`${owner.address}, ${owner.city}`}</td>
                  <td className="px-8 py-4">{owner.telephone}</td> */}
                  <td className="px-8 py-4">{owner.email}</td>
                </tr>
              )
            })}
          </tbody>
        </table>
      </div>
    )
  }

  return owner ?
    <Owner
      owner={owner}
      goBack={() => setOwner(false)}
      ownerLen={owner.length}
    />
    : (
      renderTable()
    )
}