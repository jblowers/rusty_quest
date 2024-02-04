import React, { useEffect, useState, useCallback, useContext  } from 'react';
import DebuggingTool from './components/DebuggingTool'; // Adjust the import path if necessary
import GameBoard from './components/GameBoard';
import IpAddressConfiguration from './components/IpConfigComponent';
import GameManagementComp from './components/GameManagementComp';
import GameInfoComp from './components/GameInfoComp';
import CardCollectionComponent from './components/CardCollectionComponent';
import ActionsListComp from './components/ActionsListComp';
import {GameStateProvider, useGameState} from './contexts/GameStateContext';
import { IpAddressProvider, useIpAddress } from './contexts/IpAddressContext';
import * as ApiService from './services/apiService';

import './App.css';


function App() {
  const [actionInfoList, setActionInfoList] = useState({});
  const [selectedGameId, setSelectedGameId] = useState(null);
  const {gameState, setGameState } = useGameState();
  const {ipAddress, setIpAddress} = useIpAddress();


  // // caches and updates action list info
  useEffect(() => {
    ApiService.fetchActionInfoList(ipAddress)
      .then(data => setActionInfoList(data))
      .catch(error => console.error('Error fetching actions description:', error));
  }, [ipAddress]);
  useEffect((selectedGameId,setGameState) => {
    if (selectedGameId !== 'none') {
      ApiService.fetchGameState(ipAddress,selectedGameId)
        .then(data => setGameState(data))
        .catch(error => console.error('Error fetching Game State...:', error));
    }
  }, [ipAddress]);


  const fetchGameState = async (gameId) => {
    if (!gameId) {
      console.log("No game selected");
      return;
    }
    try {
      console.log(`Fetching gamestate from ${ipAddress} / ${gameId}`);
      const data = await ApiService.fetchGameState(ipAddress, gameId,setGameState);
      console.log(`Game state data: ${data}`);
      // setGameState(data);
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
          <GameInfoComp/>
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
          <ActionsListComp actionsListInfo={actionInfoList}/>
        </div>
      </div> 
    </div>
  );
}

const AppWithProviders = () => (
  <IpAddressProvider>
    <GameStateProvider>
      <App />
    </GameStateProvider>
  </IpAddressProvider>
);

export default AppWithProviders;

// export default App;
