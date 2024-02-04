import React, { createContext, useState, useContext } from 'react';

export const GameStateContext = createContext();

export const GameStateProvider = ({ children }) => {
    const [gameState, setGameState] = useState(null); // Initial gameState

    return (
        <GameStateContext.Provider value={{ gameState, setGameState }}>
            {children}
        </GameStateContext.Provider>
    );
};

export const useGameState = () => useContext(GameStateContext);
