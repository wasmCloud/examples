import { Fireworks, FireworksHandlers } from "@fireworks-js/react"
import { useState, useEffect, useRef, FormEvent } from "react"
import api from "../services/ApiService"
import { ReactComponent as Logo } from "../assets/logo.svg"

function App() {
  const [bucket, setBucket] = useState("")
  const [count, setCount] = useState(0)
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

  useEffect(() => {
    // TODO: Update fireworks canvas size when screen is resized
    
  }, [])
  
  const handleSubmit = (e: FormEvent) => {
    e.preventDefault()
    // TODO: work out how to reset count when bucket name changes
    updateCount(bucket).then(newCount => fireworks.current?.launch(newCount - count))
  }

  useEffect(() => {
    updateCount().then(newCount => fireworks.current?.launch(newCount))
  }, [])

  return (
    <div className="h-full flex flex-col items-center justify-center">
      <div className="flex flex-col gap-2">
        <form className="flex flex-wrap gap-2" onSubmit={handleSubmit}>
          <input
            id="bucket"
            name="bucket"
            placeholder="Enter a bucket name"
            onChange={(e) => setBucket(e.target.value)}
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
