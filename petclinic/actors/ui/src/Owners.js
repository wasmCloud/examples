import React, { useEffect, useState } from 'react';
import api from './Api';
import { Modal, OwnerModal } from './Modal';
import Owner from './Owner';
import { fakeOwners } from './fake'

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

  console.log("OWNER", owner)

  const renderTable = () => {
    return (
      <div className="">
        {renderModal()}
        <button onClick={() => setShowModal(true)} className="bg-wasmcloudGreen-light rounded-md hover:bg-wasmcloudGreen-dark text-white font-bold py-2 px-4 mb-2">
          Add Owner
        </button>
        <div className="rounded-md z-20">
          <table className="w-full shadow-lg bg-white bg-opacity-50">
            <thead className="rounded-t-md">
              <tr className="text-wasmcloudGray">
                <th className="text-left px-8 py-4">Name</th>
                <th className="text-left px-8 py-4">Contact</th>
                {/* <th className="text-left px-8 py-4">Address</th>
              <th className="text-left px-8 py-4">Telephone</th> */}
              </tr>
            </thead>
            <tbody className="rounded-b-md">
              {owners && owners.length > 0 && owners.map((own, idx) => {
                return (
                  <tr key={idx} className="border">
                    <td className="px-8 py-4 flex flex-col space-y-2">
                      <span className="font-bold">{`${own.firstName} ${own.lastName}`}</span>
                      <div className="flex space-x-2">
                        <i className="fa fa-at my-auto text-wasmcloudGreen-light" />
                        <span className="italic">{own.email}</span>
                      </div>
                    </td>
                    <td className="px-8 py-4">
                      <div className="flex flex-col space-y-2">
                        <div className="flex space-x-2">
                          <i className="fa fa-house my-auto text-wasmcloudGreen-light" />
                          <span>{own.address}, {own.city}</span>
                        </div>
                        <div className="flex space-x-2">
                          <i className="fa fa-phone my-auto text-wasmcloudGreen-light" />
                          <span className="italic">{own.telephone}</span>
                        </div>
                      </div>
                    </td>
                    <td className="text-right pr-8">
                      <button onClick={() => setOwner(own)} className="bg-wasmcloudGreen-light rounded-md hover:bg-wasmcloudGreen-dark text-white font-bold py-2 px-4 mb-2">
                        Manage
                      </button>
                    </td>
                  </tr>
                )
              })}
            </tbody>
          </table>
        </div>
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