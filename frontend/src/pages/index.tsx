import React from 'react';
import { Button, ButtonGroup } from '@chakra-ui/react' 
import { useRouter } from 'next/router';

function NavigateButton() {
  const router = useRouter();

  const navigate = () => {
    router.push('/directory');
  };  

  return (
    <>
      <h1 className="text-4xl font-bold">This is the home page</h1>
      <Button className="mt-5" colorScheme='teal' variant='outline' onClick={navigate}>
        This is a button!!!
      </Button>
    </>
  );  
}


export default function Home() {
  return (
    <main>
      <NavigateButton />
    </main>
  );
}
