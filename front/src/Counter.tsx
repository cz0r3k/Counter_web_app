import './Counter.css';
import axios from 'axios'
import {useEffect, useState} from 'react';

export function Counter() {
    const [count, setCount] = useState(0);
    const api_url = 'http://localhost:8000/api'
  
    async function get_counter(){
      try {
        const obj = await axios.get(api_url + '/get_counter');
        const value = obj.data.value;
        return value;
      } catch (error) {
        console.error(error);
      }
    }
  
    async function set_counter():Promise<any>{
      const value = await get_counter();
      setCount(value);
    }
  
    useEffect(() => {
      set_counter();
    }, []);
  
    const increment = async() => {
      try {
        await axios.post(api_url + '/increment').then(await set_counter());
      } catch (error) {
        console.error(error);
      }
    }
  
    const decrement = async() => {
      try {
        await axios.post(api_url + '/decrement').then(await set_counter());
      } catch (error) {
        console.error(error);
      }
    }
  
    return (
      <>
        Licznik: {count}
        <div className="buttons">
          <button className='action_btn' onClick={decrement}>-</button>
          <button className='action_btn' onClick={increment}>+</button>
        </div>
      </>
    );
  }