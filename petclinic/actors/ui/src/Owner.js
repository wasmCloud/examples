import React, { Component } from 'react';
import { Modal, OwnerModal, PetModal, VisitsModal } from './Modal';
import api from './Api';
/*
{
  id: 1,
  name: '',
  petType: {
    id: 1,
    name: ''
  },
  birthdate: '',
}

{
  id: 1,
  description: '',
  petId: 1,
  vetId: 1
}

*/

export default class Owner extends Component {
  constructor(props) {
    super(props);
    this.state = {
      owner: props.owner || false,
      pets: [],
      pet: false,
      showModal: false
    }
  }

  componentDidMount() {
    this.getPets()
  }

  async updateOwner(owner) {
    await api.updateOwner(this.props.owner.id, owner).catch((err) => { return err })
    this.setState({
      owner: owner,
      showModal: false
    })
  }

  async getPets() {
    const response = await api.getPets(this.props.owner.id).catch((err) => { return err })
    this.setState({
      pets: response,
    })
  }

  async addOrEditPet(pet) {
    if (this.state.pet) {
      pet.petType = Number.isInteger(pet.petType) ? pet.petType : pet.petType.id
      await api.updatePet(this.props.owner.id, pet.id, pet).catch((err) => { return err })
    } else {
      await api.createPet(this.props.owner.id, pet).catch((err) => { return err })
    }
    this.setState({
      pets: !this.state.pet ?
        [pet, ...this.state.pets]
        :
        this.state.pets.map(p => p.id === pet.id ? pet : p),
      showModal: false,
      pet: false
    })
  }

  renderOwner() {
    const { owner } = this.state;
    return (
      <div className="w-1/3 pr-2">
        <div className="p-8 bg-white shadow-md">
          <h2 className="text-2xl font-bold text-gray-800">{`${owner.firstName} ${owner.lastName}`}</h2>
          {/* <p className="text-gray-600">{`${owner.address}, ${owner.city}`}</p> */}
          <p className="text-gray-600">{owner.email}</p>
          {/* <p className="text-gray-600">{owner.telephone}</p> */}
          <div className="mt-2">
            <button
              onClick={() => {
                this.setState({
                  showModal: 'owner'
                })
              }}
              className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 mb-2 mr-2">
              Edit Owner
            </button>
            <button
              onClick={() => {
                this.setState({
                  showModal: 'addOrEditPet'
                })
              }}
              className="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 mb-2">
              Add Pet
            </button>
          </div>
        </div>
      </div>
    )
  }

  renderPets() {
    const { pets } = this.state;
    return (
      <div className="w-2/3">
        {pets.map((pet, idx) => {
          return (
            <div key={idx} className="pr-2 pb-2">
              <div className="p-8 bg-white shadow-md">
                <h2 className="text-2xl font-bold text-gray-800">{pet.name}</h2>
                <p className="text-gray-600 italic">{pet.petType.name}</p>
                <p className="text-gray-600">{pet.birthdate.month}/{pet.birthdate.day}/{pet.birthdate.year}</p>
                <div className="mt-2">
                  <button
                    onClick={() => {
                      this.setState({
                        showModal: 'addOrEditPet',
                        pet: pet,
                      })
                    }}
                    className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 mb-2 mr-2">
                    Edit Pet
                  </button>
                  <button
                    onClick={() => {
                      this.setState({
                        showModal: 'visits',
                        pet: pet
                      })
                    }}
                    className="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 mb-2">
                    Visits
                  </button>
                </div>
              </div>
            </div>
          )
        })}
      </div>
    )
  }

  renderModal() {
    const closeModal = (val) => {
      this.setState({
        showModal: val,
        pet: false
      })
    };
    switch (this.state.showModal) {
      case 'owner':
        return (
          <Modal
            modalTitle={'Edit Owner'}
            setShowModal={(val) => closeModal(val)}>
            <OwnerModal
              owner={this.state.owner}
              ownerCallback={(owner) => {
                this.updateOwner(owner);
              }}
            />
          </Modal>
        );
      case 'addOrEditPet':
        return (
          <Modal
            modalTitle={this.state.pet ? 'Edit Pet' : 'Add Pet'}
            setShowModal={(val) => closeModal(val)}>
            <PetModal
              owner={this.state.owner}
              pet={this.state.pet}
              petCallback={(pet) => {
                if (!this.state.pet) {
                  pet.id = this.state.pets.length + 1;
                }
                this.addOrEditPet(pet);
              }}
            />
          </Modal>
        )
      case 'visits':
        return (
          <Modal
            modalTitle={`Visits for ${this.state.pet.name}`}
            setShowModal={(val) => closeModal(val)}>
            <VisitsModal owner={this.state.owner} pet={this.state.pet} />
          </Modal>
        )
      case false:
        return null;
      default:
        return null;
    }
  }

  render() {
    return (
      <div>
        {this.state.showModal ? this.renderModal() : null}
        <button onClick={() => this.props.goBack()} className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 mb-2">
          Back
        </button>
        <div className="flex">
          {this.renderOwner()}
          {this.renderPets()}
        </div>
      </div>
    )
  }
}