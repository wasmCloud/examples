import React, { useEffect, useState } from 'react';
import api from './Api';
/*
{
  id: 1,
  name: '',
}
*/

export default function PetTypes() {
  const [petTypes, setPetTypes] = useState([]);

  useEffect(() => {
    async function fetchPetTypes() {
      const response = await api.getPetTypes().catch((err) => { return err })
      setPetTypes(response);
    }
    fetchPetTypes();
  }, [])



  return (
    <table className="w-full shadow-lg">
      <thead className="bg-blue-500 border">
        <tr>
          <th className="text-left px-8 py-4">Name</th>
        </tr>
      </thead>
      <tbody>
        {petTypes && petTypes.length > 0 && petTypes.map((petType, idx) => {
          return (
            <tr onClick={() => console.log(petType)} key={idx} className="border cursor-pointer">
              <td className="px-8 py-4">{petType.name}</td>
            </tr>
          )
        })}
      </tbody>
    </table>
  )
}