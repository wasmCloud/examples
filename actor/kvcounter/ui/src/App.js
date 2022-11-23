import Confetti from 'react-confetti'
import { useState, useEffect } from 'react';
import api from './Api';

function getWindowDimensions() {
  const { innerWidth: width, innerHeight: height } = window;
  return {
    width,
    height
  };
}


function App() {
  const [bucket, setBucket] = useState('');
  const [windowDimensions, setWindowDimensions] = useState(getWindowDimensions());
  const [counter, setCounter] = useState(0);
  const [showConfetti, setShowConfetti] = useState(false);


  const getCounter = async (key) => {
    try {
      const response = await api.getKV(key);
      setCounter(response.counter)
    } catch (err) {
      console.log(err);
    }
  }

  useEffect(() => {
    getCounter();
  }, [])

  useEffect(() => {
    if (counter > 0) {
      setShowConfetti(true)
    }
  }, [counter])

  return (
    <div className="flex items-center justify-center h-screen relative overflow-hidden">
      <div className="z-40">
        <img src="/wasmcloud_logo.svg" className="absolute -bottom-[15%] right-2 w-1/3 h-1/2 max-h-1/2" height="50%" alt='logo' />
        <div className="flex flex-col space-y-2 mx-auto">
          <div className="flex flex-col space-y-2">
            <input
              id="bucket" name="bucket"
              placeholder='Bucket...' onChange={(e) => setBucket(e.target.value)}
              className="p-4 mx-auto w-[350px] h-[100px] text-center w-4/5 text-[88px] text-wasmcloudGray rounded-md border border-wasmcloudGreen-light" />
            <button
              onClick={() => getCounter(bucket)}
              className="bg-wasmcloudGreen-light rounded-md h-[40px] w-[350px] hover:bg-wasmcloudGreen-dark text-white font-bold py-2 px-4 my-auto mx-auto">
              Increment
            </button>
          </div>
          <h2 className="text-[84px] mx-auto font-bolder text-wasmcloudGreen-light">{counter}</h2>
        </div>
      </div>
      {showConfetti && <Confetti
        width={windowDimensions.width}
        height={window.innerHeight}
        // onConfettiComplete={() => setShowConfetti(false)}
        recycle={true}
        gravity={0.2}
      />}
    </div>

  );
}

export default App;
