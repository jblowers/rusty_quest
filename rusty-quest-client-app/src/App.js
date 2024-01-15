import React, { useEffect, useState, useCallback  } from 'react';
import DebuggingTool from './components/DebuggingTool'; // Adjust the import path if necessary
import GameBoard from './components/GameBoard';
import IpAddressConfiguration from './components/IpConfigComponent';
import GameManagementComp from './components/GameManagementComp';
import GameInfoComp from './components/GameInfoComp';
import CardCollectionComponent from './components/CardCollectionComponent';
import ActionsListComp from './components/ActionsListComp';

import './App.css';

const DEFAULT_URL = 'http://192.168.0.134:3030';
// const BASE_URL = 'http://localhost:3030';



function App() {
  const [actionInfoList, setActionInfoList] = useState({});
  const [gameState, setGameState] = useState(null);  
  const [selectedGameId, setSelectedGameId] = useState(null);
  const [ipAddress, setIpAddress] = useState(() => {
    return localStorage.getItem('ipAddress') || DEFAULT_URL;
  });


  // caches and updates action list info
  useEffect(() => {
    console.log("fetching from: ");
    console.log(`${ipAddress}/actionlistinfo`);
    fetch(`${ipAddress}/actionlistinfo`)
      .then(response => response.json())
      .then(data => setActionInfoList(data))
      .catch(error => console.error('Error fetching actions description:', error));
  }, [ipAddress]);


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
    console.log(`Game change hit. ${gameid}\tactionlist: ${actionInfoList}`);
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
            <DebuggingTool ipAddress={ipAddress} selectedGameId={selectedGameId}/>
            <hr></hr>
            <hr></hr>
            <ActionsListComp gameState={gameState} actionsListInfo={actionInfoList}/>
          </div>
        </div> 
      </div>
    </div>
  );
}

export default App;
