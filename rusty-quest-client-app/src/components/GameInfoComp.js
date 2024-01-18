
import React, { useState, useEffect } from 'react';
import {GameStateProvider, useGameState} from '../contexts/GameStateContext';

const GameInfoComp = ({}) => {
    
    const {gameState, setGameState } = useGameState();
    useEffect(() => {        
        if (!gameState || !gameState.players ) {
            console.error(`gameState not present...`);
        }
        if (gameState) {
            const playerCount = gameState.players.size;
            const activePlayer = gameState.players[gameState.active_player_id];

        }
    },[gameState]);
    if (!gameState || !gameState.board) {
        console.warn(`game state is missing.`);
        return (
            <p>Error with gameState</p>
        );
    }

    return (
        <div>
            <h3>Game Information</h3>  
        <div className='game-info-container'>
            <div className="text-container">
                <p>Game ID: {gameState.id}</p>
                {gameState.players ? (
                    <div>
                        <p>Player Count: {Object.keys(gameState.players).length}</p>
                        <hr/>
                        <h3>Player list:</h3>
                        {Object.keys(gameState.players).map(playerId => {
                            const isPlayerActive = parseInt(playerId, 10) === gameState.active_player_id;
                            return (
                                <p key={playerId} style={isPlayerActive ? { fontWeight: 'bold' } : {}}>
                                    {gameState.players[playerId].name}{isPlayerActive ? " (Active)" : ""}
                                </p>
                            );
                        })}
                    </div>
                ) : (
                    <p>No Player Info found.</p>
                )}
                <hr/>
                <p>Turn: {gameState.turn}</p>
            </div>
        </div>
        </div>
    );
}



export default GameInfoComp;