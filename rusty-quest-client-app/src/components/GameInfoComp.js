
import React, { useState, useEffect } from 'react';

const GameInfoComp = ({gameState}) => {
    if ( !gameState) {
        return <p>No Game State Found</p>;
    }
    
    const playerCount = Object.keys(gameState.players).length;
    const activePlayer = gameState.players[gameState.active_player_id];
    
    // useEffect(() => {
    //     fetchGameList();
    // }, [gameState]);

    return (
        <div>
            <h3>Game Information</h3>  
            <p>Game ID: {gameState.id}</p>
            <p>Player Count: {playerCount}</p>
            <p>Active Player: {activePlayer ? activePlayer.name : 'N/A'}</p>
            <p>Turn: {gameState.turn}</p>
        </div>
    );
}



export default GameInfoComp;