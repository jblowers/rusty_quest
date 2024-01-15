
import React, { useState, useEffect } from 'react';

const GameManagementComp = ({ipAddress, onSelectGame, refreshGameState}) => {

    const [gameList, setGameList] = useState([]);
    const [selectedGame, setSelectedGame] = useState('none');


    const fetchGameList = () => {
        fetch(`${ipAddress}/game_list`)
            .then(response => response.json())
            .then(data => setGameList(data))
            .catch(error => console.error('Error fetching game list:', error));
    };

    // Use useEffect to fetch game list on component mount and when ipAddress changes
    useEffect(() => {
        fetchGameList();
    }, [ipAddress]);

    
    const handleNewGame = () => {
        fetch(`${ipAddress}/new_game`, { method: 'POST' })
            .then(response => {
                if (response.ok) {
                    // Fetch the updated game list after a new game is created
                    fetchGameList();
                } else {
                    console.error('Failed to start new game');
                }
            })
            .catch(error => console.error('Error starting new game:', error));
    };
    
    const handleSelectGame = () => {
        if (selectedGame !== "none") {
            onSelectGame(selectedGame);
        }
    };

    return (
        <div>
            <button onClick={handleNewGame}>Start New Game</button>
            <select value={selectedGame} onChange={e => setSelectedGame(e.target.value)}>
                <option key="none" value="">None</option>
                {gameList.map(game => (
                    <option key={game} value={game}>{game}</option>
                ))}
            </select>
            <button onClick={handleSelectGame}>Select Game</button>
            <div>
                <button onClick={refreshGameState} value={selectedGame}>Refresh Game State</button> 
                {/* maybe dont' need the refresh button anymore */}
            </div>
        </div>
    );
}


export default GameManagementComp;