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

  const fetchGameState = useCallback(() => {    
    fetch(`${ipAddress}/game_state`)
    .then(response => response.json())
    .then(data => {
      setGameState(data);
    })
    .catch(error => console.error('Error fetching data:', error));
  }, [ipAddress]);

  useEffect(() => {
    fetchGameState();
    localStorage.setItem('ipAddress', ipAddress);
  }, [fetchGameState,ipAddress]);

  const handleShuffleClick = () => {
    fetch(`${ipAddress}/shuffle_deck`, { method: 'POST' }) // Assuming POST for shuffle
      .then(response => {
        if (response.ok) {
          return fetchGameState();
        } else {
          throw new Error('Shuffle failed');
        }
      })
      .then(response => response.json())
      .then(data => {
        setGameState(data);
      })
      .catch(error => console.error('Error:', error));
  };

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
        <button onClick={fetchGameState}>Fetch Data</button>
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
        {gameState ? (
          <div>
            {/* ... rest of your component ... */}
            <PlayerList players={gameState.players} />
            <hr />
            <CardList cards={gameState.deck.cards} shuffleClickHandler={handleShuffleClick} />
          </div>
        ) : (
          <p>Loading game state...</p>
        )}
      </header>
    </div>
  );
}

export default App;



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
