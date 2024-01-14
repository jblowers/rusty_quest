import React, { useEffect, useState, useCallback  } from 'react';
import DebuggingTool from './components/DebuggingTool'; // Adjust the import path if necessary
import GameBoard from './components/GameBoard';
import IpAddressConfiguration from './components/IpConfigComponent';
import GameManagementComp from './components/GameManagementComp';
import GameInfoComp from './components/GameInfoComp';
import CardCollectionComponent from './components/CardCollectionComponent';

import './App.css';

const DEFAULT_URL = 'http://192.168.0.134:3030';
// const BASE_URL = 'http://localhost:3030';



function App() {
  const [gameState, setGameState] = useState(null);  
  const [selectedGameId, setSelectedGameId] = useState(null);
  const [ipAddress, setIpAddress] = useState(() => {
    return localStorage.getItem('ipAddress') || DEFAULT_URL;
  });

  const fetchGameState = async (gameId) => {
    if (!gameId) {
        console.log("No game selected");
        return;
    }
    try {
        console.error(`${ipAddress}/game_state/${gameId}`);
        const response = await fetch(`${ipAddress}/game_state/${gameId}`);
        if (!response.ok) {
            throw new Error('Network response was not ok');
        }
        const data = await response.json();
        setGameState(data);
    } catch (error) {
        console.error('Error fetching game state:', error);
    }
};

  const applyIpAddress = (event) => {
    setIpAddress(event.target.value);
  };

  const onSelectGameChange = (gameid) => {
    setSelectedGameId(gameid);
    fetchGameState(gameid);
  }

  return (
    <div>
      <div className="App">
        <div className="container">
          <div className="column">
            {/* <IpAddressConfiguration /> */}
            
            <IpAddressConfiguration 
                  ipAddress={ipAddress} 
                  applyIpAddress={applyIpAddress} 
              />
              <GameManagementComp ipAddress={ipAddress} onSelectGame={onSelectGameChange} refreshGameState={fetchGameState}/>
          </div>
          <div className="column">
            <GameInfoComp gameState={gameState}/>
            <hr></hr>
            {gameState ?
            <div>
              <CardCollectionComponent cards={gameState.deck.cards} title={"Deck"}/>              
              <CardCollectionComponent cards={gameState.discard.cards} title={"Discard"}/>
            </div>
              :
              <p>Empty Deck</p>
            }
          </div>
          <div className="column">
            <DebuggingTool ipAddress={ipAddress}/>
          </div>
        </div> 
      </div>
    </div>
  );
}

export default App;


  // START added 12/30

  // const handleNewGame = async () => {
  //   setIsLoading(true);
  //   try {
  //     const response = await fetch(`${ipAddress}/game_state/new_game`, {
  //       method: 'POST',
  //       headers: {
  //         'Content-Type': 'application/json',
  //       },
  //     });
  //     const data = await response.json();
  //     setGameState(data);
  //     setGameId(data.id);
  //   } catch (error) {
  //     console.error('Error fetching new game:', error);
  //   } finally {
  //     setIsLoading(false);
  //   }
  // };

  
  // // const [gameState, setGameState] = useState({});
  // const [gameId, setGameId] = useState(null);
  // const [isLoading, setIsLoading] = useState(false);
  
  // const refreshGameState = async () => {
  //   if (!gameId) return;

  //   setIsLoading(true);
  //   try {
  //     const response = await fetch(`${ipAddress}/game_state/${gameId}`);
  //     const data = await response.json();
  //     setGameState(data);
  //   } catch (error) {
  //     console.error('Error fetching game state:', error);
  //   } finally {
  //     setIsLoading(false);
  //   }
  // };
  // const handleShuffleClick = () => {
  //     fetch(`${ipAddress}/game_state/${gameId}/shuffle_deck`, { method: 'POST' }) // Assuming POST for shuffle
  //       .then(response => {
  //         // if (response.ok) {
  //         //   return fetchGameState();
  //         // } else {
  //         //   throw new Error('Shuffle failed');
  //         // }
  //       })
  //       .then(response => response.json())
  //       .then(data => {
  //         setGameState(data);
  //       })
  //       .catch(error => console.error('Error:', error));
  //   };
  
  // // END added 12/30


  // let validIpAddress = typeof ipAddress === 'string' ? ipAddress : DEFAULT_URL;
  // console.log("IP Address:", validIpAddress); // Check what this logs

// function CreateGame({gameState,gameId,isLoading,handleNewGame,refreshGameState, handleShuffleClick}) {
//   return (
//     <div className="App">
//       <button onClick={handleNewGame} disabled={isLoading}>
//         {isLoading ? 'Loading...' : 'Start New Game'}
//       </button>
//     { gameState?
//     <div>
//       <button onClick={refreshGameState} disabled={isLoading || !gameId}>
//         Refresh Game State
//       </button>
//       <div>
//         <h2>Game State:</h2>
//         <textarea
//           value={JSON.stringify(gameState, null, 2)}
//           readOnly
//           style={{ width: '100%', height: '300px' }}
//         />
//       </div>
//       <div>
//         <CardList cards={gameState.deck.cards} shuffleClickHandler={handleShuffleClick} />
//         { gameState.players && gameState.players.length > 0 ?
//           <GameBoard spaces={gameState.board.spaces} activePlayerPosition={gameState.players[gameState.active_player_id].position} />
//           :
//           <h4>No active players.</h4>
//         }
//       </div>
//     </div>
//     :
//     <h1>Loading...</h1>
//     }
//     </div>
//   );
// }


// function CardList({ cards, shuffleClickHandler }) {
//   const cardListStyle = {
//     maxHeight: '200px', // Set a fixed height
//     overflowY: 'auto', // Enable vertical scrolling
//     border: '1px solid #ccc', // Optional: adds a border for better visibility
//     padding: '10px', // Optional: adds some padding inside the scrollable area
//     margin: '10px 0' // Optional: adds some margin around the scrollable area
//   };

//   return (
//     <div>
//       <h2>Cards</h2>
//       <div style={cardListStyle}>
//         {Array.isArray(cards) && cards.map((card, index) => (
//           <div key={index}>
//             {card.value}, {card.typ}
//           </div>
//         ))}
//       </div>
//       <button onClick={shuffleClickHandler}>Shuffle Deck</button>
//     </div>
//   );
// }

// // function CardList({ cards }) {
// //   return (
// //     <div>
// //       <h2>Cards</h2>
// //       {cards.map((card, index) => (
// //         <div key={index}>
// //           Type: {card.typ}, Value: {card.value}
// //         </div>
// //       ))}
// //     </div>
// //   );
// // }


// function PlayerList({ players }) {
//   return (
//     <div>
//       <h2>Players</h2>
//       {/* Render player list */}
//     </div>
//   );
// }



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

