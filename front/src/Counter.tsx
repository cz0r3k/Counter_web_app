import axios from 'axios'
import {useEffect, useState} from 'react';
// import {Button as button, ButtonToolbar as div} from 'react-bootstrap'

export function Counter() {
    const [count, setCount] = useState(0);
    const api_url = '/api'
  
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
        axios.post(api_url + '/increment');
        await set_counter();
      } catch (error) {
        console.error(error);
      }
    }
  
    const decrement = async() => {
      try {
        axios.post(api_url + '/decrement');
        await set_counter();
      } catch (error) {
        console.error(error);
      }
    }
  
    return (
      <>
        Licznik: {count}
       <div className='btn-toolbar' role='toolbar'>
         <button type='button' className='btn btn-outline-primary btn-lg' onClick={decrement}>-</button>
         <button type='button' className='btn btn-outline-primary btn-lg' onClick={increment}>+</button>
       </div>
      </>
    );
  }