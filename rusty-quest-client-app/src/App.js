import React, { useEffect, useState, useCallback  } from 'react';
import './App.css';

const DEFAULT_URL = 'http://192.168.0.134:3030';
// const BASE_URL = 'http://localhost:3030';



function App() {
  const [gameState, setGameState] = useState(null);  
  const [ipAddress, setIpAddress] = useState(() => {
    return localStorage.getItem('ipAddress') || DEFAULT_URL;
  });
  const [tempIp, setTempIp] = useState('');

  const handleInputChange = (event) => {
    setTempIp(event.target.value);
  };

  const applyIpAddress = () => {
    setIpAddress(tempIp);
  };


  // START added 12/30

  const handleNewGame = async () => {
    setIsLoading(true);
    try {
      const response = await fetch(`${ipAddress}/game_state/new_game`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
      });
      const data = await response.json();
      setGameState(data);
      setGameId(data.id);
    } catch (error) {
      console.error('Error fetching new game:', error);
    } finally {
      setIsLoading(false);
    }
  };

  
  // const [gameState, setGameState] = useState({});
  const [gameId, setGameId] = useState(null);
  const [isLoading, setIsLoading] = useState(false);
  
  const refreshGameState = async () => {
    if (!gameId) return;

    setIsLoading(true);
    try {
      const response = await fetch(`${ipAddress}/game_state/${gameId}`);
      const data = await response.json();
      setGameState(data);
    } catch (error) {
      console.error('Error fetching game state:', error);
    } finally {
      setIsLoading(false);
    }
  };
  const handleShuffleClick = () => {
      fetch(`${ipAddress}/game_state/${gameId}/shuffle_deck`, { method: 'POST' }) // Assuming POST for shuffle
        .then(response => {
          // if (response.ok) {
          //   return fetchGameState();
          // } else {
          //   throw new Error('Shuffle failed');
          // }
        })
        .then(response => response.json())
        .then(data => {
          setGameState(data);
        })
        .catch(error => console.error('Error:', error));
    };
  
  // END added 12/30

  const handleSelectChange = (event) => {
    setTempIp(event.target.value);
  };

  
  
  function IpAddressConfiguration() {
    return (
      <div>  
        <label htmlFor="currentIpInput">Current IP </label>
        <input id="currentIpInput" type="text" value={ipAddress} />
        <br />
        <hr />
        <label htmlFor="ipToSetInput">IP to set </label>
        <input id="ipToSetInput" type="text" value={tempIp} onChange={handleInputChange} />
  
        <button onClick={applyIpAddress}>Apply IP Address</button>
        {/* <button onClick={fetchGameState}>Fetch Data</button> */}
        <br />
        <label htmlFor="autoLoadSelect">Auto load</label>
        <select id="autoLoadSelect" value={ipAddress} onChange={handleSelectChange}>
          <option value="http://192.168.0.134:3030">http://192.168.0.134:3030</option>
          <option value="http://localhost:3030">http://localhost:3030</option>
        </select>
        <hr />
      </div>
    );
  }

  return (
    <div className="App">
      <header className="App-header">
        {
          <IpAddressConfiguration />
        }
        { 
          <CreateGame gameState={gameState} gameId={gameId} isLoading={isLoading} handleNewGame={handleNewGame} refreshGameState={refreshGameState} handleShuffleClick={handleShuffleClick}/>
          
        }
      </header>
    </div>
  );
}

export default App;


function CreateGame({gameState,gameId,isLoading,handleNewGame,refreshGameState, handleShuffleClick}) {
  return (
    <div className="App">
      <button onClick={handleNewGame} disabled={isLoading}>
        {isLoading ? 'Loading...' : 'Start New Game'}
      </button>
    { gameState?
    <div>
      <button onClick={refreshGameState} disabled={isLoading || !gameId}>
        Refresh Game State
      </button>
      <div>
        <h2>Game State:</h2>
        <textarea
          value={JSON.stringify(gameState, null, 2)}
          readOnly
          style={{ width: '100%', height: '300px' }}
        />
      </div>
      <div>
        <CardList cards={gameState.deck} shuffleClickHandler={handleShuffleClick} />
      </div>
    </div>
    :
    <h1>Loading...</h1>
    }
    </div>
  );
}


function CardList({ cards, shuffleClickHandler }) {
  const cardListStyle = {
    maxHeight: '200px', // Set a fixed height
    overflowY: 'auto', // Enable vertical scrolling
    border: '1px solid #ccc', // Optional: adds a border for better visibility
    padding: '10px', // Optional: adds some padding inside the scrollable area
    margin: '10px 0' // Optional: adds some margin around the scrollable area
  };

  return (
    <div>
      <h2>Cards</h2>
      <div style={cardListStyle}>
        {cards.map((card, index) => (
          <div key={index}>
            {card.value}, {card.typ}
          </div>
        ))}
      </div>
      <button onClick={shuffleClickHandler}>Shuffle Deck</button>
    </div>
  );
}

// function CardList({ cards }) {
//   return (
//     <div>
//       <h2>Cards</h2>
//       {cards.map((card, index) => (
//         <div key={index}>
//           Type: {card.typ}, Value: {card.value}
//         </div>
//       ))}
//     </div>
//   );
// }


function PlayerList({ players }) {
  return (
    <div>
      <h2>Players</h2>
      {/* Render player list */}
    </div>
  );
}



// Moving examples down here to reference later. 
// TODO: Delete when more established above
/*
  // const fetchGameState = useCallback(() => {    
  //   fetch(`${ipAddress}/game_state`)
  //   .then(response => response.json())
  //   .then(data => {
  //     setGameState(data);
  //   })
  //   .catch(error => console.error('Error fetching data:', error));
  // }, [ipAddress]);

  // useEffect(() => {
  //   fetchGameState();
  //   localStorage.setItem('ipAddress', ipAddress);
  // }, [fetchGameState,ipAddress]);

  // const handleShuffleClick = () => {
  //   fetch(`${ipAddress}/shuffle_deck`, { method: 'POST' }) // Assuming POST for shuffle
  //     .then(response => {
  //       if (response.ok) {
  //         return fetchGameState();
  //       } else {
  //         throw new Error('Shuffle failed');
  //       }
  //     })
  //     .then(response => response.json())
  //     .then(data => {
  //       setGameState(data);
  //     })
  //     .catch(error => console.error('Error:', error));
  // };
  */

