import Confetti from 'react-confetti'
import { useState, useEffect, useRef } from 'react';
import api from './Api';
import Fireworks from '@fireworks-js/react';

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
  const [showFireworks, setShowFireworks] = useState(false);

  const fireworksRef = useRef(null);


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
      if (counter % 2 === 0) {
        setShowConfetti(true)
        setShowFireworks(false)
      } else {
        setShowFireworks(true);
        setShowConfetti(false)
      }
    }
  }, [counter])

  useEffect(() => {
    if (!showFireworks) {
      if (fireworksRef.current && fireworksRef.current.isRunning) {
        fireworksRef.current.stop();
      }
    }
  }, [showFireworks])

  console.log(windowDimensions.height)
  return (
    <div className="flex items-center justify-center h-screen relative">
      <div className="z-40">
        <h1 className="absolute bottom-0 right-2 text-wasmcloudGray text-[72px]">
          <span className="text-wasmcloudGreen-light">+</span>
          <span>KVCounter</span>
        </h1>
        <img src="/wasmcloud_logo.svg" className="absolute -bottom-[12%] right-2 w-1/3 h-1/2" alt='logo' />
        <div className="flex flex-col space-y-2 mx-auto">
          <div className="flex space-x-2">
            <input
              id="bucket" name="bucket"
              placeholder='Enter Bucket Name' onChange={(e) => setBucket(e.target.value)}
              className="p-4 mx-auto h-[100px] text-center w-4/5 text-[88px] text-wasmcloudGray rounded-md border border-wasmcloudGreen-light" />
            <button
              onClick={() => getCounter(bucket)}
              className="bg-wasmcloudGreen-light rounded-md h-[40px] w-[125px] hover:bg-wasmcloudGreen-dark text-white font-bold py-2 px-4 my-auto mx-auto">
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
      {showFireworks && <Fireworks
        ref={fireworksRef}
        explosion={20}
        options={{ opacity: 0.5 }}
        style={{
          top: 0,
          left: 0,
          position: 'absolute',
          width: '100%',
          height: '100%',
          background: '#fff'
        }}
      />}
    </div>

  );
}

export default App;
