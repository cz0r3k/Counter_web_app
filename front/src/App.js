import './App.css';
import axios from 'axios'
import React, { Component, useEffect, useState} from 'react';



function Counter() {
  const [count, setCount] = useState(0);
  async function get_counter() {
    try {
      const obj = await axios.get('http://0.0.0.0:8000/get_counter');
      const value = obj.data.value;
      console.log(value);
      return value;
    } catch (error) {
      console.error(error);
    }
  }
  async function set_counter(){
    const value = await get_counter();
    setCount(value);
  }

  useEffect(() => {
    set_counter();
  }, []);

  const increment = async() => {
    try {
      await axios.post('http://0.0.0.0:8000/increment').then(await set_counter());
    } catch (error) {
      console.error(error);
    }
  }
  const decrement = async() => {
    try {
      await axios.post('http://0.0.0.0:8000/decrement').then(await set_counter());
    } catch (error) {
      console.error(error);
    }
  }
  return (
    <>
      Licznik: {count}
      <button onClick={decrement}>-</button>
      <button onClick={increment}>+</button>
    </>
  );
}


function App() {
  return (
    <div className="App">
      <header className="App-header">
        <Counter/>       
      </header>
    </div>
  );
}

export default App;
