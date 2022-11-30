import { Fireworks, FireworksHandlers } from '@fireworks-js/react'
import { useState, useEffect, useRef, FormEvent } from 'react'
import api from '../services/ApiService'
import { ReactComponent as Logo } from '../assets/logo.svg'
import { useScreenSize } from '../hooks/useScreenSize'

function App() {
  const [bucket, setBucket] = useState('')
  const [count, setCount] = useState(0)
  const dimensions = useScreenSize()
  const fireworks = useRef<FireworksHandlers>(null)

  const updateCount = async (key?: string) => {
    try {
      const response = await api.increment(key)
      setCount(response.counter)
      return response.counter
    } catch (err) {
      console.log(err)
    }
  }

  const launch = (total: number) => {
    fireworks.current?.launch(total > 50 ? 50 : total)
  }

  useEffect(() => {
    fireworks.current?.updateSize(dimensions)
  }, [dimensions])

  const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    let workingTotal = count
    const el = e.currentTarget.elements.namedItem('bucket') as HTMLInputElement
    const newBucket = el?.value ?? ''
    if (newBucket !== bucket) {
      workingTotal = 0
    }
    updateCount(newBucket).then((newCount) => {
      setBucket(newBucket)
      launch(newCount - workingTotal)
    })
  }

  useEffect(() => {
    updateCount().then((newCount) => launch(newCount))
  }, [])

  return (
    <div className="h-full flex flex-col items-center justify-center">
      <div className="flex flex-col gap-2">
        <form className="flex flex-wrap gap-2" onSubmit={handleSubmit}>
          <input
            id="bucket"
            name="bucket"
            placeholder="Enter a bucket name"
            className="mx-auto px-2 py-1.5 text-center w-56 max-w-full text-wasmcloudGray rounded-md border border-wasmcloudGreen-light"
          />
          <button
            type="submit"
            className="bg-wasmcloudGreen-light w-56 max-w-full rounded-md hover:bg-wasmcloudGreen-dark text-white font-bold py-2 px-4 my-auto mx-auto"
          >
            Increment
          </button>
        </form>
        <h2 className="text-7xl mt-5 mx-auto font-bolder text-wasmcloudGreen-light">
          {count}
        </h2>
      </div>
      <Logo className="absolute p-4 bottom-0 right-0 w-48" />
      <Fireworks
        ref={fireworks}
        autostart={false}
        className="absolute inset-0 h-full w-full -z-10"
      />
    </div>
  )
}

export default App
