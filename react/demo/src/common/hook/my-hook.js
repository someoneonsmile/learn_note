import { useState, useEffect } from 'react';

export function useInput(initialValue){
    const [value, setValue] = useState(initialValue);
    useEffect(() => {
        document.title = value; 
    }, [value])
    return {
        value,
        onChange: (e) => {
            setValue(e.target.value)
        }
    }
}