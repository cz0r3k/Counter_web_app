import axios from 'axios'
import {useEffect, useState} from 'react';

export function Counter() {
    const [counter, setLocalCounter] = useState(0);
    const api_url = '/api'
  
    async function set_counter():Promise<any>{
      const obj = await axios.get(api_url + '/get_counter');
      const value = obj.data.value;
      setLocalCounter(value);
    }
  
    useEffect(() => {
      set_counter();
    }, []);
  
    const increment = async() => {
      try {
        setLocalCounter((await axios.post(api_url + '/increment')).data.value);
      } catch (error) {
        console.error(error);
      }
    }
  
    const decrement = async() => {
      try {
        setLocalCounter((await axios.post(api_url + '/decrement')).data.value);
      } catch (error) {
        console.error(error);
      }
    }
  
    return (
      <>
        Licznik: {counter}
       <div className='btn-toolbar' role='toolbar'>
         <button type='button' className='btn btn-outline-primary btn-lg' onClick={decrement}>-</button>
         <button type='button' className='btn btn-outline-primary btn-lg' onClick={increment}>+</button>
       </div>
      </>
    );
  }