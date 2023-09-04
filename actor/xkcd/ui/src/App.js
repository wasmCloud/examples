import React, { useState, useEffect } from 'react';
import {
  ChakraProvider,
  Box,
  Button,
  VStack,
  Grid,
  Text,
  theme,
  Image
} from '@chakra-ui/react';
import { ColorModeSwitcher } from './ColorModeSwitcher';

function App() {
  const [xkcdMetadata, setXkcdMetadata] = useState(null);
  const [xkcdLoading, setXkcdLoading] = useState(false);

  const fetchComic = async () => {
    setXkcdLoading(true);
    await fetch('/comic').then(response => response.json())
      .then(data => {
        setXkcdLoading(false);
        setXkcdMetadata(data);
      })
      .catch(error => {
        console.error('Error fetching comic:', error);
      });
  };

  useEffect(() => {
    fetchComic()
  }, []);

  return (
    <ChakraProvider theme={theme}>
      <Box textAlign="center" fontSize="xl">
        <Grid minH="100vh" p={3}>
          <ColorModeSwitcher justifySelf="flex-end" />
          <VStack height="90vh" spacing={8} align="center" justify="flex-start">
            <Button onClick={fetchComic} mb="4" textColor="#00C389">
              Generate random XKCD comic
            </Button>
            {xkcdMetadata ? (
              <div>
                <Text fontSize="3xl" fontWeight="800" mb="8">{xkcdMetadata.title}</Text>
                <Image src={xkcdMetadata.img} alt={xkcdMetadata.title} />
              </div>
            ) : xkcdLoading ? <Text fontSize="3xl" fontWeight="800">Loading comic...</Text> : undefined}
          </VStack>
        </Grid>
        <Image
          src="wasmcloud.svg"  // Update with the correct path
          alt="WasmCloud Logo"
          style={{
            position: 'absolute',
            bottom: 0,
            right: 0,
            padding: '8px',
            width: '18rem'
          }}
        />
      </Box>
    </ChakraProvider >
  );
}

export default App;
