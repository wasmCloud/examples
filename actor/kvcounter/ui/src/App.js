import Confetti from 'react-confetti'
import { useState, useEffect } from 'react';

function getWindowDimensions() {
  const { innerWidth: width, innerHeight: height } = window;
  return {
    width,
    height
  };
}


function App() {
  const [windowDimensions, setWindowDimensions] = useState(getWindowDimensions());
  const [counter, setCounter] = useState(0);
  const [showConfetti, setShowConfetti] = useState(false);

  const increment = () => {
    setCounter(prevCounter => prevCounter + 1)
  }

  useEffect(() => {
    if (counter > 0) {
      setShowConfetti(true)
    }
  }, [counter])


  return (
    <div className="flex items-center justify-center h-screen relative">
      <h1 className="absolute bottom-0 right-2 text-wasmcloudGray text-[72px]">
        <span className="text-wasmcloudGreen-light">+</span>
        <span>KVCounter</span>
      </h1>
      <img src="/wasmcloud_logo.svg" className="absolute -bottom-[12%] right-2 w-1/3 h-1/2" />
      <div className="flex flex-col space-y-2">
        <h1 className="text-[84px] mx-auto font-boldest text-wasmcloudGreen-light">{counter}</h1>
        <button
          onClick={() => increment()}
          className="bg-wasmcloudGreen-light rounded-md hover:bg-wasmcloudGreen-dark text-white font-bold py-2 px-4 mb-2 mr-2">
          Increment
        </button>
      </div>
      {showConfetti && <Confetti
        width={windowDimensions.width}
        height={windowDimensions.height}
        onConfettiComplete={() => setShowConfetti(false)}
        recycle={false}
        gravity={0.2}
      />}
    </div>

  );
}

export default App;
