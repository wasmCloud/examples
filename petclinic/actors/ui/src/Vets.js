import React, { useEffect, useState } from 'react';
import api from './Api';


export default function Vets() {
  const [vets, setVets] = useState([]);

  useEffect(() => {
    async function fetchVets() {
      const response = await api.getVets().catch((err) => { return err })
      setVets(response);
    }
    fetchVets();
  }, [])

  const getVetSpecialityIcon = (spec) => {
    switch (spec) {
      case "dentistry":
        return 'fa fa-teeth-open'
      case "radiology":
        return 'fa fa-x-ray'
      case "surgery":
        return 'fa fa-scalpel'
      default:
        return 'fa fa-medical-records'
    }
  }



  return (
    <div className="rounded-md">
      <table className="w-full shadow-lg bg-white bg-opacity-50">
        <thead className="rounded-t-md">
          <tr className="text-wasmcloudGray">
            <th className="text-left px-8 py-4">Name</th>
            <th className="text-left px-8 py-4">Specialties</th>
          </tr>
        </thead>
        <tbody>
          {vets && vets.length > 0 && vets.map((vet, idx) => {
            return (
              <tr onClick={() => console.log(vet)} key={idx} className="border cursor-pointer">
                <td className="px-8 py-4">{`${vet.firstName} ${vet.lastName}`}</td>
                <td className="px-8 py-4">{vet.specialties.length === 0 ? "none" :
                  <div className="flex flex-col space-y-2">
                    {vet.specialties.map((spec, iidx) => {
                      return (
                        <div className="flex space-x-2 " key={`vet-${idx}-spec-${iidx}`}>
                          <i className={`${getVetSpecialityIcon(spec)} text-wasmcloudGreen-light my-auto`} />
                          <span>{spec}</span>
                        </div>
                      )
                    })}
                  </div>

                }
                </td>
              </tr>
            )
          })}
        </tbody>
      </table>
    </div>
  )
}