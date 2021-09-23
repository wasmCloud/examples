import React, { useEffect, useState } from 'react';
import { fakeVets } from './fake';
/*
{
  id: 1,
  firstName: '',
  lastName: '',
  specialties: []
}
*/

export default function Vets() {
  const [vets, setVets] = useState([]);

  useEffect(() => {
    // async function fetchVets() {
    //   const response = await fetch('/vets').catch((err) => { throw err });
    //   const data = await response.json().catch((err) => { throw err });
    //   return data;
    // }
    setVets(fakeVets);
  }, [])



  return (
    <table className="w-full shadow-lg">
      <thead className="bg-blue-500 border">
        <tr>
          <th className="text-left px-8 py-4">Name</th>
          <th className="text-left px-8 py-4">Specialties</th>
        </tr>
      </thead>
      <tbody>
        {vets.map((vet, idx) => {
          return (
            <tr onClick={() => console.log(vet)} key={idx} className="border cursor-pointer">
              <td className="px-8 py-4">{`${vet.firstName} ${vet.lastName}`}</td>
              <td className="px-8 py-4">{vet.specialties.length === 0 ? "none" : vet.specialties.join(", ")}</td>
            </tr>
          )
        })}
      </tbody>
    </table>
  )
}