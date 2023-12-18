import React, { useEffect, useState } from 'react';
import './App.css';

function App() {
  const [gameState, setGameState] = useState(null);

  useEffect(() => {
    fetch('http://localhost:3030/game_state')
    .then(response => response.json())
    .then(data => {
      setGameState(data);
    })
    .catch(error => console.error('Error fetching data:', error));
  }, []);

  const handleShuffleClick = () => {
    fetch('http://localhost:3030/shuffle_deck', { method: 'POST' }) // Assuming POST for shuffle
      .then(response => {
        if (response.ok) {
          return fetch('http://localhost:3030/game_state');
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

  return (
    <div className="App">
      <header className="App-header">
        {gameState ? (
          <div>
            <button onClick={handleShuffleClick}>Shuffle Deck</button>
            {/* ... rest of your component ... */}
            <PlayerList players={gameState.players} />
            <CardList cards={gameState.deck.cards} />
          </div>
        ) : (
          <p>Loading game state...</p>
        )}
      </header>
    </div>
  );
}

export default App;




function CardList({ cards }) {
  return (
    <div>
      <h2>Cards</h2>
      {cards.map((card, index) => (
        <div key={index}>
          Type: {card.typ}, Value: {card.value}
        </div>
      ))}
    </div>
  );
}


function PlayerList({ players }) {
  return (
    <div>
      <h2>Players</h2>
      {/* Render player list */}
    </div>
  );
}
