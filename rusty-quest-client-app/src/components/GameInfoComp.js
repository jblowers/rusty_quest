
import React, { useState, useEffect } from 'react';
import {GameStateProvider, useGameState} from '../contexts/GameStateContext';

const GameInfoComp = ({}) => {
    // if ( !gameState) {
        // return <p>No Game State Found</p>;
    // }
    const playerCount = -1;
    const activePlayer = [];
    const gameState = useGameState();
    useEffect(() => {        
        if (gameState && gameState.players) {
            playerCount = Object.keys(gameState.players).length;
            activePlayer = gameState.players[gameState.active_player_id];
        } else {
            console.error(`gameState.players not present...`);
        }
    },[gameState]);
    if (!gameState) {
        return (
            <p>Error with game info</p>
        );
    }
    
    // useEffect(() => {
    //     fetchGameList();
    // }, [gameState]);

    return (
        <div>
            <h3>Game Information</h3>  
        <div className='game-info-container'>
            <div className="text-container">
                <p>Game ID: {gameState.id}</p>
                <p>Player Count: {playerCount}</p>
                <p>Active Player: {activePlayer ? activePlayer.name : 'N/A'}</p>
                <p>Turn: {gameState.turn}</p>
            </div>
        </div>
        </div>
    );
}



export default GameInfoComp;